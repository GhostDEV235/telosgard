# Architecture: Telosgard

> How the app is built. *What* it does is in [`PRD.md`](PRD.md). *Why* Rust/WASM on claude.ai is in [ADR 0001](adr/0001-rust-wasm-on-claude-artifact.md).

## 1. Shape at a glance

```
┌────────────────────────────── claude.ai Artifact viewer (iframe) ──────────────────────────────┐
│                                                                                                 │
│  index.html ──loads──▶ gold_school.js (wasm-bindgen glue) ──▶ gold_school_bg.wasm (Leptos app)   │
│       │                                                         │                               │
│       └── tailwind.css (compiled)                               │ calls via wasm-bindgen         │
│                                                                 ▼                               │
│                                                     bridge.js  (thin JS shim, ~100 lines)       │
│                                                         │                                       │
│                                       window.claude.use("sample") / use("db") / use("user")     │
└─────────────────────────────────────────────────────────┼───────────────────────────────────────┘
                                                          ▼
                                       Claude (grading)       Artifact DB (Learner's private store)
```

- **No server of our own.** The whole app is static files published to one claude.ai Artifact.
- **Rust everywhere except `bridge.js`.** The platform API is JavaScript-only (`claude.use()` returns JS promises of frozen namespaces), so a small shim adapts it to plain async functions that Rust imports.

## 2. Workspace layout

```
trade_school/
├── CONTEXT.md                     # glossary (domain language)
├── docs/                          # PRD, architecture, plan, ADRs, curriculum, design system
├── Cargo.toml                     # workspace
├── crates/
│   ├── core/                      # package `telosgard-core` (`core` clashes with Rust's built-in crate). PURE logic, no web.
│   │   └── src/
│   │       ├── money.rs           # DollarMove, Price, Lots, Usd newtypes
│   │       ├── sizing.rs          # position-size calculator
│   │       ├── gate.rs            # Quiz score + Grade → passed?
│   │       ├── plan.rs            # TradingPlan, HardRule, SoftRule, versions
│   │       ├── journal.rs         # JournalEntry, hard-rule evaluation
│   │       ├── readiness.rs       # Live Readiness computation
│   │       ├── course.rs          # Stage/Lesson ids, unlock order
│   │       ├── drills.rs          # seeded illustrative candle generator + drill answer checking
│   │       ├── time.rs            # UTC ↔ New York (US DST rule), sessions, rollover, No-Trade Windows
│   │       ├── checklist.rs       # Pre-Session Checklist + validation
│   │       └── review.rs          # Weekly Review statistics
│   ├── content/                   # Lesson content compiled into Rust data at build time
│   │   ├── build.rs               # parses lessons/*.md → generated.rs
│   │   └── lessons/
│   │       ├── 01-what-is-trading.md
│   │       └── …
│   └── app/                       # Leptos CSR front-end (compiled to wasm32)
│       ├── index.html             # Trunk entry
│       ├── bridge.js              # claude.use() shim
│       ├── tailwind.css           # @import "tailwindcss"; @theme tokens
│       └── src/
│           ├── main.rs
│           ├── platform/          # Rust side of the bridge: sample(), db(), user()
│           ├── store/             # typed repositories over the db (progress, plans, journal)
│           ├── grading/           # prompt builders + JSON response types for Grade & Verdict
│           ├── ui/                # design-system components (Card, Button, Sheet, Segmented…)
│           └── screens/           # Home, Lesson, Quiz, ExplainBack, Calculator, Plan, Journal, Readiness, Drill
└── dist/                          # Trunk output, the files that get published (git-ignored)
```

**Rule:** `core` depends on nothing web-related, so every money and rule calculation is testable with `cargo test` on the Mac. `app` depends on `core` and `content`.

## 3. Technology choices

