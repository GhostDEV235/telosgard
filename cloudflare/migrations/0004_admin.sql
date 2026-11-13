-- The admin dashboard, phase D0 (ADR 0006, 0007).

-- Staff roles. The Owner is never stored: it comes only from the OWNER_EMAILS secret.
ALTER TABLE "user" ADD COLUMN "role" text NOT NULL DEFAULT 'learner';

-- Better Auth 1.7.14 two-factor plugin (TOTP + backup codes), as its schema declares for SQLite.
ALTER TABLE "user" ADD COLUMN "twoFactorEnabled" integer DEFAULT 0;
create table "twoFactor" ("id" text not null primary key, "secret" text not null, "backupCodes" text not null, "userId" text not null references "user" ("id") on delete cascade, "verified" integer default 1, "failedVerificationCount" integer default 0, "lockedUntil" date);
create index "twoFactor_secret_idx" on "twoFactor" ("secret");
create index "twoFactor_userId_idx" on "twoFactor" ("userId");

-- Server-recorded events (ADR 0006): appended by the Worker when a document save changes something.
-- `lesson` is the lesson doc id (lesson-05) when the event belongs to one; `data` is a small JSON object.
CREATE TABLE events (
  id      INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id TEXT NOT NULL,
  type    TEXT NOT NULL,
  lesson  TEXT,
  data    TEXT NOT NULL DEFAULT '{}',
  at      INTEGER NOT NULL,
  -- 1 when rebuilt from stored timestamps rather than seen happening.
  backfill INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX events_by_user ON events (user_id, at);
CREATE INDEX events_by_type ON events (type, at);
CREATE INDEX events_by_lesson ON events (lesson, type);

-- Every staff write: who, what, which record, and the values before and after (JSON).
CREATE TABLE audit_log (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  actor_id    TEXT NOT NULL,
  actor_email TEXT,
  actor_role  TEXT NOT NULL,
  action      TEXT NOT NULL,
  entity      TEXT NOT NULL,
  entity_id   TEXT,
  before      TEXT,
  after       TEXT,
  at          INTEGER NOT NULL
);
CREATE INDEX audit_by_time ON audit_log (at);
CREATE INDEX audit_by_entity ON audit_log (entity, entity_id);
