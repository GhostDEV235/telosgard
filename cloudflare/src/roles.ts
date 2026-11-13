// Roles and permissions (ADR 0007, CONTEXT.md). The Owner comes only from the OWNER_EMAILS secret;
// Instructor and Support are the user's `role` column. Every admin route asks for a permission by
// name; the dashboard only hides what this already refuses.

export type Role = "owner" | "instructor" | "support" | "learner";
export const STAFF_ROLES: readonly Role[] = ["owner", "instructor", "support"];
/** Roles the Owner may assign from the dashboard. Owner is never one of them. */
export const ASSIGNABLE: readonly Role[] = ["instructor", "support", "learner"];

export type Permission =
  | "overview.read"
  | "audit.read"
  | "events.backfill"
  | "learners.read"
  | "learners.suspend"
  | "learners.delete"
  | "progress.reset"
  | "invites.manage"
  | "passwords.reset"
  | "fullaccess.set"
  | "roles.set"
  | "journals.read"
  | "lessons.read"
  | "lessons.hide"
  | "lessons.notes"
  | "answers.read"
  | "settings.read"
  | "settings.write";

const GRANTS: Record<Permission, readonly Role[]> = {
  "overview.read": ["owner", "instructor", "support"],
  "audit.read": ["owner"],
  "events.backfill": ["owner"],
  "learners.read": ["owner", "support"],
  "learners.suspend": ["owner", "support"],
  "learners.delete": ["owner"],
  "progress.reset": ["owner", "support"],
  "invites.manage": ["owner", "support"],
  "passwords.reset": ["owner", "support"],
  "fullaccess.set": ["owner"],
  "roles.set": ["owner"],
  "journals.read": ["owner"],
  "lessons.read": ["owner", "instructor", "support"],
  "lessons.hide": ["owner", "instructor"],
  "lessons.notes": ["owner", "instructor"],
  "answers.read": ["owner", "instructor"],
  "settings.read": ["owner"],
  "settings.write": ["owner"],
};

export function can(role: Role, p: Permission): boolean {
  return GRANTS[p].includes(role);
}

export function permissionsOf(role: Role): Permission[] {
  return (Object.keys(GRANTS) as Permission[]).filter((p) => can(role, p));
}

export function isStaff(role: Role): boolean {
  return STAFF_ROLES.includes(role);
}

/** A stored role value, with anything unknown (or "owner", which is never stored) read as learner. */
export function storedRole(v: unknown): Role {
  return v === "instructor" || v === "support" ? v : "learner";
}

/** Dashboard sessions last 12 hours from sign-in (ADR 0007). */
export const STAFF_SESSION_MS = 12 * 60 * 60 * 1000;
