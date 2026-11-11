-- One row per Claude call through the Worker: a daily cap per user, and a record of token use.
CREATE TABLE claude_calls (
  user_id       TEXT NOT NULL,
  at            INTEGER NOT NULL,
  mode          TEXT NOT NULL,
  ok            INTEGER NOT NULL,
  input_tokens  INTEGER NOT NULL DEFAULT 0,
  output_tokens INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX claude_calls_by_user ON claude_calls (user_id, at);
