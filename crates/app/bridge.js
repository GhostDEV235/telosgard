// Bridge between Rust and the claude.ai Artifact runtime (ARCHITECTURE §4).
// Every export resolves to a JSON string: {"ok":true,"value":…} or {"ok":false,"code":…,"message":…}.
// It never rejects, so Rust only has to parse one shape.

const TIMEOUT_MS = 12000;
let caps = null;      // { db, user, sample } namespaces or null
let userId = null;
let initPromise = null;

const ok = (value) => JSON.stringify({ ok: true, value: value === undefined ? null : value });
const fail = (code, message) => JSON.stringify({ ok: false, code: code || "unavailable", message: String(message || "") });

function use(name) {
  try {
    return window.claude && typeof window.claude.use === "function" ? window.claude.use(name) : Promise.resolve(null);
  } catch (e) {
    return Promise.resolve(null);
  }
}

function withTimeout(promise, ms = TIMEOUT_MS) {
  let t;
  const timer = new Promise((_, reject) => {
    t = setTimeout(() => reject({ code: "unavailable", message: "timed out" }), ms);
  });
  return Promise.race([promise, timer]).finally(() => clearTimeout(t));
}

// One retry after a short random delay for transient failures, per the db contract.
async function call(fn) {
  try {
    return await withTimeout(fn());
  } catch (e) {
    if (e && e.code === "unavailable") {
      await new Promise((r) => setTimeout(r, 300 + Math.random() * 600));
      return await withTimeout(fn());
    }
    throw e;
  }
}

const errOf = (e) => fail(e && e.code, e && e.message ? e.message : e);

export function init() {
  if (!initPromise) {
    initPromise = (async () => {
      try {
        const [db, user, sample] = await Promise.all([use("db"), use("user"), use("sample")]);
        caps = { db, user, sample };
        if (user) {
          try { userId = await user.id(); } catch (e) { userId = null; }
        }
        return ok({ db: !!db && !!userId, user: !!user, sample: !!sample, signedIn: !!userId });
      } catch (e) {
        caps = { db: null, user: null, sample: null };
        return errOf(e);
      }
    })();
  }
  return initPromise;
}

function docRef(docId) {
  if (!caps || !caps.db || !userId) throw { code: "not_granted", message: "storage unavailable" };
  return caps.db.doc(`data/users/${userId}/${docId}`);
}

export async function dbGet(docId) {
  try {
    const snap = await call(() => docRef(docId).get());
    return ok(snap.exists ? snap.data() : null);
  } catch (e) { return errOf(e); }
}

export async function dbSet(docId, json) {
  try {
    const body = JSON.parse(json);
    await call(() => docRef(docId).set(body));
    return ok(null);
  } catch (e) { return errOf(e); }
}

export async function dbDelete(docId) {
  try {
    await call(() => docRef(docId).delete());
    return ok(null);
  } catch (e) { return errOf(e); }
}

/// All of the Learner's documents of one kind: [{id, data}].
export async function dbList(kind) {
  try {
    if (!caps || !caps.db || !userId) throw { code: "not_granted", message: "storage unavailable" };
    const q = caps.db.collection(`data/users/${userId}`).where("kind", "==", kind).limit(1000);
    const snap = await call(() => q.get());
    return ok(snap.docs.map((d) => ({ id: d.id, data: d.data() })));
  } catch (e) { return errOf(e); }
}

/// One structured call to Claude. `tier` is quick | default | complex.
export async function sampleJson(prompt, tier) {
  try {
    if (!caps || !caps.sample) throw { code: "not_granted", message: "Claude unavailable" };
    const value = await withTimeout(caps.sample.json(prompt, { modelTier: tier || "default" }), 90000);
    return ok(value);
  } catch (e) { return errOf(e); }
}
