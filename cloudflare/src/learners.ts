// Learners (D1): the list and profile the dashboard shows, and the actions staff take on an account.
// Every action returns the values before and after, for the audit log and its Undo.

import { makeAuth, ownerEmails, sha256, type AuthEnv } from "./auth";
import { totpAt } from "./auth-util";
import { symmetricDecrypt } from "better-auth/crypto";

const DAY = 24 * 3600 * 1000;
export const DELETE_GRACE_MS = 7 * DAY;
export const RESET_LINK_MS = DAY;

export interface Account {
  id: string;
  name: string;
  email: string;
  role: string;
  createdAt: number;
  suspendedAt: number | null;
  fullAccess: boolean;
  deleteAfter: number | null;
  joinedVia: string | null;
  isOwner: boolean;
}

const asMs = (v: unknown): number => (typeof v === "number" ? v : Date.parse(String(v)));

function account(r: Record<string, unknown>, owners: string[]): Account {
  return {
    id: String(r.id),
    name: String(r.name ?? ""),
    email: String(r.email),
    role: String(r.role ?? "learner"),
    createdAt: asMs(r.createdAt),
    suspendedAt: r.suspendedAt == null ? null : Number(r.suspendedAt),
    fullAccess: Boolean(r.fullAccess),
    deleteAfter: r.deleteAfter == null ? null : Number(r.deleteAfter),
    joinedVia: (r.joinedVia as string | null) ?? null,
    isOwner: owners.includes(String(r.email).toLowerCase()),
  };
}

const COLUMNS = 'id, name, email, role, "createdAt", "suspendedAt", "fullAccess", "deleteAfter", "joinedVia"';

export async function getAccount(env: AuthEnv, id: string): Promise<Account | null> {
  const r = await env.DB.prepare(`SELECT ${COLUMNS} FROM "user" WHERE id = ?1`).bind(id).first<Record<string, unknown>>();
  return r ? account(r, ownerEmails(env)) : null;
}

/** Joined label for the list: "Invite link", "Code GOLD-NOV", "Owner". */
async function joinedLabels(db: D1Database): Promise<Map<string, string>> {
  const { results } = await db.prepare("SELECT id, code FROM invite_codes").all<{ id: string; code: string }>();
  return new Map(results.map((r) => [`code:${r.id}`, `Code ${r.code}`]));
}

function joinedLabel(via: string | null, codes: Map<string, string>): string {
  if (!via) return "Before tracking";
  if (via === "owner") return "Owner";
  if (via.startsWith("link:")) return "Invite link";
  return codes.get(via) ?? "Invite code";
}

export interface LearnerRow extends Account {
  joined: string;
  lastActive: number | null;
  lessonsPassed: number;
}

/** Every account except the Owner's, newest first, with last activity and lessons passed. */
export async function listLearners(env: AuthEnv): Promise<LearnerRow[]> {
  const owners = ownerEmails(env);
  const { results } = await env.DB.prepare(`SELECT ${COLUMNS} FROM "user" ORDER BY "createdAt" DESC`).all<Record<string, unknown>>();
  const accounts = results.map((r) => account(r, owners)).filter((a) => !a.isOwner);
  const active = await env.DB.prepare("SELECT user_id, MAX(at) AS last FROM events WHERE type != 'signed_up' GROUP BY user_id").all<{ user_id: string; last: number }>();
  const lastBy = new Map(active.results.map((r) => [r.user_id, Number(r.last)]));
  // Lessons passed come from the stored progress documents (the source of truth for the Gate).
  // Parsed here rather than with SQLite's JSON functions, so the query stays portable (ADR 0006).
  const progress = await env.DB.prepare("SELECT user_id, body FROM docs WHERE kind = 'progress'").all<{ user_id: string; body: string }>();
  const passedBy = new Map<string, number>();
  for (const r of progress.results) {
    try {
      if (JSON.parse(r.body)?.passedAt != null) passedBy.set(r.user_id, (passedBy.get(r.user_id) ?? 0) + 1);
    } catch {}
  }
  const codes = await joinedLabels(env.DB);
  return accounts.map((a) => ({ ...a, joined: joinedLabel(a.joinedVia, codes), lastActive: lastBy.get(a.id) ?? null, lessonsPassed: passedBy.get(a.id) ?? 0 }));
}

