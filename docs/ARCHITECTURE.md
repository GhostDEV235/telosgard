# Architecture: Gold School

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
│   ├── core/                      # package `gold-core` (`core` clashes with Rust's built-in crate). PURE logic, no web.
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

`bridge.js` exposes plain async functions. It never leaks a platform namespace object into Rust.

```js
// bridge.js (sketch)
const cap = (n) => window.claude?.use ? window.claude.use(n) : Promise.resolve(null);

export async function available(name) { return (await cap(name)) !== null; }

export async function sampleJson(messages, opts) {          // → parsed JSON, or throws {code,message}
  const s = await cap("sample"); if (!s) throw { code: "unavailable" };
  return s.json(messages, opts);
}
export async function userId() { const u = await cap("user"); return u ? u.id() : null; }
export async function dbGet(path) { … }        export async function dbSet(path, data) { … }
export async function dbUpdate(path, data) { … } export async function dbQuery(coll, field, eq) { … }
```

On the Rust side, `platform::` wraps each import into a typed `async fn` returning `Result<T, PlatformError>`. `PlatformError` mirrors the codes from the platform's type definitions (`not_granted`, `rate_limited`, `upstream_error`, …) plus `Unavailable`.

**Absence by design:** `claude.use()` may resolve `null` (preview, signed out, or permission denied). Every screen renders without the platform; only Save and Grade actions show a "not available here" state. Nothing calls the platform during the first synchronous render.

## 5. Data model (Artifact DB)

The platform's rule is that a path with an even number of segments is a document and an odd number is a collection, and each viewer's `data/users/<id>/` subtree is private. The Learner's data therefore lives in **one private collection**, `data/users/<id>`, with documents tagged by `kind`:

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

- Queries use `where("kind", "==", …)` on that single collection.
- Documents stay well under the 256 KiB limit. Screenshots are stored through the `assets` capability, and only the asset **id** goes in the trade document.
- **Derived, not stored:** whether a lesson is unlocked, adherence %, readiness progress. These are computed in `core` from the stored documents, so a fix to the rules re-evaluates history.

### Declared capabilities
```js
{
  sample: { images: true },      // grading; images let Claude see an optional trade screenshot
  db:     {},                    // data/users/<id>/ is private to its owner by default; no rules needed
  user:   {},                    // id() for the private path
  assets: {}                     // trade screenshots
}
```

## 6. Grading design

Grading is one `sample.json(...)` call. The prompt is built in Rust (`grading/`) and contains everything, because the call has no memory.

### 6.1 Explain-Back
```
SYSTEM-ish preamble (as first user turn): You are a patient trading tutor grading a complete beginner…
LESSON: title, objective, the lesson's analogy (so a "Not yet" re-teach must use a DIFFERENT one)
RUBRIC: must_include[] (key ideas), misconceptions[] (with why each is wrong)
LEARNER ANSWER: …
RETURN JSON: {"grade":"got_it"|"almost"|"not_yet","feedback":string,"gap":string|null,"reteach":string|null}
```
- `modelTier: "default"`. Responses are short.
- `got_it` requires every `must_include` item, paraphrase allowed, and no misconception present.
- Prompt and validation live in `gold_core::grading` (unit-tested): the answer is fenced in `<learner_answer>` tags (tags inside it are stripped, length capped at 2,000 chars) and Claude is told to treat it as data; the reply must have a known grade, non-empty feedback, a `gap` for Almost and a `reteach` for Not yet.
- **No automatic retry.** The platform asks pages not to retry an unreadable reply (`invalid_json`) from code, so any failure shows the message and a "Try again" button. **Never default to pass.**
- Rubric tuning uses the hidden `#grader-lab` page: it runs every fixture through the same call and writes the outcome to the shared doc `lab/grader-latest`, which Claude Code reads with `ArtifactData`.

### 6.2 Adherence Verdict
1. `core::journal::evaluate_hard_rules(entry, plan)` → `Vec<HardRuleResult>`. This is deterministic.
2. Prompt: the plan's Soft Rules, the entry (with the optional screenshot attached as an image), the Hard Rule results (as facts Claude must not contradict).
3. `RETURN JSON: {"adherent":bool,"soft_results":[{"rule":…, "pass":bool, "why":…}],"summary":string}`
4. Final `adherent` = all hard rules pass **AND** Claude's `adherent`. Claude cannot overturn a Hard Rule failure.
5. **Dispute:** the same prompt plus the first verdict and the Learner's argument. The second verdict is final and stored alongside the first.

## 7. Content pipeline

Each lesson is one file, `crates/content/lessons/NN-slug.md`: TOML front matter between `+++` lines, then the explanation in Markdown. The front matter holds every other part of the lesson, so the reader always renders the CURRICULUM order (hook → explanation → analogy → example → common mistake → takeaways → videos → quiz → Explain-Back → gate → sources):

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

## 9. Build & publish

1. `trunk build --release --public-url ./` runs the Tailwind CLI → `tailwind.css`, then builds the wasm, the glue JS and a hashed `dist/`.
2. `node scripts/artifactify.mjs dist` rewrites `dist/index.html` into the Artifact page format: no doctype/html/head/body tags (the platform adds the skeleton); `<title>` and `<meta name="claude:viewport-fit-sides">` first, then head content, then body content. Relative paths throughout.
   - **Gotcha:** the platform skeleton has its own *un-layered* reset (light color-scheme, 14px font, body colours). Un-layered CSS beats any `@layer`, so `html`/`body` colour and font rules must live outside `@layer base`. A local `trunk serve` does not show this; always check the published page.
3. Publish with the Artifact tool: `file_path = dist/index.html`, `files = { "<hashed>.js", "<hashed>_bg.wasm", "<hashed>.css" }`, `capabilities` as in §5.
4. Verification pass: one `ArtifactData list` of the Learner's collection after a test write.

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
