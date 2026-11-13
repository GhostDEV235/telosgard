// The admin API, /api/admin/* (ADR 0006, 0007). Staff only: Owner (by secret), Instructor or Support,
// with two-factor turned on and a session signed in within the last 12 hours. Each route names the
// permission it needs (roles.ts); every write goes to the audit log.
//
//   GET  /api/admin/me              → { role, email, twoFactorEnabled, ready, permissions, sessionEndsAt }
//                                     (answers before two-factor is set up, so the dashboard can ask for it)
//   GET  /api/admin/overview        → { learners, newThisWeek, weeklyActive, eventsToday }
//   GET  /api/admin/audit?before=&limit=  → AuditRow[] (newest first)
//   POST /api/admin/events/backfill → { users, events }
//   GET  /api/admin/learners        → LearnerRow[]          GET /api/admin/learners/:id → profile
//   POST /api/admin/learners/:id/(suspend|full-access|reset-lesson|reset-link|delete|cancel-delete)
//   GET  /api/admin/enrollment      → { links, codes }
//   POST /api/admin/enrollment/links · DELETE /api/admin/enrollment/links/:id
//   POST /api/admin/enrollment/codes · POST /api/admin/enrollment/codes/:id/(revoke|restore)
//   POST /api/admin/audit/:id/undo
//
// Sensitive actions (Full Access, deletion) also need the staff member's current authenticator code
// in the x-telosgard-code header; without it they answer 403 code_required (ADR 0007).

import { audit, auditPage } from "./audit";
import { ownerEmails, roleOf, type AuthEnv, type Identity } from "./auth";
import { backfill } from "./events";
import { createInviteCode, createInviteLink, listEnrollment } from "./enroll";
import {
  cancelDeletion,
  createResetLink,
  getAccount,
  learnerProfile,
  listLearners,
  resetLesson,
  restoreDocs,
  scheduleDeletion,
  setFullAccess,
  setSuspended,
  verifyCode,
  type Account,
} from "./learners";
import { can, isStaff, permissionsOf, STAFF_SESSION_MS, type Permission, type Role } from "./roles";

const json = (body: unknown, status = 200) =>
  new Response(JSON.stringify(body), { status, headers: { "content-type": "application/json; charset=utf-8", "cache-control": "no-store" } });
const fail = (status: number, code: string, message: string) => json({ code, message }, status);

const DAY = 24 * 60 * 60 * 1000;

interface Staff {
  who: Identity;
  role: Role;
}

/** Why a staff member can't use the dashboard yet, or null when they can. */
export function staffBlock(who: Identity, role: Role, now = Date.now()): { code: string; message: string } | null {
  if (!isStaff(role)) return { code: "not_staff", message: "This account has no dashboard access." };
  if (who.dev) return null;
  if (!who.twoFactorEnabled) return { code: "two_factor_required", message: "Turn on two-factor sign-in to use the dashboard." };
  if (who.sessionStart === undefined || now - who.sessionStart > STAFF_SESSION_MS) {
    return { code: "session_expired", message: "Dashboard sessions last 12 hours. Sign in again." };
  }
  return null;
}

