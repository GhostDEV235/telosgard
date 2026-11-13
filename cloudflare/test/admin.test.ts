import { env } from "cloudflare:test";
import { describe, expect, it } from "vitest";
import { handle, type Env } from "../src/index";
import { INVITE_HEADER } from "../src/auth";
import { deriveEvents } from "../src/events";
import { can, storedRole } from "../src/roles";
import { staffBlock } from "../src/admin";

const origin = "http://localhost";
const e = () => ({ ...env, DEV_USER: undefined }) as unknown as Env;
const noClaude = () => {
  throw new Error("no Claude in admin tests");
};
const db = () => (env as unknown as { DB: D1Database }).DB;

let ip = 1000;
async function call(method: string, path: string, body?: unknown, opts: { cookie?: string; invite?: string } = {}) {
  ip++;
  const headers: Record<string, string> = { origin, "content-type": "application/json", "cf-connecting-ip": `10.9.${Math.floor(ip / 250)}.${ip % 250}` };
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

const PASSWORD = "correct horse battery";
const signUp = (email: string, invite?: string) => call("POST", "/api/auth/sign-up/email", { name: email.split("@")[0], email, password: PASSWORD }, { invite });
const signIn = (email: string) => call("POST", "/api/auth/sign-in/email", { email, password: PASSWORD });

/** RFC 6238 TOTP (SHA-1, 6 digits, 30 s) from a base32 secret, as an authenticator app computes it. */
async function totp(secret: string, at = Date.now()): Promise<string> {
  const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
  let bits = "";
  for (const ch of secret.replace(/=+$/, "").toUpperCase()) bits += alphabet.indexOf(ch).toString(2).padStart(5, "0");
  const key = new Uint8Array(Math.floor(bits.length / 8)).map((_, i) => parseInt(bits.slice(i * 8, i * 8 + 8), 2));
  const counter = Math.floor(at / 30000);
  const msg = new Uint8Array(8);
  new DataView(msg.buffer).setUint32(4, counter);
  const k = await crypto.subtle.importKey("raw", key, { name: "HMAC", hash: "SHA-1" }, false, ["sign"]);
  const h = new Uint8Array(await crypto.subtle.sign("HMAC", k, msg));
  const o = h[19] & 0xf;
  const n = ((h[o] & 0x7f) << 24) | (h[o + 1] << 16) | (h[o + 2] << 8) | h[o + 3];
  return String(n % 1_000_000).padStart(6, "0");
}

async function invite(ownerCookie: string): Promise<string> {
  const r = await call("POST", "/api/invites", undefined, { cookie: ownerCookie });
  return String(r.data.url).split("#join-")[1];
}

/** Signs up, turns on two-factor, then signs in again through the code step. Returns the secret and a fresh cookie. */
async function staffWithTwoFactor(email: string, inviteToken?: string) {
  const first = await signUp(email, inviteToken);
  expect(first.status).toBe(200);
  const enable = await call("POST", "/api/auth/two-factor/enable", { password: PASSWORD }, { cookie: first.cookie });
  expect(enable.status).toBe(200);
  const secret = new URL(String(enable.data.totpURI)).searchParams.get("secret")!;
  expect((enable.data.backupCodes as string[]).length).toBe(10);
  const verified = await call("POST", "/api/auth/two-factor/verify-totp", { code: await totp(secret) }, { cookie: first.cookie });
  expect(verified.status).toBe(200);

  const again = await signIn(email);
  expect(again.data).toMatchObject({ twoFactorRedirect: true });
  const done = await call("POST", "/api/auth/two-factor/verify-totp", { code: await totp(secret) }, { cookie: again.cookie });
  expect(done.status).toBe(200);
  return { secret, cookie: done.cookie };
}

describe("roles", () => {
  it("never stores the Owner and keeps permissions apart", () => {
    expect(storedRole("owner")).toBe("learner");
    expect(storedRole("support")).toBe("support");
    expect(can("instructor", "answers.read")).toBe(true);
    expect(can("instructor", "learners.read")).toBe(false);
    expect(can("support", "journals.read")).toBe(false);
    expect(can("support", "fullaccess.set")).toBe(false);
    expect(can("owner", "roles.set")).toBe(true);
  });

  it("closes the dashboard after 12 hours", () => {
    const now = Date.UTC(2026, 10, 13, 12);
    const who = { userId: "u", twoFactorEnabled: true, sessionStart: now - 11 * 3600_000 };
    expect(staffBlock(who, "owner", now)).toBeNull();
    expect(staffBlock({ ...who, sessionStart: now - 13 * 3600_000 }, "owner", now)?.code).toBe("session_expired");
    expect(staffBlock({ ...who, twoFactorEnabled: false }, "support", now)?.code).toBe("two_factor_required");
    expect(staffBlock(who, "learner", now)?.code).toBe("not_staff");
  });
});

describe("events from document saves", () => {
  it("derives quiz, pass, question, drill and journal events", () => {
    const t = 1_000;
    expect(deriveEvents("progress", "lesson-03", null, { quizBest: 80 }, t)).toEqual([{ type: "lesson_quiz", lesson: "lesson-03", data: { score: 80 }, at: t }]);
    expect(deriveEvents("progress", "lesson-03", { quizBest: 80 }, { quizBest: 60 }, t)).toEqual([]);
    const passed = deriveEvents("progress", "lesson-03", { quizBest: 80, grade: "almost" }, { quizBest: 80, grade: "got_it", passedAt: 900 }, t);
    expect(passed).toEqual([{ type: "lesson_passed", lesson: "lesson-03", data: {}, at: 900 }]);
    expect(deriveEvents("attempt", "attempt-x", null, { lesson: "3", grade: "almost", at: 950 }, t)).toEqual([{ type: "explain_back", lesson: "lesson-03", data: { grade: "almost" }, at: 950 }]);
    const q = deriveEvents("qrec", "qrec-03", { lesson: 3, questions: { q1: { answered: 1, wrong: 1, guessed: 0, lastAt: 1 } } }, { lesson: 3, questions: { q1: { answered: 3, wrong: 1, guessed: 1, lastAt: 990 }, q2: { answered: 0 } } }, t);
    expect(q).toEqual([{ type: "question", lesson: "lesson-03", data: { q: "q1", answered: 2, wrong: 0, guessed: 1 }, at: 990 }]);
    expect(deriveEvents("drill", "drill-surprise", { answered: 2, correct: 1 }, { answered: 3, correct: 2, lastAt: 999 }, t)).toEqual([{ type: "drill", data: { drill: "surprise", answered: 1, correct: 1 }, at: 999 }]);
    expect(deriveEvents("trade", "t1", null, {}, t)).toEqual([{ type: "journal_entry", data: {}, at: t }]);
    expect(deriveEvents("trade", "t1", {}, {}, t)).toEqual([]);
    expect(deriveEvents("plan", "plan-v1", null, {}, t)).toEqual([{ type: "activity", data: { kind: "plan" }, at: t }]);
  });
});

describe("the admin API", () => {
  it("is staff-only, needs two-factor, and logs writes", async () => {
    // Signed out, then the Owner without two-factor: told to set it up, refused everything else.
    expect((await call("GET", "/api/admin/me")).status).toBe(401);
    const owner0 = await signUp("owner@example.com");
    const me0 = await call("GET", "/api/admin/me", undefined, { cookie: owner0.cookie });
    expect(me0.data).toMatchObject({ role: "owner", ready: false, blocked: "two_factor_required" });
    expect((await call("GET", "/api/admin/overview", undefined, { cookie: owner0.cookie })).data).toMatchObject({ code: "two_factor_required" });
    expect((await call("GET", "/api/me", undefined, { cookie: owner0.cookie })).data).toMatchObject({ staff: true });

    // With two-factor on, the next sign-in asks for the code; then the dashboard opens.
    const enable = await call("POST", "/api/auth/two-factor/enable", { password: PASSWORD }, { cookie: owner0.cookie });
    const secret = new URL(String(enable.data.totpURI)).searchParams.get("secret")!;
    expect((await call("POST", "/api/auth/two-factor/verify-totp", { code: await totp(secret) }, { cookie: owner0.cookie })).status).toBe(200);
    const pending = await signIn("owner@example.com");
    expect(pending.data).toMatchObject({ twoFactorRedirect: true });
    expect((await call("GET", "/api/admin/me", undefined, { cookie: pending.cookie })).status).toBe(401);
    expect((await call("POST", "/api/auth/two-factor/verify-totp", { code: "000000" }, { cookie: pending.cookie })).status).not.toBe(200);
    const owner = (await call("POST", "/api/auth/two-factor/verify-totp", { code: await totp(secret) }, { cookie: pending.cookie })).cookie;
    expect((await call("GET", "/api/admin/me", undefined, { cookie: owner })).data).toMatchObject({ role: "owner", ready: true });

    // A Learner joins and studies; their saves become events, and the Overview counts them.
    const learner = await signUp("learner@example.com", await invite(owner));
    expect((await call("GET", "/api/admin/me", undefined, { cookie: learner.cookie })).data).toMatchObject({ code: "not_staff" });
    expect((await call("GET", "/api/me", undefined, { cookie: learner.cookie })).data).toMatchObject({ staff: false });
    await call("PUT", "/api/docs/lesson-01", { kind: "progress", quizBest: 100 }, { cookie: learner.cookie });
    await call("PUT", "/api/docs/lesson-01", { kind: "progress", quizBest: 100, grade: "got_it", passedAt: Date.now() }, { cookie: learner.cookie });
    const { results } = await db().prepare("SELECT type, lesson FROM events WHERE user_id = (SELECT id FROM \"user\" WHERE email = 'learner@example.com') ORDER BY id").all();
    expect(results.map((r) => r.type)).toEqual(["signed_up", "lesson_quiz", "lesson_passed"]);
    const signedUp = await db().prepare("SELECT data FROM events WHERE type = 'signed_up' AND user_id = (SELECT id FROM \"user\" WHERE email = 'learner@example.com')").first<{ data: string }>();
    expect(JSON.parse(signedUp!.data).invite).toBeTruthy();
    expect((await call("GET", "/api/admin/overview", undefined, { cookie: owner })).data).toMatchObject({ learners: 1, newThisWeek: 1, weeklyActive: 1 });

    // Backfill rebuilds history from stored documents, replaces itself, and lands in the audit log.
    const b1 = await call("POST", "/api/admin/events/backfill", undefined, { cookie: owner });
    const b2 = await call("POST", "/api/admin/events/backfill", undefined, { cookie: owner });
    expect(b1.data).toEqual(b2.data);
    const n = await db().prepare("SELECT COUNT(*) AS n FROM events WHERE backfill = 1").first<{ n: number }>();
    expect(n!.n).toBe(b2.data.events);
    const log = await call("GET", "/api/admin/audit", undefined, { cookie: owner });
    expect((log.data as unknown as { action: string; actorRole: string }[]).map((r) => r.action)).toEqual(["events.backfill", "events.backfill"]);

    // An Instructor (role set in the database) can see the Overview but not the audit log.
    const tokenI = await invite(owner);
    const instructor = await staffWithTwoFactor("instructor@example.com", tokenI);
    await db().prepare("UPDATE \"user\" SET role = 'instructor' WHERE email = 'instructor@example.com'").run();
    expect((await call("GET", "/api/admin/me", undefined, { cookie: instructor.cookie })).data).toMatchObject({ role: "instructor", ready: true });
    expect((await call("GET", "/api/admin/overview", undefined, { cookie: instructor.cookie })).status).toBe(200);
    expect((await call("GET", "/api/admin/audit", undefined, { cookie: instructor.cookie })).data).toMatchObject({ code: "not_granted" });
    expect((await call("POST", "/api/admin/events/backfill", undefined, { cookie: instructor.cookie })).status).toBe(403);

    // A stored "owner" role grants nothing: the Owner comes only from the secret.
    await db().prepare("UPDATE \"user\" SET role = 'owner' WHERE email = 'learner@example.com'").run();
    expect((await call("GET", "/api/admin/me", undefined, { cookie: learner.cookie })).data).toMatchObject({ code: "not_staff" });
  });
});
