// Settings (D3, FR-17.7): live Claude limits, Maintenance Mode, the versioned Disclaimer, and the
// status of keys and integrations (never the keys themselves). Values live in the `settings` and
// `disclaimers` tables (migration 0007); a missing setting means its default.

import { emailList, type AuthEnv } from "./auth";

export interface SettingsEnv extends AuthEnv {
  ANTHROPIC_API_KEY?: string;
  FILES?: R2Bucket;
  /** Defaults for the limits until the Owner first saves them in the dashboard. */
  CLAUDE_DAILY_LIMIT?: string;
  TESTER_DAILY_LIMIT?: string;
}

/** Claude calls per 24 hours: the Owner, every other account, and all accounts together (null: no cap). */
export interface ClaudeLimits {
  owner: number;
  learner: number;
  total: number | null;
}

export interface Maintenance {
  on: boolean;
  messageEn: string;
  messageFr: string;
  since: number | null;
}

export interface Disclaimer {
  version: number;
  en: string;
  fr: string;
  publishedAt: number;
}

export const LIMIT_MAX = { owner: 5000, learner: 1000, total: 100000 };
export const MESSAGE_MAX = 400;
export const DISCLAIMER_MAX = 4000;

async function read<T>(db: D1Database, key: string): Promise<T | null> {
  const r = await db.prepare("SELECT value FROM settings WHERE key = ?1").bind(key).first<{ value: string }>();
  if (!r) return null;
  try {
    return JSON.parse(r.value) as T;
  } catch {
    return null;
  }
}

export async function writeSetting(db: D1Database, key: string, value: unknown, by: string | null, now = Date.now()): Promise<void> {
  await db
    .prepare("INSERT INTO settings (key, value, updated_at, updated_by) VALUES (?1, ?2, ?3, ?4) ON CONFLICT (key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at, updated_by = excluded.updated_by")
    .bind(key, JSON.stringify(value), now, by)
    .run();
}

const whole = (v: unknown, fallback: number) => {
  const n = Number(v);
  return Number.isFinite(n) && n >= 0 ? Math.floor(n) : fallback;
};

export async function claudeLimits(env: SettingsEnv): Promise<ClaudeLimits> {
  const stored = await read<Partial<ClaudeLimits>>(env.DB, "claude_limits");
  const owner = whole(stored?.owner, whole(env.CLAUDE_DAILY_LIMIT, 300) || 300);
  const learner = whole(stored?.learner, whole(env.TESTER_DAILY_LIMIT, 50));
  const total = stored?.total == null ? null : whole(stored.total, 0);
  return { owner, learner, total };
}

/** Validates limits from the dashboard; returns an error message or the clean value. */
export function parseLimits(b: Record<string, unknown>): ClaudeLimits | string {
  const owner = Number(b.owner);
  const learner = Number(b.learner);
  const total = b.total === null || b.total === "" || b.total === undefined ? null : Number(b.total);
  if (!Number.isInteger(owner) || owner < 1 || owner > LIMIT_MAX.owner) return `The Owner's limit must be a whole number from 1 to ${LIMIT_MAX.owner}.`;
  if (!Number.isInteger(learner) || learner < 0 || learner > LIMIT_MAX.learner) return `The Learner limit must be a whole number from 0 to ${LIMIT_MAX.learner}.`;
  if (total !== null && (!Number.isInteger(total) || total < 1 || total > LIMIT_MAX.total)) return `The overall limit must be empty or a whole number from 1 to ${LIMIT_MAX.total}.`;
  return { owner, learner, total };
}

const OFF: Maintenance = { on: false, messageEn: "", messageFr: "", since: null };

export async function maintenance(db: D1Database): Promise<Maintenance> {
  const m = await read<Partial<Maintenance>>(db, "maintenance");
  if (!m) return OFF;
  return { on: m.on === true, messageEn: String(m.messageEn ?? ""), messageFr: String(m.messageFr ?? ""), since: typeof m.since === "number" ? m.since : null };
}

export function parseMaintenance(b: Record<string, unknown>, prev: Maintenance, now = Date.now()): Maintenance | string {
  const messageEn = String(b.messageEn ?? "").trim();
  const messageFr = String(b.messageFr ?? "").trim();
  if (messageEn.length > MESSAGE_MAX || messageFr.length > MESSAGE_MAX) return `Keep each message under ${MESSAGE_MAX} characters.`;
  const on = b.on === true;
  return { on, messageEn, messageFr, since: on ? (prev.on ? prev.since : now) : null };
}

export async function currentDisclaimer(db: D1Database): Promise<Disclaimer | null> {
  const r = await db
    .prepare("SELECT version, text_en, text_fr, published_at FROM disclaimers ORDER BY version DESC LIMIT 1")
    .first<{ version: number; text_en: string; text_fr: string; published_at: number }>();
  return r ? { version: r.version, en: r.text_en, fr: r.text_fr, publishedAt: r.published_at } : null;
}

