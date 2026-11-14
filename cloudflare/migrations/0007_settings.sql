-- The admin dashboard, phase D3: Settings (FR-17.7, CONTEXT.md "Maintenance Mode", "Disclaimer").

-- Live settings the Owner changes in the dashboard. Each value is JSON; a missing key means the default.
--   claude_limits  { owner, learner, total }   Claude calls per 24 hours (total: all accounts, null = no cap)
--   maintenance    { on, messageEn, messageFr, since }
--   cron_last_run  { at, erased }
CREATE TABLE settings (
  key        TEXT PRIMARY KEY,
  value      TEXT NOT NULL,
  updated_at INTEGER NOT NULL,
  updated_by TEXT
);

-- The first-launch disclaimer, versioned: publishing a new version asks every Learner to accept it again.
-- Paragraphs are separated by a blank line; **text** is bold.
CREATE TABLE disclaimers (
  version      INTEGER PRIMARY KEY,
  text_en      TEXT NOT NULL,
  text_fr      TEXT NOT NULL,
  published_at INTEGER NOT NULL,
  published_by TEXT
);

-- Version 1 is the text the app shipped with, so Learners who already accepted it aren't asked again.
INSERT INTO disclaimers (version, text_en, text_fr, published_at, published_by) VALUES (1, 'Telosgard teaches you how gold trading works, step by step. It is **education, not financial advice**.

It will never tell you to buy or sell, and it never shows live prices or signals. Charts in drills are illustrative, not real market data.

Professor Aurel, who grades your explanations, judges your journal and answers your questions, is Telosgard''s AI tutor.

Trading can lose money, including more than you expect, quickly. That''s why you''ll practise on a **demo account** until your discipline is proven, and only then learn about going live.', 'Telosgard t’apprend comment fonctionne le trading de l’or, pas à pas. C’est de la **formation, pas du conseil financier**.

Il ne te dira jamais d’acheter ou de vendre, et n’affiche jamais de prix en direct ni de signaux. Les graphiques des exercices sont des illustrations, pas de vraies données de marché.

Le Professeur Aurel, qui note tes explications, juge ton journal et répond à tes questions, est le tuteur IA de Telosgard.

Le trading peut faire perdre de l’argent, parfois plus que prévu, et vite. C’est pour ça que tu t’entraîneras sur un **compte démo** jusqu’à ce que ta discipline soit prouvée, et seulement ensuite tu apprendras à passer au réel.', 1794182400000, NULL);
