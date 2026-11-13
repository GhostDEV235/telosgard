// Shared by the admin tests: real Better Auth sessions (no DEV_USER), two-factor, invites.
import { env } from "cloudflare:test";
import { expect } from "vitest";
import { handle, type Env } from "../src/index";
import { INVITE_HEADER } from "../src/auth";

export const origin = "http://localhost";
export const e = () => ({ ...env, DEV_USER: undefined }) as unknown as Env;
export const noClaude = () => {
  throw new Error("no Claude in admin tests");
};
export const db = () => (env as unknown as { DB: D1Database }).DB;

let ip = Math.floor(Math.random() * 50000);
export async function call(method: string, path: string, body?: unknown, opts: { cookie?: string; invite?: string; code?: string } = {}) {
  ip++;
  const headers: Record<string, string> = { origin, "content-type": "application/json", "cf-connecting-ip": `10.9.${Math.floor(ip / 250)}.${ip % 250}` };
  if (opts.cookie) headers.cookie = opts.cookie;
  if (opts.invite) headers[INVITE_HEADER] = opts.invite;
  if (opts.code) headers["x-telosgard-code"] = opts.code;
  const res = await handle(new Request(origin + path, { method, headers, body: body === undefined ? undefined : JSON.stringify(body) }), e(), noClaude);
  const cookie = (res.headers.getSetCookie?.() ?? []).map((c) => c.split(";")[0]).join("; ");
  let data: unknown = null;
  try {
    data = await res.json();
  } catch {}
  return { status: res.status, data: data as Record<string, unknown>, cookie };
}

export const PASSWORD = "correct horse battery";
export const signUp = (email: string, invite?: string) => call("POST", "/api/auth/sign-up/email", { name: email.split("@")[0], email, password: PASSWORD }, { invite });
export const signIn = (email: string) => call("POST", "/api/auth/sign-in/email", { email, password: PASSWORD });

/** RFC 6238 TOTP (SHA-1, 6 digits, 30 s) from a base32 secret, as an authenticator app computes it. */
export async function totp(secret: string, at = Date.now()): Promise<string> {
  const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
  let bits = "";
  for (const ch of secret.replace(/=+$/, "").toUpperCase()) bits += alphabet.indexOf(ch).toString(2).padStart(5, "0");
  const key = new Uint8Array(Math.floor(bits.length / 8)).map((_, i) => parseInt(bits.slice(i * 8, i * 8 + 8), 2));
  const counter = Math.floor(at / 30000);
  const msg = new Uint8Array(8);
  new DataView(msg.buffer).setUint32(4, counter);
  const k = await crypto.subtle.importKey("raw", key, { name: "HMAC", hash: "SHA-1" }, false, ["sign"]);
  const h = new Uint8Array(await crypto.subtle.sign("HMAC", k, msg));
  const o = h[19] & 0xf;
  const n = ((h[o] & 0x7f) << 24) | (h[o + 1] << 16) | (h[o + 2] << 8) | h[o + 3];
  return String(n % 1_000_000).padStart(6, "0");
}

export async function invite(ownerCookie: string): Promise<string> {
  const r = await call("POST", "/api/invites", undefined, { cookie: ownerCookie });
  return String(r.data.url).split("#join-")[1];
}

/** Signs up, turns on two-factor, then signs in again through the code step. Returns the secret and a fresh cookie. */
export async function staffWithTwoFactor(email: string, inviteToken?: string) {
  const first = await signUp(email, inviteToken);
  expect(first.status).toBe(200);
  const enable = await call("POST", "/api/auth/two-factor/enable", { password: PASSWORD }, { cookie: first.cookie });
  expect(enable.status).toBe(200);
  const secret = new URL(String(enable.data.totpURI)).searchParams.get("secret")!;
  expect((enable.data.backupCodes as string[]).length).toBe(10);
  const verified = await call("POST", "/api/auth/two-factor/verify-totp", { code: await totp(secret) }, { cookie: first.cookie });
  expect(verified.status).toBe(200);

  const again = await signIn(email);
  expect(again.data).toMatchObject({ twoFactorRedirect: true });
  const done = await call("POST", "/api/auth/two-factor/verify-totp", { code: await totp(secret) }, { cookie: again.cookie });
  expect(done.status).toBe(200);
  return { secret, cookie: done.cookie };
}

