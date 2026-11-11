// Telosgard's Worker (ADR 0003). Static files are served by Workers static assets; this code runs
// only for /api/*. Every route needs an identity (see auth.ts) and only ever touches that user's rows.
//
//   GET    /api/me                 → { signedIn, userId?, email? }
//   GET    /api/docs?kind=K        → [{ id, data }]          (all of the user's docs of one kind)
//   GET    /api/docs/:id           → data | null
//   PUT    /api/docs/:id           ← JSON with a string `kind`
//   DELETE /api/docs/:id
//   POST   /api/files              ← an image body (≤ 8 MB)  → { id, url }
//   GET    /api/files/:id          → the image, to its owner only
//   POST   /api/claude/(json|text) ← { prompt, tier } → { value } | { text, truncated }

import Anthropic from "@anthropic-ai/sdk";
import { identify, type AuthEnv, type Identity } from "./auth";
import { askClaude, ClaudeError, parseTier, type MessagesClient, type Mode } from "./claude";

export interface Env extends AuthEnv {
  DB: D1Database;
  FILES: R2Bucket;
  ASSETS: Fetcher;
  /** Worker secret (`wrangler secret put ANTHROPIC_API_KEY`); `.dev.vars` locally. */
  ANTHROPIC_API_KEY?: string;
  /** Claude calls per user per 24 hours (default 300). */
  CLAUDE_DAILY_LIMIT?: string;
}

/** Builds the Anthropic client; tests swap it for a fake. */
export type ClientFactory = (env: Env) => MessagesClient;
const realClient: ClientFactory = (env) => new Anthropic({ apiKey: env.ANTHROPIC_API_KEY, maxRetries: 1 });

const DOC_ID = /^[A-Za-z0-9._-]{1,128}$/;
const KIND = /^[a-z][a-z0-9_-]{0,31}$/;
const MAX_DOC_BYTES = 256 * 1024;
const MAX_FILE_BYTES = 8 * 1024 * 1024;

const json = (body: unknown, status = 200) =>
  new Response(JSON.stringify(body), { status, headers: { "content-type": "application/json; charset=utf-8", "cache-control": "no-store" } });

/** Errors carry a `code` the app's bridge understands (it maps them to PlatformError). */
const fail = (status: number, code: string, message: string) => json({ code, message }, status);

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    return handle(request, env, realClient);
  },
} satisfies ExportedHandler<Env>;

export async function handle(request: Request, env: Env, makeClient: ClientFactory): Promise<Response> {
    const url = new URL(request.url);
    if (!url.pathname.startsWith("/api/")) return env.ASSETS.fetch(request);
    try {
      return await route(request, env, url, makeClient);
    } catch (e) {
      console.error("api error", e);
      return fail(500, "unavailable", "Something went wrong on the server.");
    }
}

async function route(request: Request, env: Env, url: URL, makeClient: ClientFactory): Promise<Response> {
  const who = await identify(request, env);
  const path = url.pathname.slice("/api/".length).split("/").filter(Boolean);
  const method = request.method;

  if (path[0] === "me" && path.length === 1 && method === "GET") {
    return json(who ? { signedIn: true, userId: who.userId, email: who.email ?? null } : { signedIn: false });
  }
  if (!who) return fail(401, "not_granted", "Sign in to use Telosgard.");

  if (path[0] === "docs") return docs(request, env, who, path.slice(1), url);
  if (path[0] === "files") return files(request, env, who, path.slice(1));
  if (path[0] === "claude" && path.length === 2 && (path[1] === "json" || path[1] === "text") && method === "POST") {
    return claude(request, env, who, path[1], makeClient);
  }
  return fail(404, "invalid_argument", "No such route.");
}

async function docs(request: Request, env: Env, who: Identity, rest: string[], url: URL): Promise<Response> {
  const method = request.method;
  if (rest.length === 0) {
    if (method !== "GET") return fail(405, "invalid_argument", "Use GET to list documents.");
    const kind = url.searchParams.get("kind") ?? "";
    if (!KIND.test(kind)) return fail(400, "invalid_argument", "A valid ?kind= is required.");
    const { results } = await env.DB.prepare("SELECT doc_id, body FROM docs WHERE user_id = ?1 AND kind = ?2 ORDER BY doc_id")
      .bind(who.userId, kind)
      .all<{ doc_id: string; body: string }>();
    return json(results.map((r) => ({ id: r.doc_id, data: JSON.parse(r.body) })));
  }
  const id = rest[0];
  if (rest.length !== 1 || !DOC_ID.test(id)) return fail(400, "invalid_argument", "Bad document id.");

  if (method === "GET") {
    const row = await env.DB.prepare("SELECT body FROM docs WHERE user_id = ?1 AND doc_id = ?2").bind(who.userId, id).first<{ body: string }>();
    return json(row ? JSON.parse(row.body) : null);
  }
  if (method === "PUT") {
    const text = await request.text();
    if (text.length > MAX_DOC_BYTES) return fail(413, "quota_exceeded", "That document is too large.");
    let body: unknown;
    try {
      body = JSON.parse(text);
    } catch {
      return fail(400, "invalid_argument", "The body must be JSON.");
    }
    const kind = (body as { kind?: unknown } | null)?.kind;
    if (typeof body !== "object" || body === null || Array.isArray(body) || typeof kind !== "string" || !KIND.test(kind)) {
      return fail(400, "invalid_argument", "A document is a JSON object with a string kind.");
    }
    await env.DB.prepare(
      "INSERT INTO docs (user_id, doc_id, kind, body, updated_at) VALUES (?1, ?2, ?3, ?4, ?5) " +
        "ON CONFLICT (user_id, doc_id) DO UPDATE SET kind = excluded.kind, body = excluded.body, updated_at = excluded.updated_at",
    )
      .bind(who.userId, id, kind, JSON.stringify(body), Date.now())
      .run();
    return json(null);
  }
  if (method === "DELETE") {
    await env.DB.prepare("DELETE FROM docs WHERE user_id = ?1 AND doc_id = ?2").bind(who.userId, id).run();
    return json(null);
  }
  return fail(405, "invalid_argument", "Unsupported method.");
}

