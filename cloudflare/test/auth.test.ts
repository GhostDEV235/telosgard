import { env } from "cloudflare:test";
import { describe, expect, it } from "vitest";
import { handle, type Env } from "../src/index";
import { INVITE_HEADER, sha256 } from "../src/auth";

const origin = "http://localhost";
// Real Better Auth sessions: no DEV_USER stand-in here.
const e = () => ({ ...env, DEV_USER: undefined }) as unknown as Env;
const noClaude = () => {
  throw new Error("no Claude in auth tests");
};

let ip = 0;
async function call(method: string, path: string, body?: unknown, opts: { cookie?: string; invite?: string; sameIp?: boolean } = {}) {
  // A fresh client address per request, so Better Auth's per-IP rate limit doesn't trip across tests.
  if (!opts.sameIp) ip++;
  const headers: Record<string, string> = { origin, "content-type": "application/json", "cf-connecting-ip": `10.0.${Math.floor(ip / 250)}.${ip % 250}` };
  if (opts.cookie) headers.cookie = opts.cookie;
  if (opts.invite) headers[INVITE_HEADER] = opts.invite;
  const res = await handle(new Request(origin + path, { method, headers, body: body === undefined ? undefined : JSON.stringify(body) }), e(), noClaude);
  const cookie = (res.headers.getSetCookie?.() ?? []).map((c) => c.split(";")[0]).join("; ");
  let data: unknown = null;
  try {
    data = await res.json();
  } catch {}
  return { status: res.status, data: data as Record<string, unknown>, cookie };
}

const signUp = (email: string, invite?: string, password = "correct horse battery") =>
  call("POST", "/api/auth/sign-up/email", { name: email.split("@")[0], email, password }, { invite });

describe("accounts (Better Auth, invite-only)", () => {
  it("refuses sign-up without an invite", async () => {
    const r = await signUp("stranger@example.com");
    expect(r.status).toBe(403);
    expect((await signUp("stranger2@example.com", "made-up-token-made-up-token")).status).toBe(403);
  });

  it("lets the owner sign up without an invite, and marks them owner", async () => {
    const r = await signUp("owner@example.com");
    expect(r.status).toBe(200);
    expect(r.cookie).toContain("session_token");
    const me = await call("GET", "/api/me", undefined, { cookie: r.cookie });
    expect(me.data).toMatchObject({ signedIn: true, email: "owner@example.com", isOwner: true });
  });

  it("invites are single-use, owner-only and keep users apart", async () => {
    const owner = (await call("POST", "/api/auth/sign-in/email", { email: "owner@example.com", password: "correct horse battery" })).cookie;
    expect(owner).toContain("session_token");
    const inv = await call("POST", "/api/invites", undefined, { cookie: owner });
    expect(inv.status).toBe(200);
    const token = String(inv.data.url).split("#join-")[1];
    expect(token.length).toBeGreaterThan(20);

    const tester = await signUp("tester@example.com", token);
    expect(tester.status).toBe(200);
    expect((await signUp("second@example.com", token)).status).toBe(403);

    const me = await call("GET", "/api/me", undefined, { cookie: tester.cookie });
    expect(me.data).toMatchObject({ signedIn: true, email: "tester@example.com", isOwner: false, fullAccess: false });
    expect((await call("POST", "/api/invites", undefined, { cookie: tester.cookie })).status).toBe(403);

    // A reviewer address gets every lesson open, but still needs an invite and gets no owner powers.
    expect((await signUp("reviewer@example.com")).status).toBe(403);
    const inv2 = await call("POST", "/api/invites", undefined, { cookie: owner });
    const reviewer = await signUp("Reviewer@Example.com", String(inv2.data.url).split("#join-")[1]);
    expect(reviewer.status).toBe(200);
    const rme = await call("GET", "/api/me", undefined, { cookie: reviewer.cookie });
    expect(rme.data).toMatchObject({ isOwner: false, fullAccess: true });
    expect((await call("POST", "/api/invites", undefined, { cookie: reviewer.cookie })).status).toBe(403);

    await call("PUT", "/api/docs/plan-v1", { kind: "plan", version: 1 }, { cookie: owner });
    expect((await call("GET", "/api/docs/plan-v1", undefined, { cookie: tester.cookie })).data).toBeNull();
    expect((await call("GET", "/api/docs/plan-v1", undefined, { cookie: owner })).data).toEqual({ kind: "plan", version: 1 });

    const list = await call("GET", "/api/invites", undefined, { cookie: owner });
    expect((list.data as unknown as { usedAt: number | null }[])[0].usedAt).not.toBeNull();
  });

  it("refuses an expired invite", async () => {
    const token = "expired-token-expired-token-123";
    await (env as unknown as Env).DB.prepare("INSERT INTO invites (id, token_hash, created_by, created_at, expires_at) VALUES ('old', ?1, 'x', 0, 1)").bind(await sha256(token)).run();
    expect((await signUp("late@example.com", token)).status).toBe(403);
  });

  it("checks passwords and signs out", async () => {
    expect((await call("POST", "/api/auth/sign-in/email", { email: "owner@example.com", password: "wrong password!" })).status).toBe(401);
    const ok = await call("POST", "/api/auth/sign-in/email", { email: "owner@example.com", password: "correct horse battery" });
    expect(ok.status).toBe(200);
    await call("POST", "/api/auth/sign-out", {}, { cookie: ok.cookie });
    expect((await call("GET", "/api/me", undefined, { cookie: ok.cookie })).data).toMatchObject({ signedIn: false });
  });

  it("rejects short passwords without spending the invite", async () => {
    const owner = (await call("POST", "/api/auth/sign-in/email", { email: "owner@example.com", password: "correct horse battery" })).cookie;
    const token = String((await call("POST", "/api/invites", undefined, { cookie: owner })).data.url).split("#join-")[1];
    expect((await signUp("short@example.com", token, "short")).status).toBe(400);
    expect((await signUp("short@example.com", token)).status).toBe(200);
  });

  it("rate-limits repeated sign-in attempts from one address", async () => {
    const statuses: number[] = [];
    for (let i = 0; i < 6; i++) {
      statuses.push((await call("POST", "/api/auth/sign-in/email", { email: "owner@example.com", password: "wrong password!" }, { sameIp: true })).status);
    }
    expect(statuses).toContain(429);
  });
});
