-- The Learner's documents, kind-tagged JSON exactly as the app stored them in the Artifact database.
CREATE TABLE docs (
  user_id    TEXT NOT NULL,
  doc_id     TEXT NOT NULL,
  kind       TEXT NOT NULL,
  body       TEXT NOT NULL,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY (user_id, doc_id)
);
CREATE INDEX docs_by_kind ON docs (user_id, kind);

-- Chart screenshots live in R2; this table records who owns each one.
CREATE TABLE files (
  id           TEXT PRIMARY KEY,
  user_id      TEXT NOT NULL,
  content_type TEXT NOT NULL,
  size         INTEGER NOT NULL,
  created_at   INTEGER NOT NULL
);
CREATE INDEX files_by_user ON files (user_id);