export async function adminRoute(request: Request, env: AuthEnv, who: Identity, rest: string[], url: URL): Promise<Response> {
  const role = roleOf(env, who);
  const method = request.method;

  if (rest[0] === "me" && rest.length === 1 && method === "GET") {
    if (!isStaff(role)) return fail(403, "not_staff", "This account has no dashboard access.");
    const block = staffBlock(who, role);
    return json({
      role,
      email: who.email ?? null,
      twoFactorEnabled: who.dev === true || who.twoFactorEnabled === true,
      ready: block === null,
      blocked: block?.code ?? null,
      permissions: permissionsOf(role),
      sessionEndsAt: who.sessionStart !== undefined ? who.sessionStart + STAFF_SESSION_MS : null,
    });
  }

  const block = staffBlock(who, role);
  if (block) return fail(403, block.code, block.message);
  const staff: Staff = { who, role };
  const need = (p: Permission) => (can(role, p) ? null : fail(403, "not_granted", "Your role can't do that."));

  if (rest[0] === "overview" && rest.length === 1 && method === "GET") {
    return need("overview.read") ?? json(await overview(env.DB, env));
  }
  if (rest[0] === "audit" && rest.length === 1 && method === "GET") {
    const denied = need("audit.read");
    if (denied) return denied;
    const limit = Math.min(200, Math.max(1, Number(url.searchParams.get("limit")) || 50));
    const before = Number(url.searchParams.get("before"));
    const page = await auditPage(env.DB, limit, Number.isFinite(before) && before > 0 ? before : undefined);
    const undone = await env.DB.prepare("SELECT entity_id FROM audit_log WHERE action = 'undo' AND entity = 'audit'").all<{ entity_id: string }>();
    const gone = new Set(undone.results.map((r) => Number(r.entity_id)));
    return json(page.map((r) => ({ ...r, undoable: undoable(r.action) && !gone.has(r.id), undone: gone.has(r.id) })));
  }
  if (rest[0] === "events" && rest[1] === "backfill" && rest.length === 2 && method === "POST") {
    const denied = need("events.backfill");
    if (denied) return denied;
    const result = await backfill(env.DB);
    await audit(env.DB, staff.who, staff.role, { action: "events.backfill", entity: "events", after: result });
    return json(result);
  }
  if (rest[0] === "learners") return learnersRoute(request, env, staff, rest.slice(1), url, need);
  if (rest[0] === "enrollment") return enrollmentRoute(request, env, staff, rest.slice(1), url, need);
  if (rest[0] === "audit" && rest.length === 3 && rest[2] === "undo" && method === "POST") return undo(request, env, staff, Number(rest[1]), url, need);
  return fail(404, "invalid_argument", "No such route.");
}

export const CODE_HEADER = "x-telosgard-code";

/** Step-up for sensitive actions: the current authenticator code (the localhost dev user is exempt). */
async function stepUp(request: Request, env: AuthEnv, staff: Staff, url: URL): Promise<Response | null> {
  if (staff.who.dev) return null;
  const code = request.headers.get(CODE_HEADER) ?? "";
  if (code && (await verifyCode(env, url.origin, staff.who.userId, code))) return null;
  return fail(403, "code_required", code ? "That code didn't work. Enter the current one." : "Enter your authenticator code to confirm.");
}

async function body(request: Request): Promise<Record<string, unknown>> {
  try {
    const b = await request.json();
    return b && typeof b === "object" && !Array.isArray(b) ? (b as Record<string, unknown>) : {};
  } catch {
    return {};
  }
}

/** Who staff may act on: never the Owner; Support only Learners. */
function targetBlock(staff: Staff, target: Account | null): Response | null {
  if (!target) return fail(404, "invalid_argument", "No such account.");
  if (target.isOwner) return fail(403, "not_granted", "The Owner's account can't be changed from the dashboard.");
  if (staff.role !== "owner" && target.role !== "learner") return fail(403, "not_granted", "Only the Owner can change a staff account.");
  if (target.id === staff.who.userId) return fail(403, "not_granted", "You can't do that to your own account.");
  return null;
}

type Need = (p: Permission) => Response | null;

