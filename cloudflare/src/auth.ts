// Who is asking (ADR 0004): Better Auth accounts (email + password) stored in D1, with sessions in a
// same-origin cookie. Sign-up needs a single-use invite link from the owner; an address in the
// OWNER_EMAILS secret may sign up without one (that's how the owner's own account is created).
// For `wrangler dev` and tests, DEV_USER stands in, but only for requests addressed to localhost.

import { betterAuth } from "better-auth";
import { APIError, createAuthMiddleware } from "better-auth/api";
import { twoFactor } from "better-auth/plugins";
import { recordEvents } from "./events";
import { storedRole, type Role } from "./roles";

export interface AuthEnv {
  DB: D1Database;
  BETTER_AUTH_SECRET?: string;
  OWNER_EMAILS?: string;
  /** Secret: addresses that may open every lesson without passing the ones before it (reviewers). */
  FULL_ACCESS_EMAILS?: string;
  DEV_USER?: string;
}

export interface Identity {
  userId: string;
  email?: string;
  /** The localhost-only DEV_USER (wrangler dev and tests). */
  dev?: boolean;
  /** Stored role (instructor, support or learner); see `roleOf` for the effective one. */
  stored?: Role;
  twoFactorEnabled?: boolean;
  /** When this session was signed in (ms). */
  sessionStart?: number;
}

export const INVITE_HEADER = "x-telosgard-invite";
export const INVITE_DAYS = 7;
const MIN_PASSWORD = 10;

export function isLocalhost(url: URL): boolean {
  return url.hostname === "localhost" || url.hostname === "127.0.0.1" || url.hostname === "[::1]";
}

function emailList(raw: string | undefined): string[] {
  return (raw ?? "").split(",").map((e) => e.trim().toLowerCase()).filter(Boolean);
}

export function ownerEmails(env: { OWNER_EMAILS?: string }): string[] {
  return emailList(env.OWNER_EMAILS);
}

/** Every lesson open, regardless of progress. Only the course gate changes: no owner powers. */
export function hasFullAccess(env: { FULL_ACCESS_EMAILS?: string }, who: Identity): boolean {
  return who.email !== undefined && emailList(env.FULL_ACCESS_EMAILS).includes(who.email.toLowerCase());
}

export function isOwner(env: { OWNER_EMAILS?: string }, who: Identity): boolean {
  return who.dev === true || (who.email !== undefined && ownerEmails(env).includes(who.email.toLowerCase()));
}

/** The effective role: Owner by secret (or the localhost dev user), otherwise the stored role. */
export function roleOf(env: { OWNER_EMAILS?: string }, who: Identity): Role {
  return isOwner(env, who) ? "owner" : (who.stored ?? "learner");
}

export async function sha256(text: string): Promise<string> {
  const bytes = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(text));
  return [...new Uint8Array(bytes)].map((b) => b.toString(16).padStart(2, "0")).join("");
}

/** An invite token is usable when it exists, is unused and hasn't expired. */
export async function inviteUsable(db: D1Database, token: string, now = Date.now()): Promise<boolean> {
  if (!/^[A-Za-z0-9_-]{20,100}$/.test(token)) return false;
  const row = await db.prepare("SELECT id FROM invites WHERE token_hash = ?1 AND used_at IS NULL AND expires_at > ?2").bind(await sha256(token), now).first();
  return row !== null;
}

/** One Better Auth instance per request (bindings are per request on Workers). */
export function makeAuth(env: AuthEnv, origin: string) {
  if (!env.BETTER_AUTH_SECRET) throw new Error("BETTER_AUTH_SECRET is not set");
  return betterAuth({
    appName: "Telosgard",
    baseURL: origin,
    basePath: "/api/auth",
    secret: env.BETTER_AUTH_SECRET,
    database: env.DB,
    trustedOrigins: [origin],
    telemetry: { enabled: false },
    emailAndPassword: { enabled: true, minPasswordLength: MIN_PASSWORD, autoSignIn: true },
    rateLimit: { enabled: true, storage: "database" },
    advanced: { ipAddress: { ipAddressHeaders: ["cf-connecting-ip"] } },
    // Staff roles (ADR 0007): set only by the Owner through the admin API, never by sign-up.
    user: { additionalFields: { role: { type: "string", required: false, defaultValue: "learner", input: false } } },
    // Two-factor for staff (ADR 0007): TOTP with 10 backup codes. Learners never need it.
    plugins: [twoFactor({ issuer: "Telosgard", backupCodeOptions: { amount: 10 } })],
    hooks: {
      // Sign-up is invite-only: a valid invite token in the header, or an owner address.
      before: createAuthMiddleware(async (ctx) => {
        if (ctx.path !== "/sign-up/email") return;
        const email = String((ctx.body as { email?: unknown } | undefined)?.email ?? "").trim().toLowerCase();
        if (ownerEmails(env).includes(email)) return;
        const token = ctx.headers?.get(INVITE_HEADER) ?? "";
        if (!(await inviteUsable(env.DB, token))) {
          throw new APIError("FORBIDDEN", { message: "Telosgard is invite-only. Ask for an invite link." });
        }
      }),
    },
    databaseHooks: {
      user: {
        create: {
          // The invite is spent once the account exists, so a failed sign-up doesn't burn it.
          after: async (user, ctx) => {
            const token = ctx?.headers?.get(INVITE_HEADER);
            let invite: string | undefined;
            if (token) {
              const row = await env.DB.prepare("UPDATE invites SET used_by = ?1, used_at = ?2 WHERE token_hash = ?3 AND used_at IS NULL RETURNING id")
                .bind(user.id, Date.now(), await sha256(token))
                .first<{ id: string }>();
              invite = row?.id;
            }
            await recordEvents(env.DB, user.id, [{ type: "signed_up", data: invite ? { invite } : {}, at: Date.now() }]);
          },
        },
      },
    },
  });
}

export async function identify(request: Request, env: AuthEnv): Promise<Identity | null> {
  const url = new URL(request.url);
  if (env.DEV_USER && isLocalhost(url)) return { userId: env.DEV_USER, dev: true };
  if (!env.BETTER_AUTH_SECRET) return null;
  const session = await makeAuth(env, url.origin).api.getSession({ headers: request.headers });
  if (!session) return null;
  const u = session.user as typeof session.user & { role?: unknown; twoFactorEnabled?: unknown };
  return {
    userId: u.id,
    email: u.email,
    stored: storedRole(u.role),
    twoFactorEnabled: Boolean(u.twoFactorEnabled),
    sessionStart: new Date(session.session.createdAt).getTime(),
  };
}