| Concern | Choice | Notes |
|---|---|---|
| Language | Rust (stable), `wasm32-unknown-unknown` | |
| UI framework | **Leptos** (client-side rendering only) | Fine-grained signals, JSX-like `view!` macro. Exact version pinned in Phase 0 from crates.io |
| Build | **Trunk** | Builds the wasm, generates JS glue, runs the Tailwind CLI through a pre-build hook, hashes assets |
| Styling | **Tailwind CSS v4** standalone CLI (via npm, Node is installed) | Design tokens defined in `@theme` from [`DESIGN-SYSTEM.md`](DESIGN-SYSTEM.md); compiled, not loaded from a CDN |
| JS interop | `wasm-bindgen`, `wasm-bindgen-futures`, `js-sys`, `web-sys` | `#[wasm_bindgen(module = "/bridge.js")]` |
| Serialisation | `serde`, `serde_json`, `serde-wasm-bindgen` | DB documents and Claude JSON |
| Markdown | `pulldown-cmark` **in `build.rs` only** | Lessons become pre-rendered HTML at build time, so no Markdown parser ships in the wasm |
| Ids | `ulid` | Journal entry and attempt ids sort by time |
| Charts | Hand-rolled SVG Leptos components | Candles are just rects and lines; no chart library |
| Fonts | System stack (`-apple-system, BlinkMacSystemFont, "SF Pro Text", …`) | Gives a native Apple look with nothing to load |

## 4. The bridge

> **Since Wave 1.9 (ADR 0003)** the bridge talks to Telosgard's own Cloudflare Worker instead of `window.claude`. Its exports and the JSON-string result shape did not change, so the Rust side is the same.