async function learnersRoute(request: Request, env: AuthEnv, staff: Staff, rest: string[], url: URL, need: Need): Promise<Response> {
  const method = request.method;
  if (rest.length === 0 && method === "GET") return need("learners.read") ?? json(await listLearners(env));
  const id = rest[0];
  if (rest.length === 1 && method === "GET") {
    const denied = need("learners.read");
    if (denied) return denied;
    const p = await learnerProfile(env, id);
    return p ? json(p) : fail(404, "invalid_argument", "No such Learner.");
  }
  if (rest.length !== 2 || method !== "POST") return fail(404, "invalid_argument", "No such route.");
  const target = await getAccount(env, id);
  const b = await body(request);
  const log = (action: string, before: unknown, after: unknown) => audit(env.DB, staff.who, staff.role, { action, entity: "learner", entityId: id, before, after });

  switch (rest[1]) {
    case "suspend": {
      const denied = need("learners.suspend") ?? targetBlock(staff, target);
      if (denied) return denied;
      const on = b.on !== false;
      if (!on && target!.deleteAfter != null) return fail(409, "invalid_argument", "This account is scheduled for deletion. Cancel the deletion instead.");
      await setSuspended(env, url.origin, id, on);
      await log(on ? "learner.suspend" : "learner.unsuspend", { suspended: target!.suspendedAt != null }, { suspended: on });
      return json({ ok: true });
    }
    case "full-access": {
      const denied = need("fullaccess.set") ?? targetBlock(staff, target) ?? (await stepUp(request, env, staff, url));
      if (denied) return denied;
      const on = b.on === true;
      await setFullAccess(env, id, on);
      await log("learner.full_access", { fullAccess: target!.fullAccess }, { fullAccess: on });
      return json({ ok: true });
    }
    case "reset-lesson": {
      const denied = need("progress.reset") ?? targetBlock(staff, target);
      if (denied) return denied;
      const lesson = String(b.lesson ?? "");
      if (!/^lesson-[a-z0-9]{1,4}$/.test(lesson)) return fail(400, "invalid_argument", "Choose a lesson.");
      const removed = await resetLesson(env, id, lesson);
      await log("learner.reset_lesson", { lesson, docs: removed.docs }, { lesson, docs: [] });
      return json({ ok: true, removed: removed.docs.length });
    }
    case "reset-link": {
      const denied = need("passwords.reset") ?? targetBlock(staff, target);
      if (denied) return denied;
      const link = await createResetLink(env, url.origin, id, staff.who.userId);
      // The link itself is never logged: it works as a password until it's used.
      await log("learner.reset_link", null, { linkId: link.id, expiresAt: link.expiresAt });
      return json(link);
    }
    case "delete": {
      const denied = need("learners.delete") ?? targetBlock(staff, target) ?? (await stepUp(request, env, staff, url));
      if (denied) return denied;
      const after = await scheduleDeletion(env, url.origin, id);
      await log("learner.delete_scheduled", { deleteAfter: null, suspended: target!.suspendedAt != null }, { deleteAfter: after });
      return json({ deleteAfter: after });
    }
    case "cancel-delete": {
      const denied = need("learners.delete") ?? targetBlock(staff, target);
      if (denied) return denied;
      await cancelDeletion(env, id);
      await log("learner.delete_cancelled", { deleteAfter: target!.deleteAfter }, { deleteAfter: null });
      return json({ ok: true });
    }
  }
  return fail(404, "invalid_argument", "No such route.");
}

async function enrollmentRoute(request: Request, env: AuthEnv, staff: Staff, rest: string[], url: URL, need: Need): Promise<Response> {
  const denied = need("invites.manage");
  if (denied) return denied;
  const method = request.method;
  if (rest.length === 0 && method === "GET") return json(await listEnrollment(env.DB));
  if (rest[0] === "links" && rest.length === 1 && method === "POST") {
    const link = await createInviteLink(env.DB, staff.who.userId, url.origin);
    await audit(env.DB, staff.who, staff.role, { action: "invite_link.create", entity: "invite_link", entityId: link.id, after: { expiresAt: link.expiresAt } });
    return json(link);
  }
  if (rest[0] === "links" && rest.length === 2 && method === "DELETE") {
    const r = await env.DB.prepare("DELETE FROM invites WHERE id = ?1 AND used_at IS NULL RETURNING id").bind(rest[1]).first();
    if (r) await audit(env.DB, staff.who, staff.role, { action: "invite_link.delete", entity: "invite_link", entityId: rest[1] });
    return json({ ok: r !== null });
  }
  if (rest[0] === "codes" && rest.length === 1 && method === "POST") {
    const b = await body(request);
    const maxUses = Math.floor(Number(b.maxUses));
    const days = Math.floor(Number(b.days));
    if (!(maxUses >= 1 && maxUses <= 500)) return fail(400, "invalid_argument", "Uses must be between 1 and 500.");
    if (!(days >= 1 && days <= 90)) return fail(400, "invalid_argument", "Days must be between 1 and 90.");
    const made = await createInviteCode(env.DB, staff.who.userId, { code: typeof b.code === "string" && b.code.trim() ? b.code : undefined, maxUses, days });
    if ("error" in made) return fail(400, "invalid_argument", made.error ?? "Invalid code.");
    await audit(env.DB, staff.who, staff.role, { action: "invite_code.create", entity: "invite_code", entityId: made.row.id, after: made.row });
    return json(made.row);
  }
  if (rest[0] === "codes" && rest.length === 3 && (rest[2] === "revoke" || rest[2] === "restore") && method === "POST") {
    const revoke = rest[2] === "revoke";
    const r = await env.DB.prepare("UPDATE invite_codes SET revoked_at = ?1 WHERE id = ?2 RETURNING code").bind(revoke ? Date.now() : null, rest[1]).first<{ code: string }>();
    if (!r) return fail(404, "invalid_argument", "No such code.");
    await audit(env.DB, staff.who, staff.role, { action: revoke ? "invite_code.revoke" : "invite_code.restore", entity: "invite_code", entityId: rest[1], before: { revoked: !revoke }, after: { revoked: revoke, code: r.code } });
    return json({ ok: true });
  }
  return fail(404, "invalid_argument", "No such route.");
}

