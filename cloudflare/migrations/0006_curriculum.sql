-- The admin dashboard, phase D2: Curriculum (ADR 0006, CONTEXT.md "Hidden Lesson").

-- A Hidden Lesson is offline for rework: Learners who reach it wait at a "being updated" card.
CREATE TABLE hidden_lessons (
  lesson    TEXT PRIMARY KEY,   -- the lesson's document id, e.g. lesson-07
  hidden_at INTEGER NOT NULL,
  hidden_by TEXT NOT NULL,
  reason    TEXT
);

-- Instructor and Owner notes on a lesson (what to fix, what Learners trip on).
CREATE TABLE lesson_notes (
  id           INTEGER PRIMARY KEY AUTOINCREMENT,
  lesson       TEXT NOT NULL,
  author_id    TEXT NOT NULL,
  author_email TEXT,
  body         TEXT NOT NULL,
  at           INTEGER NOT NULL
);
CREATE INDEX lesson_notes_by_lesson ON lesson_notes (lesson, at);
