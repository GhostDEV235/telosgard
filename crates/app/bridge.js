// Bridge between Rust and Telosgard's Cloudflare Worker (ARCHITECTURE §4, ADR 0003).
// Same exports as the claude.ai Artifact bridge it replaced, so the Rust side is unchanged.
// Every export resolves to a JSON string: {"ok":true,"value":…} or {"ok":false,"code":…,"message":…}.
// It never rejects, so Rust only has to parse one shape. Sign-in is Cloudflare Access: its cookie
// rides along on every same-origin request.

const TIMEOUT_MS = 12000;
const CLAUDE_TIMEOUT_MS = 90000;
let initPromise = null;

const ok = (value) => JSON.stringify({ ok: true, value: value === undefined ? null : value });
const fail = (code, message) => JSON.stringify({ ok: false, code: code || "unavailable", message: String(message || "") });
const errOf = (e) => fail(e && e.code, e && e.message ? e.message : e);

/** One API call. Resolves to the parsed JSON body, or throws {code, message}. */
async function api(method, path, body, { timeout = TIMEOUT_MS, contentType = "application/json", headers: extra = {} } = {}) {
  const ctrl = new AbortController();
  const t = setTimeout(() => ctrl.abort(), timeout);
  let res;
  try {
    res = await fetch(path, {
      method,
      body,
      headers: { ...(body === undefined ? {} : { "content-type": contentType }), ...extra },
      credentials: "same-origin",
      signal: ctrl.signal,
    });
  } catch (e) {
    throw { code: "unavailable", message: ctrl.signal.aborted ? "timed out" : "network error" };
  } finally {
    clearTimeout(t);
  }
  let data = null;
  try {
    data = await res.json();
  } catch (e) {
    data = null;
  }
  if (!res.ok) {
    // Access may answer an expired session with a redirect page instead of JSON.
    const code = (data && data.code) || (res.status === 401 || res.status === 403 ? "not_granted" : res.status === 429 ? "rate_limited" : "unavailable");
    throw { code, message: (data && data.message) || `HTTP ${res.status}` };
  }
  return data;
}

// One retry after a short random delay for transient failures.
async function call(fn) {
  try {
    return await fn();
  } catch (e) {
    if (e && e.code === "unavailable") {
      await new Promise((r) => setTimeout(r, 300 + Math.random() * 600));
      return await fn();
    }
    throw e;
  }
}

const docPath = (id) => `/api/docs/${encodeURIComponent(id)}`;

export function init() {
  if (!initPromise) {
    initPromise = (async () => {
      try {
        const me = await call(() => api("GET", "/api/me"));
        const signedIn = !!(me && me.signedIn);
        return ok({ db: signedIn, user: true, sample: signedIn, signedIn, email: (me && me.email) || null, owner: !!(me && me.isOwner), fullAccess: !!(me && me.fullAccess) });
      } catch (e) {
        return errOf(e);
      }
    })();
  }
  return initPromise;
}

export async function dbGet(docId) {
  try {
    return ok(await call(() => api("GET", docPath(docId))));
  } catch (e) { return errOf(e); }
}

export async function dbSet(docId, json) {
  try {
    await call(() => api("PUT", docPath(docId), json));
    return ok(null);
  } catch (e) { return errOf(e); }
}

export async function dbDelete(docId) {
  try {
    await call(() => api("DELETE", docPath(docId)));
    return ok(null);
  } catch (e) { return errOf(e); }
}

/// All of the Learner's documents of one kind: [{id, data}].
export async function dbList(kind) {
  try {
    return ok(await call(() => api("GET", `/api/docs?kind=${encodeURIComponent(kind)}`)));
  } catch (e) { return errOf(e); }
}

/// Grader-lab results: on Cloudflare they are the Learner's own document `lab-<name>`, read back with
/// `wrangler d1 execute`.
export async function dbSetShared(path, json) {
  try {
    if (!path.startsWith("lab/")) throw { code: "invalid_argument", message: "shared writes are limited to lab/" };
    const body = JSON.parse(json);
    if (!body.kind) body.kind = "lab";
    await call(() => api("PUT", docPath(path.replace(/\//g, "-")), JSON.stringify(body)));
    return ok(null);
  } catch (e) { return errOf(e); }
}

/// One structured call to Claude through the Worker. `tier` is quick | default | complex.
export async function sampleJson(prompt, tier) {
  try {
    const r = await api("POST", "/api/claude/json", JSON.stringify({ prompt, tier: tier || "default" }), { timeout: CLAUDE_TIMEOUT_MS });
    return ok(r && r.value);
  } catch (e) { return errOf(e); }
}

/// One plain-text call to Claude (the step tutor): {text, truncated}.
export async function sampleText(prompt, tier) {
  try {
    const r = await api("POST", "/api/claude/text", JSON.stringify({ prompt, tier: tier || "default" }), { timeout: CLAUDE_TIMEOUT_MS });
    return ok({ text: String((r && r.text) || ""), truncated: !!(r && r.truncated) });
  } catch (e) { return errOf(e); }
}

/// Uploads the image chosen in <input type=file id=inputId>: {id, url}.
export async function uploadFromInput(inputId) {
  try {
    const input = document.getElementById(inputId);
    const file = input && input.files && input.files[0];
    if (!file) return ok(null);
    if (!/^image\/(png|jpeg|webp|gif)$/.test(file.type)) throw { code: "invalid_argument", message: "please choose a PNG, JPEG, WebP or GIF image" };
    if (file.size > 8 * 1024 * 1024) throw { code: "invalid_argument", message: "images up to 8 MB" };
    const r = await api("POST", "/api/files", file, { timeout: 60000, contentType: file.type });
    return ok({ id: r.id, url: r.url });
  } catch (e) { return errOf(e); }
}

// ── Accounts (Better Auth, ADR 0004). Errors come back as code "auth" with the server's message. ──

const authErr = (e) => (e && (e.code === "unavailable" || e.code === "rate_limited") ? errOf(e) : fail("auth", (e && e.message) || "Sign-in failed."));

export async function authSignIn(email, password) {
  try {
    await api("POST", "/api/auth/sign-in/email", JSON.stringify({ email, password }));
    return ok(null);
  } catch (e) { return authErr(e); }
}

export async function authSignUp(name, email, password, invite) {
  try {
    await api("POST", "/api/auth/sign-up/email", JSON.stringify({ name, email, password }), { headers: { "x-telosgard-invite": invite || "" } });
    return ok(null);
  } catch (e) { return authErr(e); }
}

export async function authSignOut() {
  try {
    await api("POST", "/api/auth/sign-out", JSON.stringify({}));
    return ok(null);
  } catch (e) { return authErr(e); }
}

/// Owner only: a single-use invite link, valid for 7 days. {id, url, expiresAt}
export async function inviteCreate() {
  try {
    return ok(await api("POST", "/api/invites", JSON.stringify({})));
  } catch (e) { return errOf(e); }
}

/// Copies text from a click handler; resolves ok(false) when the browser refuses.
export async function copyText(text) {
  try {
    await navigator.clipboard.writeText(text);
    return ok(true);
  } catch (e) { return ok(false); }
}