`bridge.js` exposes plain async functions that each resolve to `{"ok":true,"value":…}` or `{"ok":false,"code":…,"message":…}` and never reject: `init`, `dbGet`, `dbSet`, `dbDelete`, `dbList(kind)`, `dbSetShared` (grader lab → the Learner's own `lab-…` document), `sampleJson`, `sampleText`, `uploadFromInput`. Each is one `fetch` to `/api/…` on the same origin (the Cloudflare Access cookie rides along), with a timeout and one retry for transient failures. Error codes from the Worker (`not_granted`, `invalid_argument`, `quota_exceeded`, `rate_limited`, `unavailable`) map onto `PlatformError` in `platform.rs`.

**Absence by design:** if the Worker says the viewer is signed out, the app still renders and teaches; only saving and Claude actions show a "not available" state.

### The Worker (`cloudflare/`)
- One Worker with **static assets**: `dist/` (the student app) and `dist/admin/` (the dashboard, ADR 0006), assembled by `scripts/build.sh`, are served as files; the Worker runs only for `/api/*` (`run_worker_first`).
- **Identity (ADR 0004):** Better Auth sessions (email + password) at `/api/auth/*`, tables in D1; the session's user id keys every row. Sign-up is invite-only: owner-made single-use links (`/#join-<token>`, 7 days) checked in a Better Auth `before` hook and spent in a database hook; `OWNER_EMAILS` addresses may sign up without one. `GET /api/me` reports `isOwner`; `POST|GET|DELETE /api/invites` are owner-only. `DEV_USER` (from `.dev.vars`) stands in only for requests to localhost.
- **Routes:** `/api/auth/*` (Better Auth); `GET /api/me`; `/api/invites` (owner); `GET /api/docs?kind=`; `GET|PUT|DELETE /api/docs/:id`; `POST /api/files`, `GET /api/files/:id`; `POST /api/claude/json|text`.
- **Claude** (`src/claude.ts`, official `@anthropic-ai/sdk`): the app sends `{prompt, tier}`; the Worker calls `claude-haiku-5-5` (the lowest-cost current model, chosen 2026-11-13; Opus 5.5 before) with adaptive thinking, effort by tier (quick → low, default → medium, complex → high), a short system line per mode and an output cap (JSON 16,000, text 4,000 tokens). Haiku 5.5 has no server-side refusal fallback, so a decline comes back as `refused`. JSON replies are parsed (code fences tolerated); a refusal, a cut-short or unreadable reply, or an API error becomes an error code (`refused`, `invalid_json`, `rate_limited`, `not_granted`, `unavailable`), never a made-up answer. Each call is logged in D1 `claude_calls` (user, time, mode, ok, tokens), which also enforces a cap per 24 h: the owner (an address in the `OWNER_EMAILS` secret, or the localhost dev user) gets the Owner limit and everyone else the Learner limit, both live settings in the dashboard (D3; `CLAUDE_DAILY_LIMIT` 300 and `TESTER_DAILY_LIMIT` 50 are only the defaults), with an optional cap on all accounts together. The key is the Worker secret `ANTHROPIC_API_KEY`; without it the route answers `not_granted`. Prompts are built per request with the Learner's own words in them, so prompt caching isn't used.
- **Storage:** D1 table `docs (user_id, doc_id, kind, body, updated_at)`, primary key `(user_id, doc_id)`, index `(user_id, kind)`; `files` table for screenshot ownership; images in R2 under `<user_id>/<id>`.
- **Tests:** Vitest with `@cloudflare/vitest-plugin` (Workers runtime, local D1 and R2): identity, isolation between users, validation, files, and the Claude route against a fake client (model, effort, fallbacks, JSON parsing, refusals, cap and logging).

## 5. Data model (D1 `docs` table; formerly the Artifact DB)

Each row of D1's `docs` table is one of the Learner's documents (keyed by user and doc id), tagged by `kind`. The shapes are unchanged from the Artifact database, so data moves across as-is:

| Doc id | `kind` | Fields |
|---|---|---|
| `profile` | profile | `createdAt`, `disclaimerAcceptedAt`, `theme` (`system`/`dark`/`light`) |
| `lesson-<NN>` | progress | `quizBest` (0–100), `quizPassedAt?`, `grade` (latest), `gotItAt?`, `passedAt?` |
| `attempt-<ulid>` | attempt | `lesson`, `answer`, `grade`, `feedback`, `reteach?`, `at` |
| `plan-v<N>` | plan | `version`, `createdAt`, `hardRules{…}`, `softRules{setup, extra[]}` |
| `checklist-<YYYY-MM-DD>` | checklist | local `date`, `events[{name, tier, atUtc}]`, `asianHigh`, `asianLow`, `mood`, `bias`, `zones[]`, `scenarios[]`, `risk{perTradePct, dailyLimitR, plannedSizes[]}` |
| `review-<YYYY-MM-DD>` | review | `week` (Monday, days since epoch), `note` |
| `trade-<ulid>` | journal | `status` (`taken`/`skipped`), `entry{at, direction, entry, stop?, takeProfit?, lots, balanceBefore, contractOz, setupReason, planVersion}`, `checklistDay`, `run`, `scenario?`, `hardResults[]`, `adherence{state: pending/judged/dispute_pending/disputed, …}`, `softResults[]`, `verdictSummary?`, `firstSummary?`, `disputeText?`, `screenshot?` (asset id), `result?{exit, pnl, balanceAfter, closedAt}` |
| `drill-<id>` | drill | `answered`, `correct`, `lastAt` (all-time totals) |
| `run-<N>` | run | `number`, `startedAt`, `startingBalance`, `endedAt?`, `endReason?` (`breach`). Max total loss is measured against the run's starting balance |
| `readiness` | readiness | `reachedAt?` (written once; unlocks the Live and Advanced Tracks permanently) |

- Lists use `WHERE user_id = ? AND kind = ?` on the `(user_id, kind)` index.
- Documents stay well under the 256 KiB limit. Screenshots are stored in R2 through `POST /api/files`, and only the file **id** goes in the trade document.
- **Derived, not stored:** whether a lesson is unlocked, adherence %, readiness progress. These are computed in `core` from the stored documents, so a fix to the rules re-evaluates history.

## 6. Grading design

Grading is one `sample.json(...)` call. The prompt is built in Rust (`grading/`) and contains everything, because the call has no memory.

### 6.1 Explain-Back
```
SYSTEM-ish preamble (as first user turn): You are a patient trading tutor grading a complete beginner…
LESSON: title, objective, the lesson's analogy (so a "Not yet" re-teach must use a DIFFERENT one)
RUBRIC: must_include[] (key ideas), misconceptions[] (with why each is wrong)
LEARNER ANSWER: …
RETURN JSON: {"grade":"got_it"|"almost"|"not_yet","feedback":string,"gap":string|null,"reteach":string|null,"suggestion":string|null}
```
- `modelTier: "default"`. Responses are short.
- `got_it` requires every `must_include` item, paraphrase allowed, and no misconception present.
- Prompt and validation live in `telosgard_core::grading` (unit-tested): the answer is fenced in `<learner_answer>` tags (tags inside it are stripped, length capped at 2,000 chars) and Claude is told to treat it as data; the reply must have a known grade, non-empty feedback, a `gap` for Almost and a `reteach` for Not yet.
- `suggestion` (required for Almost / Not yet): the Learner's own answer, changed as little as possible to include every key idea, with each addition in `[[double brackets]]`; the app highlights those parts. It's shown as a model to learn from; passing still needs the Learner's own rewrite.
- **No automatic retry.** The platform asks pages not to retry an unreadable reply (`invalid_json`) from code, so any failure shows the message and a "Try again" button. **Never default to pass.**
- Rubric tuning uses the hidden `#grader-lab` page: it runs every fixture through the same call and writes the outcome to the shared doc `lab/grader-latest`, which Claude Code reads with `ArtifactData`.

### 6.2 Adherence Verdict
1. `core::journal::evaluate_hard_rules(entry, plan)` → `Vec<HardRuleResult>`. This is deterministic.
2. Prompt: the plan's Soft Rules, the entry (with the optional screenshot attached as an image), the Hard Rule results (as facts Claude must not contradict).
3. `RETURN JSON: {"adherent":bool,"soft_results":[{"rule":…, "pass":bool, "why":…}],"summary":string}`
4. Final `adherent` = all hard rules pass **AND** Claude's `adherent`. Claude cannot overturn a Hard Rule failure.
5. **Dispute:** the same prompt plus the first verdict and the Learner's argument. The second verdict is final and stored alongside the first.

## 7. Content pipeline

*(Format v2, Wave 1.5: the explanation lives in `[[steps]]`, each `{heading, text, why?, widget?}`; a `[[recall]]` set of exactly 3 questions; every `[[quiz]]` / `[[recall]]` has an `id`; the Markdown body must be empty; main path 2–5 minutes. The sample below shows the original v1 fields that are unchanged.)*

Each lesson is one file, `crates/content/lessons/NN-slug.md`: TOML front matter between `+++` lines. The front matter holds every other part of the lesson, so the reader always renders the CURRICULUM order (hook → explanation → analogy → example → common mistake → takeaways → videos → quiz → Explain-Back → gate → sources):

```toml
id = 5
stage = 1
title = "Bid, ask & spread"
objective = "…"            # completes "By the end, you'll be able to …"
hook = "…"
analogy = "…"
analogy_tag = "airport currency booth"   # a Not yet re-teach must use a different one
example = "…"
mistake = "…"
takeaways = ["…", "…", "…"]
drills = ["bid_ask"]

[[videos]]                  # 0–2; opened and confirmed (oEmbed) before commit
id = "r1N22WzhzPM"          # YouTube id; the app builds the link
title = "…"  channel = "…"  duration = "7:53"
watch_for = "…"  fallback_search = "…"  checked = "2026-11-10"

[[sources]]                 # ≥ 1; primary or official pages only (CURRICULUM rule 8)
claim = "…"  name = "…"  url = "https://…"  checked = "2026-11-10"

[[quiz]]                    # 5–8
q = "…"  options = ["…", "…"]  answer = 0  why = "…"

[explain_back]
prompt = "…"  must_include = ["…"]  misconceptions = ["…"]
[[explain_back.fixtures]]   # exactly 3: got_it, almost, almost_or_not_yet (grader-lab tuning)
answer = "…"  expect = "got_it"
```

`build.rs` rejects unknown fields and **fails the build** when a lesson breaks a rule: id/filename mismatch, 3–5 takeaways, 5–8 quiz questions with `answer` in range, 2–4 `must_include` and 1–3 misconceptions, the fixture order, ≤ 2 videos with an 11-character id and a checked date, ≥ 1 https source with a checked date, a known drill kind, the word "pip" outside Lesson 6, or a reading time outside 5–8 minutes (estimated at 130 words a minute). It renders the Markdown body (tables enabled) to HTML and writes `generated.rs` with `static LESSONS: &[Lesson]`. The app reads lessons, rubrics and grader fixtures from there; nothing about a written lesson lives in the app crate.

## 8. Navigation

The app runs inside an iframe at a single URL, so routing is **hash-based**. The platform passes only a **plain `#name`** (ASCII letters, digits and hyphens; never `#/path` or `#key=value`), so the routes are `#today`, `#lesson-5`, `#size`, `#plan`, `#journal`, `#readiness`, `#drill-6a`, with no hash meaning the start view. Reserved names (`settings`, `sidebar`, `customize`, `upgrade`, `downloads`, `comment-*`) are never used. A small signal-based router listens to `hashchange`. Unlock checks run on every navigation: deep-linking to a locked lesson shows its lock card.

## 9. Build & deploy (Cloudflare)

1. `scripts/build.sh` runs `trunk build --release` for `crates/app` and `crates/admin` and assembles `dist/`. Each build runs the Tailwind CLI (both import the shared `crates/ui/style/base.css`), then builds the wasm, the glue JS and a hashed `dist/` (a normal web page; the Artifact conversion step is gone).
2. `cd cloudflare && npm test` runs the Worker tests; `npm run typecheck` checks types.
3. Locally: `cp .dev.vars.example .dev.vars`, `npm run db:local`, then `npm run dev` serves the app and the API on `http://localhost:8787` with a local D1 and R2.
4. Deploy (Phase 20): `wrangler d1 create telosgard` (put its id in `wrangler.toml`), `wrangler r2 bucket create telosgard-files`, `npm run db:remote`, `wrangler secret put ANTHROPIC_API_KEY`, set `ACCESS_TEAM_DOMAIN` and `ACCESS_AUD`, then `npm run deploy`.

The claude.ai Artifact build (`scripts/artifactify.mjs`) is kept only for reference; the Artifact is frozen at version 21.

## 10. Testing strategy

| Layer | How |
|---|---|
| `core` | `cargo test`, test-first: sizing edge cases, rounding, gate logic, readiness windows, hard rules, plan versioning, drill generator determinism |
| `content` | `build.rs` validation is the test; plus a `cargo test` that every lesson id is contiguous and stages are ordered |
| Grading prompts | Golden fixtures: a set of sample answers per lesson (a correct one, one with a gap, one with a misconception), run by hand against Claude in Phase 5 to tune each rubric |
| `app` | `wasm-pack test --headless` for the bridge's JSON mapping; manual QA checklist per screen in both themes and at 360 px / 1280 px |

## 11. Known unknowns, settled in Phase 1 (proof of concept)
1. Does the Artifact viewer's content security policy allow WebAssembly compilation (`WebAssembly.instantiateStreaming`) and serve `.wasm` as `application/wasm`?
2. ~~Are YouTube iframes allowed?~~ **Settled: no.** The platform blocks all third-party iframes and images from other hosts. The VideoCard is a styled text card (title, channel, duration, "Watch for", start time) linking out with `<a href target=_blank>`.
3. Does `sample.json` with `images: true` accept a screenshot blob from `assets`?
4. ~~Wasm bundle size~~ **Settled:** the spike is 88 KB raw / 37 KB gzipped (Leptos 0.8.23, `opt-level="z"`, LTO, wasm-opt).

If (1) fails, **stop and return to the Learner** before changing architecture (ADR 0001).
