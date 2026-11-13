import { describe, expect, it } from "vitest";
import { call, db, invite, signUp, staffWithTwoFactor } from "./helpers";

const progress = (passedAt: number | null) => ({ kind: "progress", quizBest: 100, grade: passedAt ? "got_it" : "almost", passedAt });

describe("Curriculum and Overview (D2)", () => {
  it("counts lessons, hides one with the paused count, and keeps answers anonymous", async () => {
    const owner = await staffWithTwoFactor("owner@example.com");
    const o = owner.cookie;
    const a = await signUp("ann@example.com", await invite(o));
    const b = await signUp("bo@example.com", await invite(o));

    // Ann passed Lessons 1 and 2; Bo passed Lesson 1 and is stuck on 2.
    const t = Date.now();
    await call("PUT", "/api/docs/lesson-01", progress(t), { cookie: a.cookie });
    await call("PUT", "/api/docs/lesson-02", progress(t), { cookie: a.cookie });
    await call("PUT", "/api/docs/lesson-01", progress(t), { cookie: b.cookie });
    await call("PUT", "/api/docs/lesson-02", progress(null), { cookie: b.cookie });
    await call("PUT", "/api/docs/qrec-02", { kind: "qrec", lesson: 2, questions: { q1: { answered: 3, wrong: 2, guessed: 1 }, q2: { answered: 1, wrong: 0, guessed: 0 } } }, { cookie: b.cookie });
    await call("PUT", "/api/docs/attempt-x1", { kind: "attempt", lesson: "lesson-02", grade: "not_yet", answer: "Bo's secret words", at: t }, { cookie: b.cookie });

    const sum = (await call("GET", "/api/admin/curriculum", undefined, { cookie: o })).data as unknown as Record<string, number | string | boolean>[];
    const l2 = sum.find((s) => s.lesson === "lesson-02")!;
    expect(l2).toMatchObject({ started: 2, passed: 1, waiting: 1, answered: 4, wrong: 2, notYet: 1, hidden: false });
    // Ann is waiting at Lesson 3 before saving anything there; nobody is left at Lesson 1.
    expect(sum.find((s) => s.lesson === "lesson-03")).toMatchObject({ started: 0, waiting: 1 });
    expect(sum.find((s) => s.lesson === "lesson-01")).toMatchObject({ waiting: 0 });

    // Hiding Lesson 2 reports who it pauses; the app learns about it from /api/me.
    const hid = await call("POST", "/api/admin/curriculum/lesson-02/hide", { reason: "q1 is ambiguous" }, { cookie: o });
    expect(hid.data).toMatchObject({ ok: true, paused: 1 });
    expect((await call("GET", "/api/me", undefined, { cookie: b.cookie })).data).toMatchObject({ hidden: ["lesson-02"] });
    const detail = (await call("GET", "/api/admin/curriculum/lesson-02", undefined, { cookie: o })).data;
    expect(detail).toMatchObject({ hidden: true, reason: "q1 is ambiguous", questions: [{ q: "q1", answered: 3, wrong: 2 }, { q: "q2", answered: 1 }] });
    const answers = detail.answers as Record<string, unknown>[];
    expect(answers).toEqual([{ grade: "not_yet", answer: "Bo's secret words", at: t }]);
    expect(JSON.stringify(detail)).not.toContain("bo@example.com");

    // Undo puts it back.
    const log = (await call("GET", "/api/admin/audit", undefined, { cookie: o })).data as unknown as { id: number; action: string; undoable: boolean }[];
    const hideRow = log.find((r) => r.action === "lesson.hide")!;
    expect(hideRow.undoable).toBe(true);
    expect((await call("POST", `/api/admin/audit/${hideRow.id}/undo`, undefined, { cookie: o })).status).toBe(200);
    expect((await call("GET", "/api/me", undefined, { cookie: b.cookie })).data).toMatchObject({ hidden: [] });

    // Instructors hide, read answers and write notes; Support reads stats but no answers and can't hide.
    const ins = await staffWithTwoFactor("ins@example.com", await invite(o));
    const sup = await staffWithTwoFactor("sup@example.com", await invite(o));
    await db().prepare("UPDATE \"user\" SET role = 'instructor' WHERE email = 'ins@example.com'").run();
    await db().prepare("UPDATE \"user\" SET role = 'support' WHERE email = 'sup@example.com'").run();
    expect((await call("POST", "/api/admin/curriculum/lesson-02/notes", { body: "Rewrite q1's second option." }, { cookie: ins.cookie })).status).toBe(200);
    expect((await call("POST", "/api/admin/curriculum/lesson-02/hide", {}, { cookie: ins.cookie })).data).toMatchObject({ paused: 1 });
    const insView = (await call("GET", "/api/admin/curriculum/lesson-02", undefined, { cookie: ins.cookie })).data;
    expect((insView.answers as unknown[]).length).toBe(1);
    expect(insView.notes).toMatchObject([{ author: "ins@example.com", body: "Rewrite q1's second option." }]);
    const supView = (await call("GET", "/api/admin/curriculum/lesson-02", undefined, { cookie: sup.cookie })).data;
    expect(supView.answers).toEqual([]);
    expect((await call("POST", "/api/admin/curriculum/lesson-02/unhide", {}, { cookie: sup.cookie })).status).toBe(403);
    expect((await call("POST", "/api/admin/curriculum/lesson-02/notes", { body: "x" }, { cookie: sup.cookie })).status).toBe(403);
    expect((await call("GET", "/api/admin/curriculum/../learners", undefined, { cookie: ins.cookie })).status).not.toBe(200);

    // The Overview: passes per lesson, activation, At-Risk names only for roles that may see Learners.
    const ov = (await call("GET", "/api/admin/overview", undefined, { cookie: o })).data as Record<string, unknown>;
    expect(ov.passedCounts).toMatchObject({ "lesson-01": 2, "lesson-02": 1 });
    expect(ov).toMatchObject({ learners: 2, activation: { cohort: 0, activated: 0 }, claude: { today: { calls: 0, cost: 0 } } });
    const insOv = (await call("GET", "/api/admin/overview", undefined, { cookie: ins.cookie })).data as Record<string, { list: unknown[] }>;
    expect(insOv.atRisk.list).toEqual([]);
  });
});
