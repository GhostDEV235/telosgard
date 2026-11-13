import { env } from "cloudflare:test";
import { describe, expect, it } from "vitest";
import { call, db, invite, PASSWORD, signIn, signUp, staffWithTwoFactor, totp } from "./helpers";
import { eraseDue } from "../src/learners";
import type { Env } from "../src/index";

// Storage is shared across this file's tests, so the Owner signs up (with two-factor) only once.
let ownerSession: ReturnType<typeof staffWithTwoFactor> | undefined;
const theOwner = () => (ownerSession ??= staffWithTwoFactor("owner@example.com"));

const idOf = async (email: string) => (await db().prepare('SELECT id FROM "user" WHERE email = ?1').bind(email).first<{ id: string }>())!.id;

describe("Learners and Enrollment (D1)", () => {
  it("runs the whole Learner lifecycle from the dashboard", async () => {
    const owner = await theOwner();
    const o = owner.cookie;
    const code = async () => totp(owner.secret);

    // An Invite Code lets a group join, up to its limit; a revoked code stops working.
    const made = await call("POST", "/api/admin/enrollment/codes", { code: "gold nov", maxUses: 2, days: 7 }, { cookie: o });
    expect(made.data).toMatchObject({ code: "GOLD-NOV", maxUses: 2, uses: 0 });
    expect((await call("POST", "/api/admin/enrollment/codes", { code: "GOLD-NOV", maxUses: 2, days: 7 }, { cookie: o })).status).toBe(400);
    const a = await signUp("ada@example.com", "gold-nov");
    expect(a.status).toBe(200);
    expect((await signUp("ben@example.com", "GOLD-NOV")).status).toBe(200);
    expect((await signUp("cy@example.com", "GOLD-NOV")).status).toBe(403);
    const list = await call("GET", "/api/admin/enrollment", undefined, { cookie: o });
    expect((list.data.codes as { uses: number }[])[0].uses).toBe(2);

    const learners = await call("GET", "/api/admin/learners", undefined, { cookie: o });
    const ada = (learners.data as unknown as { email: string; joined: string; id: string }[]).find((l) => l.email === "ada@example.com")!;
    expect(ada.joined).toBe("Code GOLD-NOV");
    expect((learners.data as unknown as { email: string }[]).some((l) => l.email === "owner@example.com")).toBe(false);

    // Progress, then a lesson reset that Undo puts back.
    await call("PUT", "/api/docs/lesson-01", { kind: "progress", quizBest: 100, grade: "got_it", passedAt: 5 }, { cookie: a.cookie });
    await call("PUT", "/api/docs/attempt-1", { kind: "attempt", lesson: "1", grade: "got_it", answer: "My words", at: 4 }, { cookie: a.cookie });
    await call("PUT", "/api/docs/trade-1", { kind: "trade", secret: "journal text" }, { cookie: a.cookie });
    const profile = await call("GET", `/api/admin/learners/${ada.id}`, undefined, { cookie: o });
    expect(profile.data).toMatchObject({ progress: [{ lesson: "lesson-01", quizBest: 100 }], journalEntries: 1 });
    expect(JSON.stringify(profile.data)).not.toContain("journal text");
    expect((await call("POST", `/api/admin/learners/${ada.id}/reset-lesson`, { lesson: "lesson-01" }, { cookie: o })).data).toMatchObject({ removed: 1 });
    expect((await call("GET", "/api/docs/lesson-01", undefined, { cookie: a.cookie })).data).toBeNull();
    const log1 = (await call("GET", "/api/admin/audit", undefined, { cookie: o })).data as unknown as { id: number; action: string; undoable: boolean }[];
    const reset = log1.find((r) => r.action === "learner.reset_lesson")!;
    expect(reset.undoable).toBe(true);
    expect((await call("POST", `/api/admin/audit/${reset.id}/undo`, undefined, { cookie: o })).status).toBe(200);
    expect((await call("GET", "/api/docs/lesson-01", undefined, { cookie: a.cookie })).data).toMatchObject({ passedAt: 5 });
    expect((await call("POST", `/api/admin/audit/${reset.id}/undo`, undefined, { cookie: o })).status).toBe(409);

    // Full Access needs the current authenticator code.
    expect((await call("POST", `/api/admin/learners/${ada.id}/full-access`, { on: true }, { cookie: o })).data).toMatchObject({ code: "code_required" });
    expect((await call("POST", `/api/admin/learners/${ada.id}/full-access`, { on: true }, { cookie: o, code: "123456" })).data).toMatchObject({ code: "code_required" });
    expect((await call("POST", `/api/admin/learners/${ada.id}/full-access`, { on: true }, { cookie: o, code: await code() })).status).toBe(200);
    expect((await call("GET", "/api/me", undefined, { cookie: a.cookie })).data).toMatchObject({ fullAccess: true });

    // Suspended: signed out at once, and sign-in is refused; unsuspending lets them back in.
    expect((await call("POST", `/api/admin/learners/${ada.id}/suspend`, { on: true }, { cookie: o })).status).toBe(200);
    expect((await call("GET", "/api/me", undefined, { cookie: a.cookie })).data).toEqual({ signedIn: false });
    expect((await signIn("ada@example.com")).data).toMatchObject({ code: "ACCOUNT_SUSPENDED" });
    await call("POST", `/api/admin/learners/${ada.id}/suspend`, { on: false }, { cookie: o });
    const back = await signIn("ada@example.com");
    expect(back.status).toBe(200);

    // A one-time reset link sets a new password, signs out everywhere, and can't be reused.
    const link = await call("POST", `/api/admin/learners/${ada.id}/reset-link`, undefined, { cookie: o });
    const token = String(link.data.url).split("#reset-")[1];
    expect(JSON.stringify((await call("GET", "/api/admin/audit", undefined, { cookie: o })).data)).not.toContain(token);
    expect((await call("POST", "/api/password-reset", { token, password: "short" })).status).toBe(400);
    expect((await call("POST", "/api/password-reset", { token, password: "a brand new password" })).status).toBe(200);
    expect((await call("POST", "/api/password-reset", { token, password: "a brand new password" })).status).toBe(400);
    expect((await call("GET", "/api/me", undefined, { cookie: back.cookie })).data).toEqual({ signedIn: false });
    expect((await call("POST", "/api/auth/sign-in/email", { email: "ada@example.com", password: "a brand new password" })).status).toBe(200);

    // Account Deletion: suspended for 7 days (undoable), then erased with numbers kept anonymously.
    expect((await call("POST", `/api/admin/learners/${ada.id}/delete`, undefined, { cookie: o })).data).toMatchObject({ code: "code_required" });
    const del = await call("POST", `/api/admin/learners/${ada.id}/delete`, undefined, { cookie: o, code: await code() });
    expect(Number(del.data.deleteAfter)).toBeGreaterThan(Date.now() + 6 * 24 * 3600_000);
    expect((await call("POST", "/api/auth/sign-in/email", { email: "ada@example.com", password: "a brand new password" })).status).toBe(403);
    const eventsBefore = await db().prepare("SELECT COUNT(*) AS n FROM events WHERE user_id = ?1").bind(ada.id).first<{ n: number }>();
    expect(await eraseDue(env as unknown as Env, Date.now())).toBe(0);
    expect(await eraseDue(env as unknown as Env, Date.now() + 8 * 24 * 3600_000)).toBe(1);
    expect(await db().prepare('SELECT id FROM "user" WHERE id = ?1').bind(ada.id).first()).toBeNull();
    expect((await db().prepare("SELECT COUNT(*) AS n FROM docs WHERE user_id = ?1").bind(ada.id).first<{ n: number }>())!.n).toBe(0);
    const anon = await db().prepare("SELECT user_id, COUNT(*) AS n FROM events WHERE user_id LIKE 'anon-%' GROUP BY user_id").first<{ user_id: string; n: number }>();
    expect(anon!.n).toBe(eventsBefore!.n);
    const leftovers = await db().prepare("SELECT COUNT(*) AS n FROM audit_log WHERE entity_id = ?1 OR before LIKE ?2 OR after LIKE ?2").bind(ada.id, "%ada@example.com%").first<{ n: number }>();
    expect(leftovers!.n).toBe(0);
  });

  it("keeps Support and Instructors inside their roles", async () => {
    const owner = await theOwner();
    const o = owner.cookie;
    const sup = await staffWithTwoFactor("support@example.com", await invite(o));
    const ins = await staffWithTwoFactor("instructor@example.com", await invite(o));
    await db().prepare("UPDATE \"user\" SET role = 'support' WHERE email = 'support@example.com'").run();
    await db().prepare("UPDATE \"user\" SET role = 'instructor' WHERE email = 'instructor@example.com'").run();
    expect((await signUp("lee@example.com", await invite(o))).status).toBe(200);
    const lee = await idOf("lee@example.com");
    const insId = await idOf("instructor@example.com");

    // Support: sees Learners, invites and suspends them; never Full Access, deletion, staff or the audit log.
    expect((await call("GET", "/api/admin/learners", undefined, { cookie: sup.cookie })).status).toBe(200);
    expect((await call("POST", "/api/admin/enrollment/links", undefined, { cookie: sup.cookie })).status).toBe(200);
    expect((await call("POST", `/api/admin/learners/${lee}/suspend`, { on: true }, { cookie: sup.cookie })).status).toBe(200);
    expect((await call("POST", `/api/admin/learners/${lee}/full-access`, { on: true }, { cookie: sup.cookie, code: await totp(sup.secret) })).data).toMatchObject({ code: "not_granted" });
    expect((await call("POST", `/api/admin/learners/${lee}/delete`, undefined, { cookie: sup.cookie, code: await totp(sup.secret) })).data).toMatchObject({ code: "not_granted" });
    expect((await call("POST", `/api/admin/learners/${insId}/suspend`, { on: true }, { cookie: sup.cookie })).status).toBe(403);
    expect((await call("GET", "/api/admin/audit", undefined, { cookie: sup.cookie })).status).toBe(403);

    // Instructor: no Learner identities at all.
    expect((await call("GET", "/api/admin/learners", undefined, { cookie: ins.cookie })).status).toBe(403);
    expect((await call("GET", `/api/admin/learners/${lee}`, undefined, { cookie: ins.cookie })).status).toBe(403);
    expect((await call("GET", "/api/admin/enrollment", undefined, { cookie: ins.cookie })).status).toBe(403);

    // Nobody acts on themselves, and Undo needs the original action's permission.
    expect((await call("POST", `/api/admin/learners/${await idOf("owner@example.com")}/suspend`, { on: true }, { cookie: o })).status).toBe(403);
    const log = (await call("GET", "/api/admin/audit", undefined, { cookie: o })).data as unknown as { id: number; action: string }[];
    const suspended = log.find((r) => r.action === "learner.suspend")!;
    expect((await call("POST", `/api/admin/audit/${suspended.id}/undo`, undefined, { cookie: ins.cookie })).status).toBe(403);
    expect((await call("POST", `/api/admin/audit/${suspended.id}/undo`, undefined, { cookie: sup.cookie })).status).toBe(200);
    expect((await signIn("lee@example.com")).status).toBe(200);
  });
});

describe("Learner self-service", () => {
  it("lets a Learner delete their own account with their password, and the Owner cancel it", async () => {
    const owner = await theOwner();
    const me = await signUp("mo@example.com", await invite(owner.cookie));
    expect((await call("POST", "/api/account/delete", { password: "not my password" }, { cookie: me.cookie })).status).toBe(403);
    expect((await call("POST", "/api/account/delete", { password: PASSWORD }, { cookie: me.cookie })).status).toBe(200);
    expect((await signIn("mo@example.com")).status).toBe(403);
    const mo = await idOf("mo@example.com");
    expect((await call("POST", `/api/admin/learners/${mo}/cancel-delete`, undefined, { cookie: owner.cookie })).status).toBe(200);
    expect((await signIn("mo@example.com")).status).toBe(200);
  });
});
