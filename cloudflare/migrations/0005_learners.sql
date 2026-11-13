-- The admin dashboard, phase D1: Learners and Enrollment (ADR 0006, 0007, 0008).

-- Suspended (CONTEXT.md): signed out and unable to sign in, all data kept. Reversible.
ALTER TABLE "user" ADD COLUMN "suspendedAt" integer;
-- Full Access (CONTEXT.md): every lesson open. Replaces the FULL_ACCESS_EMAILS secret.
ALTER TABLE "user" ADD COLUMN "fullAccess" integer NOT NULL DEFAULT 0;
-- Account Deletion (ADR 0008): erased for good after this time unless cancelled.
ALTER TABLE "user" ADD COLUMN "deleteAfter" integer;
-- How the account was created: "link:<invite id>", "code:<code id>" or "owner".
ALTER TABLE "user" ADD COLUMN "joinedVia" text;

-- Invite Codes (CONTEXT.md): shareable, with a use limit and an expiry date.
CREATE TABLE invite_codes (
  id         TEXT PRIMARY KEY,
  code       TEXT NOT NULL UNIQUE,
  created_by TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  expires_at INTEGER NOT NULL,
  max_uses   INTEGER NOT NULL,
  uses       INTEGER NOT NULL DEFAULT 0,
  revoked_at INTEGER
);

-- One-time password-reset links made by the Owner or Support (only the SHA-256 is stored).
CREATE TABLE password_resets (
  id         TEXT PRIMARY KEY,
  token_hash TEXT NOT NULL UNIQUE,
  user_id    TEXT NOT NULL,
  created_by TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  expires_at INTEGER NOT NULL,
  used_at    INTEGER
);
CREATE INDEX password_resets_by_user ON password_resets (user_id);
