// Who is asking. In production Cloudflare Access signs every request with a JWT
// (`Cf-Access-Jwt-Assertion`); we verify it against the team's public keys, the audience tag and
// the issuer, and use its `sub` as the user id. For `wrangler dev`, DEV_USER stands in, but only
// for requests addressed to localhost, so it can never open the deployed site.

import { createRemoteJWKSet, jwtVerify, type JWTVerifyGetKey } from "jose";

export interface AuthEnv {
  ACCESS_TEAM_DOMAIN?: string;
  ACCESS_AUD?: string;
  DEV_USER?: string;
}

export interface Identity {
  userId: string;
  email?: string;
}

const keySets = new Map<string, JWTVerifyGetKey>();

function teamKeys(team: string): JWTVerifyGetKey {
  let keys = keySets.get(team);
  if (!keys) {
    keys = createRemoteJWKSet(new URL(`${team}/cdn-cgi/access/certs`));
    keySets.set(team, keys);
  }
  return keys;
}

export function isLocalhost(url: URL): boolean {
  return url.hostname === "localhost" || url.hostname === "127.0.0.1" || url.hostname === "[::1]";
}

/** Verifies an Access JWT. `keys` is injectable for tests. */
export async function verifyAccessJwt(token: string, team: string, aud: string, keys?: JWTVerifyGetKey): Promise<Identity | null> {
  try {
    const { payload } = await jwtVerify(token, keys ?? teamKeys(team), { issuer: team, audience: aud });
    if (typeof payload.sub !== "string" || payload.sub.length === 0) return null;
    const email = typeof payload.email === "string" ? payload.email : undefined;
    return { userId: payload.sub, email };
  } catch {
    return null;
  }
}

export async function identify(request: Request, env: AuthEnv, keys?: JWTVerifyGetKey): Promise<Identity | null> {
  const url = new URL(request.url);
  const token = request.headers.get("Cf-Access-Jwt-Assertion");
  const team = env.ACCESS_TEAM_DOMAIN?.replace(/\/+$/, "");
  if (token && team && env.ACCESS_AUD) {
    return verifyAccessJwt(token, team, env.ACCESS_AUD, keys);
  }
  if (env.DEV_USER && isLocalhost(url)) {
    return { userId: env.DEV_USER };
  }
  return null;
}
