// Enrollment (CONTEXT.md): single-use Invite Links and shareable Invite Codes. Sign-up sends either
// one in the x-telosgard-invite header; links are long random tokens, codes are short words.

import { sha256 } from "./auth-util";

export const INVITE_DAYS = 7;
const DAY = 24 * 3600 * 1000;
const TOKEN = /^[A-Za-z0-9_-]{20,100}$/;
const CODE = /^[A-Z0-9][A-Z0-9-]{2,23}$/;

export function normalizeCode(raw: string): string {
  return raw.trim().toUpperCase().replace(/\s+/g, "-");
}

export function validCode(code: string): boolean {
  return CODE.test(code);
}

function randomToken(): string {
  const bytes = crypto.getRandomValues(new Uint8Array(24));
  return btoa(String.fromCharCode(...bytes)).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

/** A readable code such as GOLD-7KQ2 (no 0/O or 1/I). */
export function randomCode(prefix = "GOLD"): string {
  const alphabet = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
  const bytes = crypto.getRandomValues(new Uint8Array(4));
  return `${prefix}-${[...bytes].map((b) => alphabet[b % alphabet.length]).join("")}`;
}

export async function createInviteLink(db: D1Database, createdBy: string, origin: string, now = Date.now()) {
  const token = randomToken();
  const id = crypto.randomUUID();
  const expiresAt = now + INVITE_DAYS * DAY;
  await db.prepare("INSERT INTO invites (id, token_hash, created_by, created_at, expires_at) VALUES (?1, ?2, ?3, ?4, ?5)").bind(id, await sha256(token), createdBy, now, expiresAt).run();
  return { id, url: `${origin}/#join-${token}`, expiresAt };
}

export type Invite = { kind: "link"; id: string } | { kind: "code"; id: string };

/** Which invite this header value opens, if it is usable right now. */
export async function usableInvite(db: D1Database, raw: string, now = Date.now()): Promise<Invite | null> {
  if (TOKEN.test(raw)) {
    const row = await db.prepare("SELECT id FROM invites WHERE token_hash = ?1 AND used_at IS NULL AND expires_at > ?2").bind(await sha256(raw), now).first<{ id: string }>();
    if (row) return { kind: "link", id: row.id };
  }
  const code = normalizeCode(raw);
  if (validCode(code)) {
    const row = await db
      .prepare("SELECT id FROM invite_codes WHERE code = ?1 AND revoked_at IS NULL AND expires_at > ?2 AND uses < max_uses")
      .bind(code, now)
      .first<{ id: string }>();
    if (row) return { kind: "code", id: row.id };
  }
  return null;
}

/** Spends the invite once the account exists, and returns where the account came from. */
export async function spendInvite(db: D1Database, raw: string, userId: string, now = Date.now()): Promise<string | null> {
  if (TOKEN.test(raw)) {
    const row = await db
      .prepare("UPDATE invites SET used_by = ?1, used_at = ?2 WHERE token_hash = ?3 AND used_at IS NULL RETURNING id")
      .bind(userId, now, await sha256(raw))
      .first<{ id: string }>();
    if (row) return `link:${row.id}`;
  }
  const code = normalizeCode(raw);
  if (validCode(code)) {
    const row = await db.prepare("UPDATE invite_codes SET uses = uses + 1 WHERE code = ?1 AND revoked_at IS NULL RETURNING id").bind(code).first<{ id: string }>();
    if (row) return `code:${row.id}`;
  }
  return null;
}

export async function createInviteCode(db: D1Database, createdBy: string, opts: { code?: string; maxUses: number; days: number }, now = Date.now()) {
  const code = opts.code ? normalizeCode(opts.code) : randomCode();
  if (!validCode(code)) return { error: "A code is 3 to 24 letters, digits or dashes, starting with a letter or digit." } as const;
  const taken = await db.prepare("SELECT id FROM invite_codes WHERE code = ?1").bind(code).first();
  if (taken) return { error: "That code already exists. Choose another." } as const;
  const id = crypto.randomUUID();
  const row = { id, code, createdAt: now, expiresAt: now + opts.days * DAY, maxUses: opts.maxUses, uses: 0, revokedAt: null as number | null };
  await db
    .prepare("INSERT INTO invite_codes (id, code, created_by, created_at, expires_at, max_uses) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")
    .bind(id, code, createdBy, now, row.expiresAt, opts.maxUses)
    .run();
  return { row } as const;
}

export async function listEnrollment(db: D1Database) {
  const links = await db
    .prepare(
      `SELECT i.id, i.created_at, i.expires_at, i.used_at, u.email AS used_by_email, c.email AS created_by_email
       FROM invites i LEFT JOIN "user" u ON u.id = i.used_by LEFT JOIN "user" c ON c.id = i.created_by
       ORDER BY i.created_at DESC LIMIT 100`,
    )
    .all<Record<string, unknown>>();
  const codes = await db.prepare("SELECT * FROM invite_codes ORDER BY created_at DESC LIMIT 100").all<Record<string, unknown>>();
  return {
    links: links.results.map((r) => ({
      id: String(r.id),
      createdAt: Number(r.created_at),
      expiresAt: Number(r.expires_at),
      usedAt: r.used_at == null ? null : Number(r.used_at),
      usedBy: (r.used_by_email as string | null) ?? null,
      createdBy: (r.created_by_email as string | null) ?? null,
    })),
    codes: codes.results.map((r) => ({
      id: String(r.id),
      code: String(r.code),
      createdAt: Number(r.created_at),
      expiresAt: Number(r.expires_at),
      maxUses: Number(r.max_uses),
      uses: Number(r.uses),
      revokedAt: r.revoked_at == null ? null : Number(r.revoked_at),
    })),
  };
}