async function files(request: Request, env: Env, who: Identity, rest: string[]): Promise<Response> {
  if (rest.length === 0 && request.method === "POST") {
    const type = request.headers.get("content-type") ?? "";
    if (!/^image\/(png|jpeg|webp|gif)$/.test(type)) return fail(400, "invalid_argument", "Please choose a PNG, JPEG, WebP or GIF image.");
    const declared = Number(request.headers.get("content-length") ?? "0");
    if (declared > MAX_FILE_BYTES) return fail(413, "invalid_argument", "Images up to 8 MB.");
    const bytes = await request.arrayBuffer();
    if (bytes.byteLength === 0) return fail(400, "invalid_argument", "The image is empty.");
    if (bytes.byteLength > MAX_FILE_BYTES) return fail(413, "invalid_argument", "Images up to 8 MB.");
    const id = crypto.randomUUID().replace(/-/g, "");
    await env.FILES.put(`${who.userId}/${id}`, bytes, { httpMetadata: { contentType: type } });
    await env.DB.prepare("INSERT INTO files (id, user_id, content_type, size, created_at) VALUES (?1, ?2, ?3, ?4, ?5)")
      .bind(id, who.userId, type, bytes.byteLength, Date.now())
      .run();
    return json({ id, url: `/api/files/${id}` });
  }
  if (rest.length === 1 && request.method === "GET" && /^[a-f0-9]{32}$/.test(rest[0])) {
    const owned = await env.DB.prepare("SELECT content_type FROM files WHERE id = ?1 AND user_id = ?2").bind(rest[0], who.userId).first<{ content_type: string }>();
    if (!owned) return fail(404, "invalid_argument", "No such image.");
    const obj = await env.FILES.get(`${who.userId}/${rest[0]}`);
    if (!obj) return fail(404, "invalid_argument", "No such image.");
    return new Response(obj.body, { headers: { "content-type": owned.content_type, "cache-control": "private, max-age=3600" } });
  }
  return fail(404, "invalid_argument", "No such route.");
}

async function claude(request: Request, env: Env, who: Identity, mode: Mode, makeClient: ClientFactory): Promise<Response> {
  if (!env.ANTHROPIC_API_KEY) return fail(503, "not_granted", "Claude isn't set up on the server yet.");
  let body: { prompt?: unknown; tier?: unknown };
  try {
    body = await request.json();
  } catch {
    return fail(400, "invalid_argument", "The body must be JSON.");
  }
  if (typeof body.prompt !== "string") return fail(400, "invalid_argument", "A prompt is required.");

  const limit = Number(env.CLAUDE_DAILY_LIMIT ?? "300") || 300;
  const since = Date.now() - 24 * 3600 * 1000;
  const used = await env.DB.prepare("SELECT COUNT(*) AS n FROM claude_calls WHERE user_id = ?1 AND at > ?2").bind(who.userId, since).first<{ n: number }>();
  if ((used?.n ?? 0) >= limit) return fail(429, "rate_limited", `You've reached today's limit of ${limit} Claude requests. It resets over the next 24 hours.`);

  const log = (ok: boolean, input = 0, output = 0) =>
    env.DB.prepare("INSERT INTO claude_calls (user_id, at, mode, ok, input_tokens, output_tokens) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")
      .bind(who.userId, Date.now(), mode, ok ? 1 : 0, input, output)
      .run();
  try {
    const r = await askClaude(makeClient(env), mode, body.prompt, parseTier(body.tier));
    await log(true, r.usage.input, r.usage.output);
    return json(mode === "json" ? { value: r.value } : { text: r.text, truncated: !!r.truncated });
  } catch (e) {
    await log(false);
    if (e instanceof ClaudeError) return fail(e.status, e.code, e.message);
    throw e;
  }
}
