// The audit log (ADR 0006): every staff write records who did it, what, to which record, and the
// values before and after, so it can be reviewed and, where the action allows, undone.

import type { Identity } from "./auth";
import type { Role } from "./roles";

export interface AuditEntry {
  action: string;
  entity: string;
  entityId?: string | null;
  before?: unknown;
  after?: unknown;
}

export async function audit(db: D1Database, who: Identity, role: Role, e: AuditEntry, at = Date.now()): Promise<void> {
  await db
    .prepare("INSERT INTO audit_log (actor_id, actor_email, actor_role, action, entity, entity_id, before, after, at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)")
    .bind(who.userId, who.email ?? null, role, e.action, e.entity, e.entityId ?? null, e.before === undefined ? null : JSON.stringify(e.before), e.after === undefined ? null : JSON.stringify(e.after), at)
    .run();
}

export interface AuditRow {
  id: number;
  actorId: string;
  actorEmail: string | null;
  actorRole: string;
  action: string;
  entity: string;
  entityId: string | null;
  before: unknown;
  after: unknown;
  at: number;
}

/** Newest first, in pages: pass the smallest id seen as `beforeId` for the next page. */
export async function auditPage(db: D1Database, limit: number, beforeId?: number): Promise<AuditRow[]> {
  const { results } = await db
    .prepare("SELECT * FROM audit_log WHERE (?1 IS NULL OR id < ?1) ORDER BY id DESC LIMIT ?2")
    .bind(beforeId ?? null, limit)
    .all<Record<string, unknown>>();
  const parse = (v: unknown) => (typeof v === "string" ? JSON.parse(v) : null);
  return results.map((r) => ({
    id: Number(r.id),
    actorId: String(r.actor_id),
    actorEmail: (r.actor_email as string | null) ?? null,
    actorRole: String(r.actor_role),
    action: String(r.action),
    entity: String(r.entity),
    entityId: (r.entity_id as string | null) ?? null,
    before: parse(r.before),
    after: parse(r.after),
    at: Number(r.at),
  }));
}
