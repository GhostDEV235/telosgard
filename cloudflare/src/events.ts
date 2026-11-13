// Server-recorded events (ADR 0006). The app only saves documents; the Worker compares each save with
// the stored version and appends what changed. The student app never sends events itself.
//
// Types:
//   signed_up            {invite?}                     an account was created
//   lesson_quiz          {score}                       a lesson's best quiz score went up
//   explain_back         {grade}                       an Explain-Back was graded
//   lesson_passed        {}                            a lesson's Gate was passed (at = its passedAt)
//   question             {q, answered, wrong, guessed} answers to one quiz question since the last save
//   drill                {drill, answered, correct}    drill answers since the last save
//   journal_entry        {}                            a new Journal Entry (trade document)
//   checklist            {}                            a new Pre-Session Checklist
//   activity             {kind}                        any other save (plan, review, run, profile…)

export interface NewEvent {
  type: string;
  lesson?: string;
  data: Record<string, unknown>;
  at: number;
}

type Doc = Record<string, unknown> | null;

const num = (v: unknown): number => (typeof v === "number" && Number.isFinite(v) ? v : 0);
const obj = (v: unknown): Record<string, unknown> => (v && typeof v === "object" && !Array.isArray(v) ? (v as Record<string, unknown>) : {});

/** Two-digit lesson doc id for a lesson number or id string ("lesson-05"), or undefined. */
function lessonDoc(v: unknown): string | undefined {
  if (typeof v === "number" && Number.isInteger(v) && v > 0) return `lesson-${String(v).padStart(2, "0")}`;
  if (typeof v === "string" && /^lesson-[a-z0-9]{1,4}$/.test(v)) return v;
  if (typeof v === "string" && /^\d{1,3}$/.test(v)) return `lesson-${v.padStart(2, "0")}`;
  return undefined;
}

/** What changed between the stored document (`before`, null when new) and the one being saved. */
export function deriveEvents(kind: string, docId: string, before: Doc, after: Record<string, unknown>, now: number): NewEvent[] {
  switch (kind) {
    case "progress": {
      const lesson = lessonDoc(docId);
      if (!lesson) return [];
      const out: NewEvent[] = [];
      const [qb, qa] = [before?.quizBest, after.quizBest];
      if (typeof qa === "number" && (typeof qb !== "number" || qa > qb)) out.push({ type: "lesson_quiz", lesson, data: { score: qa }, at: now });
      if (after.passedAt != null && before?.passedAt == null) out.push({ type: "lesson_passed", lesson, data: {}, at: num(after.passedAt) || now });
      return out;
    }
    case "attempt": {
      // Attempts are written once each (a fresh id per grade).
      if (before) return [];
      const grade = typeof after.grade === "string" ? after.grade : "unknown";
      return [{ type: "explain_back", lesson: lessonDoc(after.lesson), data: { grade }, at: num(after.at) || now }];
    }
    case "qrec": {
      const lesson = lessonDoc(after.lesson);
      const was = obj(before?.questions);
      const out: NewEvent[] = [];
      for (const [q, rec] of Object.entries(obj(after.questions))) {
        const a = obj(rec);
        const b = obj(was[q]);
        const answered = num(a.answered) - num(b.answered);
        if (answered <= 0) continue;
        const wrong = Math.max(0, num(a.wrong) - num(b.wrong));
        const guessed = Math.max(0, num(a.guessed) - num(b.guessed));
        out.push({ type: "question", lesson, data: { q, answered, wrong, guessed }, at: num(a.lastAt) || now });
      }
      return out;
    }
    case "drill": {
      const answered = num(after.answered) - num(before?.answered);
      if (answered <= 0) return [];
      const correct = Math.max(0, num(after.correct) - num(before?.correct));
      return [{ type: "drill", data: { drill: docId.replace(/^drill-/, ""), answered, correct }, at: num(after.lastAt) || now }];
    }
    case "trade":
      return before ? [] : [{ type: "journal_entry", data: {}, at: now }];
    case "checklist":
      return before ? [] : [{ type: "checklist", data: {}, at: now }];
    default:
      return [{ type: "activity", data: { kind }, at: now }];
  }
}

export async function recordEvents(db: D1Database, userId: string, events: NewEvent[], backfill = false): Promise<void> {
  if (events.length === 0) return;
  const stmt = db.prepare("INSERT INTO events (user_id, type, lesson, data, at, backfill) VALUES (?1, ?2, ?3, ?4, ?5, ?6)");
  await db.batch(events.map((e) => stmt.bind(userId, e.type, e.lesson ?? null, JSON.stringify(e.data), e.at, backfill ? 1 : 0)));
}

/**
 * Rebuilds history from what is already stored, for activity before events existed: sign-ups from
 * account creation, passes from passedAt, Explain-Backs from each attempt, question and drill totals
 * at their last answer. Replaces any earlier backfill, so it can be run again safely. Approximate:
 * many answers collapse into one event at the time of the last one.
 */
export async function backfill(db: D1Database): Promise<{ users: number; events: number }> {
  await db.prepare("DELETE FROM events WHERE backfill = 1").run();
  let total = 0;
  const { results: users } = await db.prepare('SELECT id, "createdAt" AS created FROM "user"').all<{ id: string; created: string | number }>();
  const docUsers = await db.prepare("SELECT DISTINCT user_id FROM docs").all<{ user_id: string }>();
  const ids = new Set([...users.map((u) => u.id), ...docUsers.results.map((r) => r.user_id)]);
  for (const u of users) {
    const at = typeof u.created === "number" ? u.created : Date.parse(String(u.created));
    if (Number.isFinite(at)) {
      await recordEvents(db, u.id, [{ type: "signed_up", data: {}, at }], true);
      total += 1;
    }
  }
  for (const id of ids) {
    const { results } = await db.prepare("SELECT doc_id, kind, body, updated_at FROM docs WHERE user_id = ?1").bind(id).all<{ doc_id: string; kind: string; body: string; updated_at: number }>();
    const evs: NewEvent[] = [];
    for (const r of results) {
      let body: Record<string, unknown>;
      try {
        body = obj(JSON.parse(r.body));
      } catch {
        continue;
      }
      if (["progress", "attempt", "qrec", "drill", "trade", "checklist"].includes(r.kind)) {
        // Anything without a time of its own (a best quiz score, a journal entry) is dated at its last save.
        evs.push(...deriveEvents(r.kind, r.doc_id, null, body, r.updated_at));
      }
    }
    await recordEvents(db, id, evs, true);
    total += evs.length;
  }
  return { users: ids.size, events: total };
}
