// Spike bridge: adapts window.claude.use() to plain async functions returning strings.
const cap = (n) => (window.claude && window.claude.use) ? window.claude.use(n) : Promise.resolve(null);
const errStr = (e) => "ERR " + (e && e.code ? e.code + ": " : "") + (e && e.message ? e.message : String(e));

export async function available(name) {
  try { return (await cap(name)) !== null ? "yes" : "no (null)"; } catch (e) { return errStr(e); }
}
export async function samplePing() {
  try {
    const s = await cap("sample");
    if (!s) return "sample unavailable (null)";
    const v = await s.json('Reply with exactly this JSON and nothing else: {"ok": true}', { modelTier: "quick" });
    return "OK " + JSON.stringify(v);
  } catch (e) { return errStr(e); }
}
export async function dbPing() {
  try {
    const [db, user] = await Promise.all([cap("db"), cap("user")]);
    if (!db || !user) return "db/user unavailable (null)";
    const id = await user.id();
    if (!id) return "no user id";
    const ref = db.doc(`data/users/${id}/ping`);
    const before = await ref.get();
    const prev = before.exists ? before.data().at : "none";
    const now = new Date().toISOString();
    await ref.set({ at: now });
    return `OK previous=${prev} wrote=${now}`;
  } catch (e) { return errStr(e); }
}
