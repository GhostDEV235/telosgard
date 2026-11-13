// Curriculum (D2): how each lesson is going across all accounts, Hidden Lessons and notes.
// Lesson text lives in the repo (ADR 0006); here we only count what Learners did with it.
// Explain-Back answers are returned without any identity (Instructors never see who wrote them).

import { ownerEmails, type AuthEnv } from "./auth";

export const LESSON_ID = /^lesson-([0-9]{2}|a[0-9]{1,2}|ind)$/;

type Row = { user_id: string; doc_id: string; kind: string; body: string };

function parse(body: string): Record<string, unknown> {
  try {
    const v = JSON.parse(body);
    return v && typeof v === "object" ? v : {};
  } catch {
    return {};
  }
}

const num = (v: unknown) => (typeof v === "number" && Number.isFinite(v) ? v : 0);
const lessonOf = (v: unknown): string | null => {
  if (typeof v === "number") return `lesson-${String(v).padStart(2, "0")}`;
  if (typeof v === "string" && LESSON_ID.test(v)) return v;
  if (typeof v === "string" && /^\d{1,2}$/.test(v)) return `lesson-${v.padStart(2, "0")}`;
  return null;
};

export async function hiddenLessons(db: D1Database): Promise<string[]> {
  const { results } = await db.prepare("SELECT lesson FROM hidden_lessons ORDER BY lesson").all<{ lesson: string }>();
  return results.map((r) => r.lesson);
}

/** The lesson before a numbered one ("lesson-08" → "lesson-07"); null for Lesson 1 and non-numbered ids. */
function previous(lesson: string): string | null {
  const m = /^lesson-(\d{2})$/.exec(lesson);
  if (!m || m[1] === "01") return null;
  return `lesson-${String(Number(m[1]) - 1).padStart(2, "0")}`;
}

export interface LessonStats {
  lesson: string;
  hidden: boolean;
  started: number;
  passed: number;
  /** Accounts that passed the lesson before and not this one: who a hide would pause. */
  waiting: number;
  answered: number;
  wrong: number;
  guessed: number;
  gotIt: number;
  almost: number;
  notYet: number;
}

async function rows(db: D1Database): Promise<Row[]> {
  const { results } = await db.prepare("SELECT user_id, doc_id, kind, body FROM docs WHERE kind IN ('progress', 'qrec', 'attempt')").all<Row>();
  return results;
}

/** `include`: lessons to report even when nobody has saved anything for them yet. */
export async function curriculumSummary(env: AuthEnv, include: string[] = []): Promise<LessonStats[]> {
  const all = await rows(env.DB);
  const hidden = new Set(await hiddenLessons(env.DB));
  const stats = new Map<string, LessonStats>();
  const get = (lesson: string) => {
    let s = stats.get(lesson);
    if (!s) {
      s = { lesson, hidden: hidden.has(lesson), started: 0, passed: 0, waiting: 0, answered: 0, wrong: 0, guessed: 0, gotIt: 0, almost: 0, notYet: 0 };
      stats.set(lesson, s);
    }
    return s;
  };
  const passedBy = new Map<string, Set<string>>();
  // Who a hide can pause: active Learner accounts (staff preview Hidden Lessons instead of waiting).
  const owners = ownerEmails(env);
  const { results: accounts } = await env.DB.prepare(`SELECT id, email FROM "user" WHERE role = 'learner' AND "suspendedAt" IS NULL`).all<{ id: string; email: string }>();
  const users = new Set(accounts.filter((a) => !owners.includes(a.email.toLowerCase())).map((a) => a.id));
  for (const r of all) {
    const b = parse(r.body);
    if (r.kind === "progress" && LESSON_ID.test(r.doc_id)) {
      const s = get(r.doc_id);
      s.started++;
      if (b.passedAt != null) {
        s.passed++;
        if (!passedBy.has(r.user_id)) passedBy.set(r.user_id, new Set());
        passedBy.get(r.user_id)!.add(r.doc_id);
      }
    } else if (r.kind === "qrec") {
      const lesson = lessonOf(b.lesson);
      if (!lesson) continue;
      const s = get(lesson);
      for (const q of Object.values((b.questions ?? {}) as Record<string, Record<string, unknown>>)) {
        s.answered += num(q.answered);
        s.wrong += num(q.wrong);
        s.guessed += num(q.guessed);
      }
    } else if (r.kind === "attempt") {
      const lesson = lessonOf(b.lesson);
      if (!lesson) continue;
      const s = get(lesson);
      if (b.grade === "got_it") s.gotIt++;
      else if (b.grade === "almost") s.almost++;
      else if (b.grade === "not_yet") s.notYet++;
    }
  }
  for (const lesson of [...hidden, ...include, "lesson-01"]) get(lesson);
  // The lesson after each one someone passed: people may be waiting at it before saving anything.
  for (const done of passedBy.values()) {
    for (const l of done) {
      const m = /^lesson-(\d{2})$/.exec(l);
      if (m) get(`lesson-${String(Number(m[1]) + 1).padStart(2, "0")}`);
    }
  }
  for (const s of stats.values()) s.waiting = waitingAt(s.lesson, users, passedBy);
  return [...stats.values()].sort((a, b) => a.lesson.localeCompare(b.lesson));
}