/** One Learner: account, progress per lesson, Explain-Back history, drills and counts. Never journal contents. */
export async function learnerProfile(env: AuthEnv, id: string) {
  const a = await getAccount(env, id);
  if (!a || a.isOwner) return null;
  const { results } = await env.DB.prepare("SELECT doc_id, kind, body, updated_at FROM docs WHERE user_id = ?1 AND kind IN ('progress', 'attempt', 'drill', 'trade', 'checklist')")
    .bind(id)
    .all<{ doc_id: string; kind: string; body: string; updated_at: number }>();
  const parse = (b: string) => {
    try {
      return JSON.parse(b) as Record<string, unknown>;
    } catch {
      return {};
    }
  };
  const progress = results
    .filter((r) => r.kind === "progress")
    .map((r) => ({ lesson: r.doc_id, ...pick(parse(r.body), ["quizBest", "grade", "passedAt"]) }))
    .sort((x, y) => x.lesson.localeCompare(y.lesson));
  const attempts = results
    .filter((r) => r.kind === "attempt")
    .map((r) => pick(parse(r.body), ["lesson", "grade", "answer", "at"]))
    .sort((x, y) => Number(y.at ?? 0) - Number(x.at ?? 0))
    .slice(0, 50);
  const drills = results.filter((r) => r.kind === "drill").map((r) => ({ drill: r.doc_id.replace(/^drill-/, ""), ...pick(parse(r.body), ["answered", "correct", "lastAt"]) }));
  const last = await env.DB.prepare("SELECT MAX(at) AS last FROM events WHERE user_id = ?1 AND type != 'signed_up'").bind(id).first<{ last: number | null }>();
  const codes = await joinedLabels(env.DB);
  return {
    ...a,
    joined: joinedLabel(a.joinedVia, codes),
    lastActive: last?.last ?? null,
    progress,
    attempts,
    drills,
    journalEntries: results.filter((r) => r.kind === "trade").length,
    checklists: results.filter((r) => r.kind === "checklist").length,
  };
}

function pick(o: Record<string, unknown>, keys: string[]): Record<string, unknown> {
  return Object.fromEntries(keys.filter((k) => o[k] !== undefined).map((k) => [k, o[k]]));
}

async function context(env: AuthEnv, origin: string) {
  return makeAuth(env, origin).$context;
}

export const makeAuthContext = context;

/** Signs the account out everywhere. */
export async function endSessions(env: AuthEnv, origin: string, userId: string): Promise<void> {
  const ctx = await context(env, origin);
  await ctx.internalAdapter.deleteUserSessions(userId);
}

export async function setSuspended(env: AuthEnv, origin: string, id: string, on: boolean, now = Date.now()) {
  await env.DB.prepare('UPDATE "user" SET "suspendedAt" = ?1 WHERE id = ?2').bind(on ? now : null, id).run();
  if (on) await endSessions(env, origin, id);
}

export async function setFullAccess(env: AuthEnv, id: string, on: boolean) {
  await env.DB.prepare('UPDATE "user" SET "fullAccess" = ?1 WHERE id = ?2').bind(on ? 1 : 0, id).run();
}

/** Removes one lesson's progress and Question Records; returns what was removed so Undo can put it back. */
export async function resetLesson(env: AuthEnv, id: string, lesson: string): Promise<{ docs: { doc_id: string; kind: string; body: string }[] }> {
  const n = lesson.replace(/^lesson-/, "");
  const ids = [lesson, `qrec-${n}`];
  const { results } = await env.DB.prepare("SELECT doc_id, kind, body FROM docs WHERE user_id = ?1 AND doc_id IN (?2, ?3)").bind(id, ids[0], ids[1]).all<{ doc_id: string; kind: string; body: string }>();
  await env.DB.prepare("DELETE FROM docs WHERE user_id = ?1 AND doc_id IN (?2, ?3)").bind(id, ids[0], ids[1]).run();
  return { docs: results };
}

