import { describe, expect, it } from "vitest";
import { call, db, invite, signIn, signUp, staffWithTwoFactor, totp } from "./helpers";

// One Owner for the file (sign-up happens once); both tests use its session.
let owner: { secret: string; cookie: string };

describe("Settings and staff roles (D3)", () => {
  it("saves limits, runs Maintenance Mode, publishes a Disclaimer version, and reports keys without values", async () => {
    owner = await staffWithTwoFactor("owner@example.com");
    const o = owner.cookie;
    const learner = await signUp("lena@example.com", await invite(o));

    const first = (await call("GET", "/api/admin/settings", undefined, { cookie: o })).data;
    expect(first.limits).toEqual({ owner: 300, learner: 50, total: null });
    expect(first.maintenance).toMatchObject({ on: false });
    expect(first.disclaimer).toMatchObject({ version: 1 });
    const status = first.status as { name: string; state: string; detail: string }[];
    expect(status.find((s) => s.name === "Anthropic API key")).toMatchObject({ state: "missing" });
    expect(status.find((s) => s.name === "FULL_ACCESS_EMAILS")).toMatchObject({ state: "warn" });
    // Presence only: no secret or address appears anywhere in the answer.
    const raw = JSON.stringify(first);
    for (const secret of ["test-secret-for-vitest", "owner@example.com", "reviewer@example.com"]) expect(raw).not.toContain(secret);

    // Limits: validated, saved, applied, audited.
    expect((await call("PUT", "/api/admin/settings/limits", { owner: 0, learner: 5 }, { cookie: o })).status).toBe(400);
    expect((await call("PUT", "/api/admin/settings/limits", { owner: 100, learner: 0, total: "" }, { cookie: o })).data).toEqual({ owner: 100, learner: 0, total: null });

    // Maintenance: /api/me tells everyone; Claude is refused for Learners; saves still work.
    const on = await call("PUT", "/api/admin/settings/maintenance", { on: true, messageEn: "Back at 18:00 UTC.", messageFr: "De retour à 18 h UTC." }, { cookie: o });
    expect(on.data).toMatchObject({ on: true, messageEn: "Back at 18:00 UTC." });
    expect((await call("GET", "/api/me", undefined, { cookie: learner.cookie })).data).toMatchObject({ maintenance: { messageEn: "Back at 18:00 UTC.", messageFr: "De retour à 18 h UTC." } });
    expect((await call("GET", "/api/me")).data).toMatchObject({ signedIn: false, maintenance: { messageEn: "Back at 18:00 UTC." } });
    const ask = await call("POST", "/api/claude/text", { prompt: "hi" }, { cookie: learner.cookie });
    expect([ask.status, ask.data.code]).toEqual([503, "maintenance"]);
    expect((await call("PUT", "/api/docs/profile", { kind: "profile", theme: "dark" }, { cookie: learner.cookie })).status).toBe(200);
    // Turning it back off keeps nothing behind.
    await call("PUT", "/api/admin/settings/maintenance", { on: false, messageEn: "", messageFr: "" }, { cookie: o });
    expect((await call("GET", "/api/me", undefined, { cookie: learner.cookie })).data).toMatchObject({ maintenance: null });

    // Disclaimer: both languages required; unchanged text refused; a new version reaches /api/me.
    expect((await call("POST", "/api/admin/settings/disclaimer", { en: "Only English", fr: "" }, { cookie: o })).status).toBe(400);
    const v1 = first.disclaimer as { en: string; fr: string };
    expect((await call("POST", "/api/admin/settings/disclaimer", { en: v1.en, fr: v1.fr }, { cookie: o })).status).toBe(400);
    const v2 = await call("POST", "/api/admin/settings/disclaimer", { en: v1.en + "\n\nNew paragraph.", fr: v1.fr + "\n\nNouveau paragraphe." }, { cookie: o });
    expect(v2.data).toMatchObject({ version: 2 });
    expect((await call("GET", "/api/me", undefined, { cookie: learner.cookie })).data).toMatchObject({ disclaimer: { version: 2 } });
    const history = (await call("GET", "/api/admin/settings", undefined, { cookie: o })).data.history as { version: number; publishedBy: string | null }[];
    expect(history.map((h) => h.version)).toEqual([2, 1]);
    expect(history[0].publishedBy).toBe("owner@example.com");

    const log = (await call("GET", "/api/admin/audit", undefined, { cookie: o })).data as unknown as { action: string }[];
    expect(log.map((r) => r.action)).toEqual(expect.arrayContaining(["settings.limits", "settings.maintenance_on", "settings.maintenance_off", "settings.disclaimer"]));

    // Only the Owner: a Support account can't read or change Settings.
    const sup = await staffWithTwoFactor("sup2@example.com", await invite(o));
    await db().prepare("UPDATE \"user\" SET role = 'support' WHERE email = 'sup2@example.com'").run();
    expect((await call("GET", "/api/admin/settings", undefined, { cookie: sup.cookie })).status).toBe(403);
    expect((await call("PUT", "/api/admin/settings/limits", { owner: 1, learner: 1 }, { cookie: sup.cookie })).status).toBe(403);
    await db().prepare("DELETE FROM settings").run();
  });

  it("lets the Owner assign Instructor and Support with the code, signs the account out, and can undo it", async () => {
    const o = owner.cookie;
    const kim = await signUp("kim@example.com", await invite(o));
    const id = String((await call("GET", "/api/me", undefined, { cookie: kim.cookie })).data.userId);

    // The code is required; a wrong role is refused.
    expect((await call("POST", `/api/admin/learners/${id}/role`, { role: "instructor" }, { cookie: o })).data.code).toBe("code_required");
    expect((await call("POST", `/api/admin/learners/${id}/role`, { role: "owner" }, { cookie: o, code: await totp(owner.secret) })).status).toBe(400);

    const made = await call("POST", `/api/admin/learners/${id}/role`, { role: "instructor" }, { cookie: o, code: await totp(owner.secret) });
    expect(made.status).toBe(200);
    // Signed out everywhere; on the next sign-in the role applies, and the dashboard asks for two-factor.
    expect((await call("GET", "/api/me", undefined, { cookie: kim.cookie })).data).toMatchObject({ signedIn: false });
    const again = await signIn("kim@example.com");
    expect((await call("GET", "/api/me", undefined, { cookie: again.cookie })).data).toMatchObject({ staff: true });
    expect((await call("GET", "/api/admin/me", undefined, { cookie: again.cookie })).data).toMatchObject({ role: "instructor", ready: false, blocked: "two_factor_required" });

    // Instructors and Support can't assign roles, and nobody can make an Owner.
    const sup = await staffWithTwoFactor("sup3@example.com", await invite(o));
    await db().prepare("UPDATE \"user\" SET role = 'support' WHERE email = 'sup3@example.com'").run();
    expect((await call("POST", `/api/admin/learners/${id}/role`, { role: "learner" }, { cookie: sup.cookie, code: await totp(sup.secret) })).status).toBe(403);

    // Undo puts the Learner role back (with the code again).
    const log = (await call("GET", "/api/admin/audit", undefined, { cookie: o })).data as unknown as { id: number; action: string; undoable: boolean; before: unknown; after: unknown }[];
    const row = log.find((r) => r.action === "learner.role")!;
    expect(row).toMatchObject({ undoable: true, before: { role: "learner" }, after: { role: "instructor" } });
    expect((await call("POST", `/api/admin/audit/${row.id}/undo`, undefined, { cookie: o })).data.code).toBe("code_required");
    expect((await call("POST", `/api/admin/audit/${row.id}/undo`, undefined, { cookie: o, code: await totp(owner.secret) })).status).toBe(200);
    const role = await db().prepare('SELECT role FROM "user" WHERE id = ?1').bind(id).first<{ role: string }>();
    expect(role?.role).toBe("learner");
  });
});