export async function disclaimerHistory(db: D1Database): Promise<{ version: number; publishedAt: number; publishedBy: string | null }[]> {
  const { results } = await db
    .prepare('SELECT d.version, d.published_at, u.email FROM disclaimers d LEFT JOIN "user" u ON u.id = d.published_by ORDER BY d.version DESC LIMIT 20')
    .all<{ version: number; published_at: number; email: string | null }>();
  return results.map((r) => ({ version: r.version, publishedAt: r.published_at, publishedBy: r.email }));
}

/** Publishes a new version; every Learner who accepted an older one is asked again. */
export async function publishDisclaimer(db: D1Database, en: string, fr: string, by: string, now = Date.now()): Promise<Disclaimer | string> {
  const a = en.trim();
  const b = fr.trim();
  if (!a || !b) return "Write the disclaimer in both English and French.";
  if (a.length > DISCLAIMER_MAX || b.length > DISCLAIMER_MAX) return `Keep each language under ${DISCLAIMER_MAX} characters.`;
  const cur = await currentDisclaimer(db);
  if (cur && cur.en === a && cur.fr === b) return "Nothing changed: this is already the current version.";
  const version = (cur?.version ?? 0) + 1;
  await db.prepare("INSERT INTO disclaimers (version, text_en, text_fr, published_at, published_by) VALUES (?1, ?2, ?3, ?4, ?5)").bind(version, a, b, now, by).run();
  return { version, en: a, fr: b, publishedAt: now };
}

export interface StatusItem {
  name: string;
  state: "ok" | "warn" | "missing";
  detail: string;
}

/** Keys and integrations, by presence and recent results only: no secret value ever leaves the Worker. */
export async function keyStatus(env: SettingsEnv, now = Date.now()): Promise<StatusItem[]> {
  const items: StatusItem[] = [];
  const since = now - 24 * 3600 * 1000;
  const calls = await env.DB.prepare("SELECT SUM(ok) AS good, COUNT(*) - SUM(ok) AS bad, MAX(CASE WHEN ok = 1 THEN at END) AS last FROM claude_calls WHERE at > ?1")
    .bind(since)
    .first<{ good: number | null; bad: number | null; last: number | null }>();
  const lastOk = await env.DB.prepare("SELECT MAX(at) AS at FROM claude_calls WHERE ok = 1").first<{ at: number | null }>();
  if (!env.ANTHROPIC_API_KEY) {
    items.push({ name: "Anthropic API key", state: "missing", detail: "Not set. Professor Aurel can't grade or answer until it is (wrangler secret put ANTHROPIC_API_KEY)." });
  } else {
    const bad = Number(calls?.bad ?? 0);
    const good = Number(calls?.good ?? 0);
    const last = lastOk?.at ? `last answered ${ago(now - lastOk.at)}` : "no successful call yet";
    items.push({
      name: "Anthropic API key",
      state: bad > 0 && good === 0 ? "warn" : "ok",
      detail: `Set; ${last}. Last 24 hours: ${good} answered, ${bad} failed.`,
    });
  }
  items.push(
    env.BETTER_AUTH_SECRET
      ? { name: "Sign-in secret", state: "ok", detail: "Set." }
      : { name: "Sign-in secret", state: "missing", detail: "Not set: nobody can sign in (wrangler secret put BETTER_AUTH_SECRET)." },
  );
  const owners = emailList(env.OWNER_EMAILS).length;
  items.push(owners ? { name: "Owner address", state: "ok", detail: `${owners} address${owners === 1 ? "" : "es"} set.` } : { name: "Owner address", state: "missing", detail: "Not set: there is no Owner (wrangler secret put OWNER_EMAILS)." });
  const reviewers = emailList(env.FULL_ACCESS_EMAILS).length;
  if (reviewers) {
    items.push({ name: "FULL_ACCESS_EMAILS", state: "warn", detail: `Still set (${reviewers} address${reviewers === 1 ? "" : "es"}). Turn Full Access on for them in Learners, then delete the secret (wrangler secret delete FULL_ACCESS_EMAILS).` });
  }
  items.push({ name: "Database (D1)", state: "ok", detail: "Connected." });
  items.push(env.FILES ? { name: "Screenshot storage (R2)", state: "ok", detail: "Connected." } : { name: "Screenshot storage (R2)", state: "missing", detail: "Not bound: journal screenshots can't be saved." });
  const cron = await read<{ at: number; erased: number }>(env.DB, "cron_last_run");
  if (!cron) items.push({ name: "Daily clean-up", state: "warn", detail: "Hasn't run yet. It runs at 03:17 UTC and erases accounts whose 7-day deletion grace has ended." });
  else items.push({ name: "Daily clean-up", state: now - cron.at > 36 * 3600 * 1000 ? "warn" : "ok", detail: `Last ran ${ago(now - cron.at)}; erased ${cron.erased} account${cron.erased === 1 ? "" : "s"}.` });
  return items;
}

function ago(ms: number): string {
  const m = Math.max(0, Math.round(ms / 60000));
  if (m < 2) return "just now";
  if (m < 60) return `${m} min ago`;
  if (m < 48 * 60) return `${Math.round(m / 60)} h ago`;
  return `${Math.round(m / 1440)} days ago`;
}