/** Which audit actions can be undone, and the permission an Undo needs (the original action's). */
const UNDOABLE: Record<string, Permission> = {
  "learner.suspend": "learners.suspend",
  "learner.unsuspend": "learners.suspend",
  "learner.full_access": "fullaccess.set",
  "learner.reset_lesson": "progress.reset",
  "learner.delete_scheduled": "learners.delete",
  "invite_code.revoke": "invites.manage",
};

export function undoable(action: string): boolean {
  return action in UNDOABLE;
}

async function undo(request: Request, env: AuthEnv, staff: Staff, id: number, url: URL, need: Need): Promise<Response> {
  const row = await env.DB.prepare("SELECT * FROM audit_log WHERE id = ?1").bind(id).first<Record<string, unknown>>();
  if (!row) return fail(404, "invalid_argument", "No such entry.");
  const action = String(row.action);
  const perm = UNDOABLE[action];
  if (!perm) return fail(400, "invalid_argument", "This change can't be undone.");
  const denied = need(perm);
  if (denied) return denied;
  const done = await env.DB.prepare("SELECT id FROM audit_log WHERE action = 'undo' AND entity = 'audit' AND entity_id = ?1").bind(String(id)).first();
  if (done) return fail(409, "invalid_argument", "This change was already undone.");
  const entityId = String(row.entity_id ?? "");
  const before = typeof row.before === "string" ? JSON.parse(row.before) : null;

  if (action.startsWith("learner.")) {
    const target = await getAccount(env, entityId);
    const blocked = targetBlock(staff, target);
    if (blocked) return blocked;
    if (action === "learner.full_access") {
      const code = await stepUp(request, env, staff, url);
      if (code) return code;
      await setFullAccess(env, entityId, Boolean(before?.fullAccess));
    } else if (action === "learner.suspend" || action === "learner.unsuspend") {
      if (target!.deleteAfter != null) return fail(409, "invalid_argument", "This account is scheduled for deletion. Cancel the deletion instead.");
      await setSuspended(env, url.origin, entityId, Boolean(before?.suspended));
    } else if (action === "learner.reset_lesson") {
      await restoreDocs(env, entityId, Array.isArray(before?.docs) ? before.docs : []);
    } else if (action === "learner.delete_scheduled") {
      await cancelDeletion(env, entityId);
      if (before?.suspended) await setSuspended(env, url.origin, entityId, true);
    }
  } else if (action === "invite_code.revoke") {
    await env.DB.prepare("UPDATE invite_codes SET revoked_at = NULL WHERE id = ?1").bind(entityId).run();
  }
  await audit(env.DB, staff.who, staff.role, { action: "undo", entity: "audit", entityId: String(id), before: { action }, after: null });
  return json({ ok: true });
}

/** Learner accounts: neither the Owner nor staff. Filtered here, not in SQL, to stay portable. */
async function learnerIds(db: D1Database, env: AuthEnv): Promise<Set<string>> {
  const owners = ownerEmails(env);
  const { results } = await db.prepare(`SELECT id, email FROM "user" WHERE role = 'learner'`).all<{ id: string; email: string }>();
  return new Set(results.filter((u) => !owners.includes(u.email.toLowerCase())).map((u) => u.id));
}

async function overview(db: D1Database, env: AuthEnv, now = Date.now()) {
  const learners = await learnerIds(db, env);
  const weekAgo = now - 7 * DAY;
  const { results } = await db
    .prepare("SELECT user_id, MAX(CASE WHEN type = 'signed_up' THEN 1 ELSE 0 END) AS joined, MAX(CASE WHEN type != 'signed_up' THEN 1 ELSE 0 END) AS active FROM events WHERE at >= ?1 GROUP BY user_id")
    .bind(weekAgo)
    .all<{ user_id: string; joined: number; active: number }>();
  const mine = results.filter((r) => learners.has(r.user_id));
  const today = await db.prepare("SELECT COUNT(*) AS n FROM events WHERE at >= ?1").bind(now - DAY).first<{ n: number }>();
  return {
    learners: learners.size,
    newThisWeek: mine.filter((r) => r.joined === 1).length,
    weeklyActive: mine.filter((r) => r.active === 1).length,
    eventsToday: today?.n ?? 0,
  };
}
