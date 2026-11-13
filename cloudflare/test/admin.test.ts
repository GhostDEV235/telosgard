import { describe, expect, it } from "vitest";
import { deriveEvents } from "../src/events";
import { can, storedRole } from "../src/roles";
import { staffBlock } from "../src/admin";
import { call, db, invite, PASSWORD, signIn, signUp, staffWithTwoFactor, totp } from "./helpers";

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
    expect(JSON.parse(signedUp!.data).via).toMatch(/^link:/);
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