export async function restoreDocs(env: AuthEnv, id: string, docs: { doc_id: string; kind: string; body: string }[], now = Date.now()) {
  for (const d of docs) {
    await env.DB.prepare(
      "INSERT INTO docs (user_id, doc_id, kind, body, updated_at) VALUES (?1, ?2, ?3, ?4, ?5) ON CONFLICT (user_id, doc_id) DO UPDATE SET kind = excluded.kind, body = excluded.body, updated_at = excluded.updated_at",
    )
      .bind(id, d.doc_id, d.kind, d.body, now)
      .run();
  }
}

/** A one-time link that lets the Learner choose a new password, valid for 24 hours. */
export async function createResetLink(env: AuthEnv, origin: string, userId: string, createdBy: string, now = Date.now()) {
  const bytes = crypto.getRandomValues(new Uint8Array(24));
  const token = btoa(String.fromCharCode(...bytes)).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
  const id = crypto.randomUUID();
  // A new link replaces any older unused one.
  await env.DB.prepare("UPDATE password_resets SET used_at = ?1 WHERE user_id = ?2 AND used_at IS NULL").bind(now, userId).run();
  await env.DB.prepare("INSERT INTO password_resets (id, token_hash, user_id, created_by, created_at, expires_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")
    .bind(id, await sha256(token), userId, createdBy, now, now + RESET_LINK_MS)
    .run();
  return { id, url: `${origin}/#reset-${token}`, expiresAt: now + RESET_LINK_MS };
}

/** Spends a reset link: sets the new password and signs the account out everywhere. */
export async function useResetLink(env: AuthEnv, origin: string, token: string, password: string, now = Date.now()): Promise<"ok" | "invalid" | "weak"> {
  if (password.length < 10 || password.length > 128) return "weak";
  if (!/^[A-Za-z0-9_-]{20,100}$/.test(token)) return "invalid";
  const row = await env.DB.prepare("UPDATE password_resets SET used_at = ?1 WHERE token_hash = ?2 AND used_at IS NULL AND expires_at > ?1 RETURNING user_id")
    .bind(now, await sha256(token))
    .first<{ user_id: string }>();
  if (!row) return "invalid";
  const ctx = await context(env, origin);
  const hash = await ctx.password.hash(password);
  if (await ctx.internalAdapter.findCredentialAccount(row.user_id)) {
    await ctx.internalAdapter.updatePassword(row.user_id, hash);
  } else {
    await ctx.internalAdapter.createAccount({ userId: row.user_id, providerId: "credential", accountId: row.user_id, password: hash });
  }
  await ctx.internalAdapter.deleteUserSessions(row.user_id);
  return "ok";
}

/** Account Deletion, step 1 (ADR 0008): suspend now, erase after the grace period. */
export async function scheduleDeletion(env: AuthEnv, origin: string, id: string, now = Date.now()): Promise<number> {
  const after = now + DELETE_GRACE_MS;
  await env.DB.prepare('UPDATE "user" SET "deleteAfter" = ?1, "suspendedAt" = COALESCE("suspendedAt", ?2) WHERE id = ?3').bind(after, now, id).run();
  await endSessions(env, origin, id);
  return after;
}

export async function cancelDeletion(env: AuthEnv, id: string) {
  await env.DB.prepare('UPDATE "user" SET "deleteAfter" = NULL, "suspendedAt" = NULL WHERE id = ?1').bind(id).run();
}

/**
 * Account Deletion, step 2: erase the person, keep anonymised numbers. Events and Claude usage move
 * to a random id; documents, screenshots, sign-in data and the account itself are removed; the audit
 * log keeps that it happened but loses the values.
 */
