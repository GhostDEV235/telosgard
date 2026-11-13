// The admin API, /api/admin/* (ADR 0006, 0007). Staff only: Owner (by secret), Instructor or Support,
// with two-factor turned on and a session signed in within the last 12 hours. Each route names the
// permission it needs (roles.ts); every write goes to the audit log.
//
//   GET  /api/admin/me              → { role, email, twoFactorEnabled, ready, permissions, sessionEndsAt }
//                                     (answers before two-factor is set up, so the dashboard can ask for it)
//   GET  /api/admin/overview        → { learners, newThisWeek, weeklyActive, eventsToday }
//   GET  /api/admin/audit?before=&limit=  → AuditRow[] (newest first)
//   POST /api/admin/events/backfill → { users, events }

import { audit, auditPage } from "./audit";
import { ownerEmails, roleOf, type AuthEnv, type Identity } from "./auth";
import { backfill } from "./events";
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
    return json(await auditPage(env.DB, limit, Number.isFinite(before) && before > 0 ? before : undefined));
  }
  if (rest[0] === "events" && rest[1] === "backfill" && rest.length === 2 && method === "POST") {
    const denied = need("events.backfill");
    if (denied) return denied;
    const result = await backfill(env.DB);
    await audit(env.DB, staff.who, staff.role, { action: "events.backfill", entity: "events", after: result });
    return json(result);
  }
  return fail(404, "invalid_argument", "No such route.");
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