function waitingAt(lesson: string, users: Set<string>, passedBy: Map<string, Set<string>>): number {
  const prev = previous(lesson);
  let n = 0;
  for (const u of users) {
    const p = passedBy.get(u) ?? new Set();
    if (p.has(lesson)) continue;
    if (prev === null ? lesson === "lesson-01" : p.has(prev)) n++;
  }
  return n;
}

/** How many accounts a hide would pause right now. */
export async function waitingCount(env: AuthEnv, lesson: string): Promise<number> {
  const s = (await curriculumSummary(env, [lesson])).find((x) => x.lesson === lesson);
  return s?.waiting ?? 0;
}

export async function lessonDetail(env: AuthEnv, lesson: string) {
  const all = (await rows(env.DB)).filter((r) => (r.kind === "progress" ? r.doc_id === lesson : lessonOf(parse(r.body).lesson) === lesson));
  const questions = new Map<string, { q: string; answered: number; wrong: number; guessed: number; accounts: number }>();
  const answers: { grade: string; answer: string; at: number }[] = [];
  for (const r of all) {
    const b = parse(r.body);
    if (r.kind === "qrec") {
      for (const [q, rec] of Object.entries((b.questions ?? {}) as Record<string, Record<string, unknown>>)) {
        const s = questions.get(q) ?? { q, answered: 0, wrong: 0, guessed: 0, accounts: 0 };
        s.answered += num(rec.answered);
        s.wrong += num(rec.wrong);
        s.guessed += num(rec.guessed);
        s.accounts += num(rec.answered) > 0 ? 1 : 0;
        questions.set(q, s);
      }
    } else if (r.kind === "attempt" && typeof b.answer === "string") {
      answers.push({ grade: String(b.grade ?? ""), answer: b.answer.slice(0, 2000), at: num(b.at) });
    }
  }
  answers.sort((a, b) => b.at - a.at);
  const notes = await env.DB.prepare("SELECT id, author_email, body, at FROM lesson_notes WHERE lesson = ?1 ORDER BY at DESC LIMIT 100").bind(lesson).all<{ id: number; author_email: string | null; body: string; at: number }>();
  const hidden = await env.DB.prepare("SELECT hidden_at, reason FROM hidden_lessons WHERE lesson = ?1").bind(lesson).first<{ hidden_at: number; reason: string | null }>();
  return {
    lesson,
    hidden: hidden !== null,
    hiddenAt: hidden?.hidden_at ?? null,
    reason: hidden?.reason ?? null,
    waiting: await waitingCount(env, lesson),
    questions: [...questions.values()].sort((a, b) => a.q.localeCompare(b.q, undefined, { numeric: true })),
    answers: answers.slice(0, 100),
    notes: notes.results.map((n) => ({ id: n.id, author: n.author_email, body: n.body, at: n.at })),
  };
}