export async function erase(env: AuthEnv & { FILES?: R2Bucket }, id: string, now = Date.now()): Promise<string> {
  const anon = `anon-${crypto.randomUUID()}`;
  const files = await env.DB.prepare("SELECT id FROM files WHERE user_id = ?1").bind(id).all<{ id: string }>();
  if (env.FILES && files.results.length) await env.FILES.delete(files.results.map((f) => `${id}/${f.id}`));
  await env.DB.batch([
    env.DB.prepare("UPDATE events SET user_id = ?1 WHERE user_id = ?2").bind(anon, id),
    env.DB.prepare("UPDATE claude_calls SET user_id = ?1 WHERE user_id = ?2").bind(anon, id),
    env.DB.prepare("DELETE FROM docs WHERE user_id = ?1").bind(id),
    env.DB.prepare("DELETE FROM files WHERE user_id = ?1").bind(id),
    env.DB.prepare("DELETE FROM password_resets WHERE user_id = ?1").bind(id),
    env.DB.prepare("UPDATE invites SET used_by = NULL WHERE used_by = ?1").bind(id),
    env.DB.prepare("UPDATE audit_log SET entity_id = ?1, before = NULL, after = NULL WHERE entity = 'learner' AND entity_id = ?2").bind(anon, id),
    env.DB.prepare("UPDATE audit_log SET actor_email = NULL WHERE actor_id = ?1").bind(id),
    env.DB.prepare('DELETE FROM "twoFactor" WHERE "userId" = ?1').bind(id),
    env.DB.prepare('DELETE FROM "session" WHERE "userId" = ?1').bind(id),
    env.DB.prepare('DELETE FROM "account" WHERE "userId" = ?1').bind(id),
    env.DB.prepare('DELETE FROM "user" WHERE id = ?1').bind(id),
    env.DB.prepare("INSERT INTO audit_log (actor_id, actor_email, actor_role, action, entity, entity_id, before, after, at) VALUES ('system', NULL, 'system', 'learner.erased', 'learner', ?1, NULL, NULL, ?2)").bind(anon, now),
  ]);
  return anon;
}

/** Runs daily (cron): erases every account whose grace period has ended. */
/** Staff roles (D3): the Owner assigns Instructor or Support. The account is signed out everywhere, so
 * the new role (and, for staff, the two-factor setup) applies from its next sign-in. */
export async function setRole(env: AuthEnv, id: string, role: "learner" | "instructor" | "support"): Promise<void> {
  await env.DB.batch([
    env.DB.prepare('UPDATE "user" SET role = ?1 WHERE id = ?2').bind(role, id),
    env.DB.prepare('DELETE FROM "session" WHERE "userId" = ?1').bind(id),
  ]);
}

export async function eraseDue(env: AuthEnv & { FILES?: R2Bucket }, now = Date.now()): Promise<number> {
  const owners = ownerEmails(env);
  const { results } = await env.DB.prepare('SELECT id, email FROM "user" WHERE "deleteAfter" IS NOT NULL AND "deleteAfter" <= ?1').bind(now).all<{ id: string; email: string }>();
  let n = 0;
  for (const r of results) {
    if (owners.includes(r.email.toLowerCase())) continue;
    await erase(env, r.id, now);
    n++;
  }
  return n;
}

/** Step-up check (ADR 0007): the staff member's current authenticator code, for sensitive actions. */
export async function verifyCode(env: AuthEnv, origin: string, userId: string, code: string, now = Date.now()): Promise<boolean> {
  const clean = code.replace(/\s+/g, "");
  if (!/^\d{6}$/.test(clean)) return false;
  const row = await env.DB.prepare('SELECT secret FROM "twoFactor" WHERE "userId" = ?1').bind(userId).first<{ secret: string }>();
  if (!row) return false;
  const ctx = await context(env, origin);
  const secret = await symmetricDecrypt({ key: ctx.secretConfig, data: row.secret });
  if (secret.startsWith("base32:")) return false; // imported secrets aren't used by Telosgard
  for (const step of [-1, 0, 1]) if ((await totpAt(secret, now + step * 30000)) === clean) return true;
  return false;
}
