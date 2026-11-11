# Implementation Plan: Wave 1

> Build order for Wave 1 (the app, all tools, Stage 1). Each phase ends with an **exit check**. A failed check stops the line rather than being worked around.
> References: [PRD](PRD.md) · [Architecture](ARCHITECTURE.md) · [Design System](DESIGN-SYSTEM.md) · [Curriculum](CURRICULUM.md)
>
> **Revised 2026-11-10:** folds in the Learner's outside 7-phase lesson plan (PRD §8 Q3): course map v2 (8 stages, 39 core lessons), Live Readiness 50/8 weeks, new Hard Rules, Pre-Session Checklist, Weekly Review, session clock, Fibonacci and ATR.

## Phase 0: Toolchain ✅
- [x] Rust (`rustup`, stable 1.99.1) + `wasm32-unknown-unknown`
- [x] Trunk 0.22.1 (fetches wasm-bindgen 0.2.130 and wasm-opt itself)
- [x] Tailwind v4.3.5 CLI via npm, local to the project
- [x] Exact crate versions pinned in `Cargo.toml`
- [x] `git init`, `.gitignore`

## Phase 1: Proof of concept ✅ *(go/no-go on ADR 0001)*
- [x] **WASM loads** in the claude.ai viewer (Learner confirmed the counter works)
- [x] **Bridge → db/user:** confirmed by the published app loading the Learner's profile (Phase 4)
- [x] **Bridge → sample** (test 2b): confirmed 2026-11-10, the Learner's Lesson 1 Explain-Back was graded live by Claude (stored attempt, grade "almost")
- [x] **YouTube:** iframes and thumbnails are blocked by the platform → link-out VideoCards
- [x] **Bundle size:** spike 37 KB gzipped; full app 175 KB gzipped

## Phase 2: Domain core (`crates/core`, package `telosgard-core`), test-first ✅
- [x] `money`, `sizing`, `gate`, `course`, `plan`, `journal`, `readiness`, `drills` (56 tests)

## Phase 3: App shell & design system ✅
- [x] Workspace + Trunk + Tailwind pipeline; tokens from DESIGN-SYSTEM §6
- [x] Theme: system default, toggle, `data-theme`, persisted (localStorage now; `profile.theme` in Phase 4)
- [x] Plain-hash router: `#today`, `#lesson-5`, `#practice`, `#drill-direction`, `#size`, `#plan`, `#journal`, `#readiness`
- [x] Components: StageCard, LessonRow, Reader, VideoCard, QuizCard, ExplainBackSheet, GateStatus, SegmentedControl, NumberField, CandleChart, RingProgress, Toast
- [x] Home "Today" card stack; first-launch disclaimer
- [x] Fix: `html`/`body` styles un-layered so the viewer's reset can't override them (ARCHITECTURE §9)

## Phase 3b: Realign to course map v2 ✅ *(test-first in `telosgard-core`)*
Code built before the 2026-11-10 revision that now disagrees with the docs:
- [x] `course`: `LAST_CORE = 39`, `LAST_LIVE = 43`, `LAST_ADVANCED = 10`, 8 `STAGES` (1–11, 12–14, 15–19, 20–22, 23–27, 28–30, 31–35, 36–39); Indicators unlocks after L27. Update tests.
- [x] `readiness`: `REQUIRED_TRADES = 50`, `REQUIRED_DAYS = 56`. Update tests (boundary cases at 49/50 trades and 55/56 days).
- [x] `time` *(new module)*: UTC ↔ New York conversion with the US daylight-saving rule (2nd Sunday of March → 1st Sunday of November, 02:00 local), so sessions and rollover need no timezone database. Session at an instant (Asian / London / New York / London–NY overlap / closed). Tests on both DST transition days and across the year boundary.
- [x] `journal`: new Hard Rules, each with a plain-language detail: *(the session is now derived from the entry time, not typed in)*
  - `DailyLossLimit`: closed losses earlier that day (in R, where 1R = the plan's max risk) have reached the plan's limit → fail
  - `NoTradeWindow`: entry within the daily rollover (17:00–18:00 NY), the first hour after the Sunday open (18:00–19:00 NY), or ±5 min of a tier-1 event listed in that day's checklist → fail
  - `ChecklistDone`: no Pre-Session Checklist saved for the entry's local day → fail
- [x] `plan`: `HardRules.daily_loss_limit_r` (validated > 0); the "matches a written Scenario" Soft Rule is implicit in every plan
- [x] `checklist` *(new module)*: `PreSessionChecklist` (date, events with UTC times and tier, Asian high/low, macro mood, bias, zones, Scenarios, risk commitment) + validation (≥ 1 Scenario, ≥ 1 zone)
- [x] `review` *(new module)*: `weekly_review(entries) -> WeeklyReview` (win rate, average R, rule breaks by rule, skipped count); R of a closed trade = P/L ÷ planned risk
- [x] App: `catalog.rs` titles and stage blurbs from CURRICULUM v2; stale copy ("20 trades over 3 weeks", "Lesson 24/25") → 50 / 8 weeks, Lesson 38/39; StageCard shows the suggested pace

**Exit check:** `cargo test --workspace` green; every new Hard Rule has a pass case, a fail case and a boundary case; Home shows 8 stages + Live + Advanced with the new titles.

## Phase 4: Platform bridge & persistence
- [x] `bridge.js` (JSON-string results, one retry on `unavailable`, 12 s timeout for storage, 90 s for Claude) + `platform::{init, get, set, delete, list, sample_json}` with typed `PlatformError`
- [x] `store::` repositories: profile, progress, attempts, plans, trades, checklists, runs, drills, readiness; every document `kind`-tagged
- [x] Race-safe saves: a lesson save merges with what's stored (`LessonProgress::merge`, tested in `telosgard-core`); progress made before the load finishes is saved once it lands
- [x] Wired now: lesson progress, theme + disclaimer in `profile`, drill totals. Attempts, plans, trades, checklists and runs are wired in Phases 5 and 7
- [x] Degraded mode: a status chip on Home (Connecting / Saved / Not saving in this view / sign in / couldn't save); with no platform the app renders all content (checked locally, no console errors)
- [x] Published with capabilities `db`, `user`, `sample{images}`, `assets`; the live page reports "Progress saved to your account"
- [x] **Learner check:** confirmed 2026-11-10 via `ArtifactData`: the Learner's `lesson-01` progress (quiz best 100%) and their Explain-Back attempt are stored in their private folder

**Exit check:** in the published preview, progress survives a reload, and with the platform absent the app still renders all content.

## Phase 5: Learning loop
- [x] Quiz flow: shuffle, instant feedback, best score saved
- [x] Explain-Back: prompt builder + strict reply check in `telosgard_core::grading` (7 tests), the three Grade UIs (Got it + optional tip / Almost + "what's missing" / Not yet + re-teach with a different analogy), errors with "Try again" (no auto-retry, per the platform), attempt history saved and listed
- [x] GateStatus wiring + unlocking the next lesson; a gold "Lesson passed" banner and toast
- [x] Rubrics + 3 fixtures for each of L1–11 (`rubrics.rs`; moves into `crates/content` in Phase 6)
- [x] **Rubric tuning:** grader lab run 2026-11-10: 32 of 33 as expected. The miss: L8's correct answer got Almost because the rubric demanded "margin is not the cost of the trade" literally; reworded to "a deposit that lets you control a much bigger position (it is not the full cost)". **Re-run 2026-11-10 19:06 UTC: 33 of 33 as expected**, and all 22 Almost / Not yet replies carried a marked suggested answer (one awkward but correct L11 sentence noted)

**Exit check:** for all 11 Stage 1 lessons, correct → Got it, gap → Almost, misconception → Almost or Not yet.

## Phase 6: Content (Stage 1, L1–11) ✅
- [x] `crates/content` with `build.rs` validation + Markdown pre-rendering (ARCHITECTURE §7); 4 tests; a planted rule break fails the build
- [x] Lessons 1–11 written per CURRICULUM's authoring rules (prices near 4,100; ~5 min each at 130 wpm)
- [x] **Fact check:** each lesson lists its sources with the date checked. Confirmed pages: MT5 Help (order types, execution, trade), SIX ISO 4217 list (XAU = 959), LBMA (USD per troy ounce), World Gold Council (demand), FCA (CFD leverage 30:1–2:1; high-risk investments), NFA BASIC, ASIC registers, MT5 platform (demo). Unconfirmable claims were dropped or reworded: troy-ounce grams, CME's 100-oz contract (site unreachable; 100 oz is stated as the course's working assumption), FCA's 20:1 for gold
- [x] 15 videos, 1–2 per lesson, each confirmed via YouTube oEmbed on 2026-11-10 (title, channel)
- [x] Stage 1 drill kinds tagged per lesson (`candle_direction`, `bid_ask`, `dollar_distance`, `pip_convert`, `order_type`); the players arrive in Phase 7
- [x] App reads lessons, rubrics and grader fixtures from `telosgard-content`; the Phase 3 Lesson 1 draft and `rubrics.rs` are gone

**Exit check:** the build passes validation; each lesson takes 5–8 minutes to read; every fact has a recorded source; every video link was opened and confirmed within the last 7 days. *(Met 2026-11-10.)*

## Phase 7: Tools ✅ *(Claude's live verdicts pending the Learner's check)*
- [x] New core modules, test-first: `clock` (next overlap/rollover, week start), `daily` (same-day count and realized P/L), `adherence` (verdict prompt with Hard Rule facts, fenced learner notes, dispute section; strict reply check incl. consistency), `stage1_drills` (bid/ask, distance, pip conversion, order type). 108 unit tests
- [x] Position-size calculator (Tools) and the plan's max size suggested live in the trade form
- [x] Trading Plan builder: Hard + Soft Rules incl. Daily Loss Limit; every save is a new version; earlier versions listed read-only
- [x] Session clock on Home, Journal and the checklist (local time; updates every 30 s)
- [x] Pre-Session Checklist: news events (local time → UTC, tier), Asian range, mood, bias, zones, Scenarios, risk with sizes for planned stops; links to the BLS and Fed calendars
- [x] Journal: log a trade (Hard Rules previewed as you type, Scenario picker, optional screenshot via `assets`), log a skipped setup; trade page with Hard Rules, Claude's verdict (judged against the plan version in force), one Dispute, result after close. Trade fields can't be edited after saving
- [x] Weekly Review (this / last / two weeks ago) with a saved note
- [x] Live Readiness on real data: Readiness Runs (Run 1 starts with the first trade's balance; a breach blocks new trades until the next run is started with a fresh balance), rings, drawdown, permanent unlock record
- [x] Chart Drill player: the candle drill plus four Stage 1 drills, with all-time totals
- [x] Exit check, deterministic part: `crates/core/tests/phase7_e2e.rs` runs the whole scenario and matches the hand calculation
- [x] Local walk-through without the platform: plan → checklist → trade (8/8 Hard Rules previewed) → verdict fails safely with "Get Claude's verdict" → result → readiness, review, drills; no console errors
- [ ] **Learner check:** on the live app, log a trade and get a real verdict; dispute one; confirm everything survives a reload
- Note: screenshots are stored and shown, but not yet sent to Claude (`has_screenshot: false`); sending images is a later improvement

**Exit check:** an end-to-end scripted run: create plan v1 → log a trade with no checklist (not adherent, no dispute) → save a checklist → log an adherent trade matching a Scenario → log a trade at 17:30 New York (No-Trade Window fail) → log trades until the Daily Loss Limit is hit, then one more (fail) → dispute a Soft Rule verdict → the dashboard and Weekly Review numbers match a hand calculation.

## Phase 8: Ship Wave 1 ✅
- [x] Accessibility pass:
  - contrast measured for every text/background token pair in both themes; one failure fixed (light `up` on `bg` was 4.22 → `#2A6E27`, 4.95)
  - axe-core 4.10.2 (WCAG 2 A/AA) on 13 screens × both themes, plus populated plan, journal, trade, readiness and an open Explain-Back sheet: one violation (unlabelled review note), fixed
  - keyboard: visible focus everywhere; sheets now move focus inside when they open and close on Escape
  - reduced motion respected (CSS + the new loading screen); charts and the session clock have accessible names; colour is never the only signal (▲/▼, ✓/✗, words)
- [x] Loading screen while the WebAssembly starts (the first visit can take several seconds)
- [x] Published with capabilities `db`, `user`, `sample{images}`, `assets` (version 8); verified with `ArtifactData list`: the Learner's private folder holds their saved profile (theme, disclaimer acceptance)
- [x] Handover: the link and a 2-minute "how to use this" note (README)

**Still open (Learner checks):** Phase 7 live verdict and dispute.

## After Wave 1
- [x] Fix: finishing the quiz reset it to question 1 instead of showing the score (the lesson reader re-rendered on every progress save; it now re-renders only when the lock state changes). The same bug closed the Explain-Back sheet when a grade arrived
- [x] "Want to be quizzed on more concepts?" after the quiz: Claude writes 5 fresh questions on the lesson, or mixed with earlier passed lessons (`telosgard_core::practice`, 5 tests; strict checks, malformed questions dropped, fewer than 3 good = error). Practice never affects the Gate
- [x] Explain-Back attempts now also store the "what's missing" note
- [x] **Suggested answer:** on Almost / Not yet, Claude returns the Learner's own answer minimally rewritten to include the missing idea, additions in [[brackets]] and highlighted in the app; required by the reply check (3 tests). The Learner still rewrites it in their own words to pass
- [x] The latest grade, feedback and suggestion also show on the Explain-Back card, so they can't be lost when the sheet closes; sheets focus their first field on open

## Wave 1.5: Learning upgrades *(from the second LLM review, decided 2026-11-10)*

> Source: `Gold Trading Course Notes — LLM 2 Review`. It reviewed an earlier, different dashboard, so only the ideas that fit Telosgard are taken. Not adopted: a US futures track (the Learner trades outside the US on MT5), fresh numbers on retry, bar replay, zone-tolerance grading (the existing Wave 5 drill stays as planned), data export, dated real headlines, Lightweight Charts, Papermorph narration. Already covered: current example prices (~4,100), Dollar Moves instead of pips, real yields and central banks (Stage 3), accessibility, process-based graduation (Live Readiness), DST-correct sessions.

### Phase 9: Lessons in steps, the new Lesson 2, risk early ✅
- [x] `telosgard-core` course map v3, test-first: 40 core lessons, Live Track L41–44, `STAGES = (1,12) (13,15) (16,20) (21,23) (24,28) (29,31) (32,36) (37,40)`, Indicators after L28. Checked with `ArtifactData` before publishing: the Learner's only lesson record is `lesson-01` (passed 2026-11-10), so nothing needed migrating
- [x] Content format v2 (ARCHITECTURE §7): hook + **3–6 explanation steps** (≤ 60 words of main text each; detail in `why`; optional `widget`) + analogy, example, mistake, takeaways, videos + a 3-question **Recall Set**; stable quiz ids (`q1…`, recall `r1…`); main path 2–5 min at 130 wpm, ≤ 8 min with every "Why?" opened; the Markdown body must be empty. A planted 65-word step fails the build with a clear message
- [x] Reader v2: one screen at a time (9–12 per lesson), progress bar, Back / Next, "Why? Tell me more", Recall Set, then Quiz, Explain-Back and Gate; "Skip to the quiz" for lessons already started
- [x] Nine Widgets, checked one by one on the hidden `#widget-lab` page against hand calculations: `price_drag`, `lots_money`, `one_percent`, `stop_first` (Enter disabled until the stop is set), `short_risk` (whole $1,000 gone at 4,250, with the close-out / negative-balance note), `leverage_toggle` (50×: 12.20 oz, $20 move = ±$243.90, wiped after $82), `bid_ask`, `win_rate` (35% at 2:1 → +0.05R), `pip_convert`
- [x] **Lesson 2: The honest maths of trading**: ESMA's 74–89% of retail CFD accounts losing money and the FCA/ESMA risk-warning and negative-balance rules (both pages checked 2026-11-10), win-rate maths, process over conviction; `win_rate` widget; 2 videos confirmed via oEmbed; rubric + 3 fixtures
- [x] Lessons 1 and 3–12 rewritten into steps by a one-off migration that kept every fact, source, video, quiz and rubric. Additions: the 1% step (`one_percent`) in Lesson 8, the shorts step (`short_risk`) in Lesson 5, stop-first in Lessons 10 and 12. Two changes from the plan: lessons have **up to** 2 widgets (Lessons 3 and 11 have none; nothing there moves one number), and explanation steps are 3–6 with the analogy, example, mistake and takeaways as their own screens
- [x] Renumbered everywhere: content files, catalogue, drill labels, "Lesson 39/40" copy; the router test caught the new valid Lesson 44
- [x] **Grader lab re-run 2026-11-10 20:09 UTC: 36 of 36 as expected**, including Lesson 2 (correct → Got it, gap → Almost, misconception → Not yet, each with a marked suggested answer)

**Exit check:** `cargo test --workspace` green (124); the build rejects a step over 60 words; all 12 Stage 1 lessons render step by step with working widgets; grader lab 36 of 36 as expected. *(Met 2026-11-10.)*

### Phase 10: Retention (Spaced Review, Confidence, Question Records)
- [x] `telosgard-core` `retention` module, test-first: `QuestionRecord { answered, wrong, guessed_right, last_at }`; `record(answer_correct, confidence)`; `pick_review(records, eligible_lessons, n, seed) -> Vec<QuestionId>` weighted towards wrong (×3) and right-but-guessing (×2), only from **passed** lessons other than the current one, deterministic for a seed; a question answered right while "Sure" twice in a row drops out
- [x] Quiz: a **Sure / Guessing** choice per answer (before the answer is revealed). A right answer counts towards the Gate either way (decided 2026-11-10)
- [x] Storage: one `qrec-<lesson>` document per lesson (question id → record); written after each quiz and review
- [x] **Spaced Review** card at the start of each lesson (2–3 questions; skipped when there's nothing eligible); practice only
- [x] Question stats view for course tuning: per question, answered / wrong % / guessed %, readable by Claude Code through `ArtifactData` on `data/users/me`

*Done 2026-11-11: 9 retention tests (133 in total); records in `qrec-NN` docs; stats at the hidden `#question-stats`; Spaced Review checked on the live Artifact (version 13).*

**Exit check:** retention tests green (weighting, eligibility, determinism, drop-out rule); a guessed right answer still passes the Gate and reappears in the next lesson's Spaced Review.

### Phase 11: Drills and journal upgrades
- [x] **Calibration drill** (Practice, after Lesson 2): predict up/down on 10 seeded illustrative charts, then see the hit rate (expected ≈ 50%) and the lesson: you can't predict the next candle; you manage risk over many trades. No streaks. `telosgard-core` test: seeded rounds are reproducible; the score is a plain hit rate
- [x] Journal: **R-Multiple** on each closed trade and in the list (`telosgard-core` `r_multiple`, test-first); **Emotion Tag** (calm / FOMO / revenge / fear / overconfident) set when closing; a one-line **post-trade lesson**, separate from the pre-trade reason
- [x] Weekly Review: **Expectancy** in R (`telosgard-core` `review`, test-first), emotions shown against broken rules, and the week's post-trade lessons
- [x] Phase 7's end-to-end test extended with R, Expectancy and emotion counts against a hand calculation

*Done 2026-11-11: `calibration` module and `r_multiple` / `Emotion` in `journal`; the Weekly Review's "Average R" became Expectancy; an Emotion Tag is required when recording a result, the lesson is optional; 140 tests; published as Artifact version 14.*

**Exit check:** tests green; a closed −$100 trade risking $100 shows −1.0R; a week of +2R, −1R, −1R, +2R shows Expectancy +0.5R.

### Phase 12: Ask Claude and the risk footer
- [x] **Ask Claude about this step**: a button on each Lesson Step opens a small sheet; the question goes to Claude with the lesson title, the current and previous steps, and the rule "answer as this course's tutor, using its terms; no trade recommendations; say so when a question is beyond this lesson". Plain-text answer (bridge gains `sampleText`), answers kept for the session only, failures show a message and never invent an answer
- [x] **Risk footer** on every screen: "Education, not financial advice. Trading CFDs carries a high risk of losing money."
- [x] Accessibility re-scan (axe) on the reader v2, widgets and new sheets, both themes

*Done 2026-11-11: `tutor` module (6 tests, 144 in total); bridge `sampleText` (no cache); the sheet was checked locally (focus, empty question, the unavailable message, closing) and live by the Learner ("Why is a 60% win rate not enough?" on Lesson 2: stayed on the lesson, used 1R, no trade advice). The footer shows on every route. axe-core 4.10.2 (WCAG 2/2.1 A/AA) ran in both themes on every Lesson 1 screen, the Confidence step, Recall and the quiz (picked and revealed), the Ask sheet, the widget lab, question stats, Today, Practice, all three calibration states, Tools, Plan, a new and a closed trade, Journal, Weekly Review, Checklist and Readiness. One real violation: no `lang` on the page, because the viewer's skeleton has none; it's now set at startup. Scanning note: in a hidden tab, CSS transitions don't advance, so disable them before scanning or axe reads half-faded colours. Published as Artifact version 16.*

**Exit check:** asking about a step returns an answer that stays on the lesson; the footer shows on every route; axe finds no violations.

## Wave 1.75: Design polish *(decided 2026-11-11)*

> A polished design file in the Learner's green palette, blended with the structure of Figma's own workspace. taste-skill (github.com/leonxlnx/taste-skill, 2.5.1) is **not installed**: it targets marketing sites (hero, one section per screen, photographs, a finish-gate hook) and its house rules conflict with a learning app. Its fitting principles are folded into [`DESIGN-SYSTEM.md`](DESIGN-SYSTEM.md) v2; checking stays with our own tools. Built straight into the app, no separate mockup; each phase is published for the Learner to review. Comes before Stage 2 so new lessons are written in the final layout.

### Phase 13: Design spec, tokens and fonts ✅
- [x] `DESIGN-SYSTEM.md` v2: workspace layout, type pairing, spacing and radius system, hairlines, canvas, motion tokens, Milestone moments, the principles borrowed from taste-skill and the ones left out
- [x] Fonts: Inter (interface, body) and Newsreader (screen and lesson titles) from Google Fonts with system fallbacks; tabular figures kept
- [x] Tokens in `tailwind.css`: type scale (nothing under 13px, labels in sentence case), one radius family, a hairline, dot-grid colours per theme, motion durations and curves; contrast measured for every new text pairing
- [x] Applied app-wide without layout changes: titles in Newsreader, tracked capitals become sentence-case labels, the quiet press and hover layer on buttons, links and linked cards

*Done 2026-11-11: `DESIGN-SYSTEM.md` v2 (incl. what was borrowed from taste-skill and what wasn't, measured contrast, workspace and motion specs); Inter + Newsreader loading in the app (display sizes always Newsreader); hairline rings on every card, `press` and `lift` motion utilities, one button shape (`rounded-btn`), 13 px sentence-case labels, tab labels 13 px. axe (WCAG 2/2.1 A, AA) clean on 12 screens in both themes. Bundle 568 KB gz (budget 2 MB); font transfer sizes aren't readable cross-origin, Google serves only the Latin subsets in use. Published as Artifact version 18.*

**Exit check:** every screen renders in the new type and tokens in both themes; contrast table in the spec is filled; axe clean; bundle within budget.

### Phase 14: Workspace shell ✅
- [x] From 1024 px: left sidebar (Today, Practice, Tools, Journal; the course outline as a layers-style tree: every stage, current stage open, ✓ passed, ● current, 🔒 locked) replacing the bottom tab bar; centre canvas with a faint dot grid and a frame card with a Figma-style label; hairline borders between panels
- [x] Right panel where it adds context: Lesson (progress, Gate, Ask Claude inline, sources); Journal and trade pages (Live Readiness at a glance, today's checklist). Today, Practice and Tools use the full canvas width
- [x] Below 1024 px: one column and the bottom tab bar, as now; Ask Claude stays a sheet

*Done 2026-11-11: `screens/workspace.rs` (sidebar with the course outline, frame labels, Journal panel), `LessonPanel` (jump to any step, Gate checks, Ask Claude inline, sources) sharing the step through a `LessonUi` context; Ask Claude split into one body used by the phone sheet and the panel ("Ask Claude about this step" focuses the panel on wide screens). Sidebar 240 / panels 272–280 px from 1024, 272 / 300–320 from 1280. Checked at 1512, 1024, 768 and 390 (same-origin iframes; the browser window couldn't be resized): no horizontal scroll; keyboard order sidebar → canvas → panel. axe found locked outline rows dimmed below AA (fixed); then clean on 11 screens in both themes. Published as Artifact version 19.*

**Exit check:** every route works at 1440 / 1024 / 768 / 390 with no horizontal scroll; keyboard order follows sidebar → canvas → panel; axe clean.

### Phase 15: Screens and motion
- [x] Screen-by-screen polish (Today, lesson reader, quiz and Explain-Back, Practice and drills, Tools, Plan, Checklist, Journal, trade, Weekly Review, Readiness) to the v2 spec
- [x] Motion: the quiet layer everywhere; steps slide a few pixels; progress bars fill; **Milestone** moments (lesson passed on the Gate card, stage complete, Live Readiness); count-up numbers on stats; screen transitions on the canvas. Never for a profitable trade. All off under reduced motion

*Built 2026-11-11 (awaiting the Learner's review): `ui/moments.rs` (Moments context, Badge with a drawn ring and a green/gold burst on the Gate card, a full-canvas card for stage complete and Live Readiness, CountUp). Moments fire only from the Learner's own action (quiz or Explain-Back passing the Gate; the readiness write), never on load. Steps slide in the direction travelled; routes fade up; Readiness rings fill on arrival; count-ups on the quiz score, hit rate, Readiness, Weekly Review tiles, the progress card and the Journal panel. Screens: Today gets a two-column top (next lesson | progress + session clock) and the real lesson time; Practice cards equal height, in lesson order; lesson section headings in Newsreader; the Journal panel hides its checklist link on the checklist. `#widget-lab` replays all three moments. axe clean on 10 screens and the overlay, both themes; phone widths without horizontal scroll; bundle 604 KB gz. Published as Artifact version 20.*

**Exit check:** each screen reviewed by the Learner on the published Artifact; reduced motion shows every final state.

### Phase 16: Checks ✅
- [x] Chrome at 1440 / 1024 / 768 / 390 in both themes; axe (WCAG 2/2.1 A and AA, transitions disabled while scanning); bundle and font weight; first paint under 2 s

*Done 2026-11-11, on the Phase 15 build (Artifact version 20), in same-origin iframes at exact widths (the browser window can't be resized):*
- *Widths × themes: 16 routes (Today, Lesson 1, Practice, three drills, Tools, Plan, Journal, Readiness, Checklist, Weekly Review, new trade, skipped setup, widget lab, question stats) at 1440 / 1024 / 768 / 390 in dark and light: 128 of 128 clean.*
- *axe-core 4.10.2, WCAG 2 / 2.1 A and AA, transitions off while scanning: 0 violations in all 128.*
- *Horizontal overflow: no element past the screen edge on any of the 64 route × width pairs (checked per element, since `body` hides horizontal overflow).*
- *Weight: app bundle 604 KB gz (1.56 MB raw; budget 2 MB compressed), JS glue 7 KB gz, page 38 KB, fonts ≈ 176 KB (two Latin woff2 files; `display=swap`, so they never block content).*
- *Start-up: with files cached, lesson content is on screen 43–54 ms after the frame starts loading; the loading screen within ~15 ms. A first visit is dominated by downloading ~650 KB (≈ 0.5 s at 10 Mbit/s, ≈ 1 s at 5 Mbit/s if the host serves it compressed; ≈ 1.3 s at 10 Mbit/s if not), so under 2 s on a typical connection. Paint timings couldn't be read because the automated tab is hidden.*

**Exit check:** all checks pass and are recorded here. ✅

## Wave 1.9: Telosgard on Cloudflare, and the new look *(decided 2026-11-11)*

> The Learner asked for Postgres, Cloudflare deployment and the look of a reference screenshot (warm off-white, deep teal-green ink, an ochre accent, pill buttons, serif headlines with an italic accent phrase, small-caps eyebrows, white cards with soft shadows, tinted status chips). Decisions: **Cloudflare only** (Pages, a Worker, **D1**, R2, **Access**); the claude.ai Artifact is frozen at version 21. Cloudflare has no Postgres of its own, so D1 (SQLite) is used ([ADR 0003](adr/0003-cloudflare-pages-d1-access.md)). Fonts: **Newsreader** (display, with italic accents) and **Plus Jakarta Sans** (interface), the closest Google Fonts to the screenshot.

### Phase 17: The new look ✅
- [x] `DESIGN-SYSTEM.md` v3: light-first warm palette (off-white ground, deep teal-green ink, ochre accent), dark mode redesigned from the original greens; Plus Jakarta Sans + Newsreader; pill buttons (ochre primary, outlined secondary); small-caps eyebrows in ochre; white cards with soft shadows; tinted status chips; contrast measured for every pairing
- [x] Applied app-wide, keeping the workspace layout, the motion and the Milestone moments

*Done 2026-11-11: fonts identified by setting the reference sentence in 8 sans and 4 serif candidates (Plus Jakarta Sans and Newsreader matched); tokens rewritten light-first with dark from the original greens; pills, ochre eyebrows, tinted chips, teal progress bars, italic accent titles ("Today, one step at a time." · "Practice, without pressure." · "Tools for the plan." · "Your journal, kept honest."). 16 routes × 4 widths × 2 themes: axe clean, no overflow. Not published: the Artifact is frozen; it ships with Phase 20.*

**Exit check:** every screen in the new look at 1440 / 1024 / 768 / 390, both themes; axe clean.

### Phase 18: Cloudflare backend (local first) ✅
- [x] `cloudflare/`: a Worker with static assets (TypeScript; Cloudflare's current recommendation instead of Pages Functions) with a D1 binding: `GET/PUT/DELETE /api/docs/:id`, `GET /api/docs?kind=`, `GET /api/me`; identity from the Access JWT (verified against the team's keys), with a local-dev identity under `wrangler dev`
- [x] D1 schema and migrations: `docs (user_id, doc_id, kind, body, updated_at)`, primary key `(user_id, doc_id)`, index on `(user_id, kind)`
- [x] R2 for chart screenshots (`POST /api/files`, served from `/api/files/:id` to the owner only)
- [x] `bridge.js` Cloudflare twin with the same exports; the Rust app unchanged apart from asset URLs
- [x] Tests for the Worker (Vitest with `@cloudflare/vitest-plugin`, the renamed Workers pool): auth required, one user can't read another's documents, list by kind

*Done 2026-11-11: `cloudflare/` (wrangler 4.154.0, jose 6.2.13, vitest 4.1.12 with `@cloudflare/vitest-plugin` 1.4.9; versions at least two weeks old). 9 Worker tests green, types clean. The app's `bridge.js` now calls `/api/*` with the same exports; images at `/api/files/:id`. Checked in Chrome against `wrangler dev` with a local D1 and R2: quiz progress and Question Records saved, plan, a trade with a screenshot (served back as image/png), reload keeps everything (disclaimer stays dismissed, "Progress saved to your account"), Delete this entry removes the trade and the emptied run. Claude calls answer "not connected yet" until Phase 19.*

**Exit check:** the whole app works against `wrangler dev` with a local D1: progress, plan, checklist, journal, delete, screenshots.

### Phase 19: Claude through the Worker
- [ ] `POST /api/claude/json` and `/api/claude/text`: fixed model and output cap per call type (grading, verdict, practice, tutor), API key as a Worker secret, per-user rate limit, errors mapped to the bridge's codes (never a made-up answer)
- [ ] The grader lab re-run against the new path (36 fixtures)

**Exit check:** grading, verdicts, practice and Ask Claude work through the Worker; grader lab 36/36.

### Phase 20: Deploy and move the data
- [ ] The Learner's one-time steps, guided: Cloudflare account and `wrangler login`, the Anthropic key as a secret, Access on the site
- [ ] Deploy to Pages; copy the Learner's existing documents from the Artifact database into D1 (read with ArtifactData, imported by a script); the Artifact gets a note pointing to the new address
- [ ] Phase 16's checks repeated on the deployed site

**Exit check:** the Learner signs in through Access on the live site and sees their own progress.

## Later waves
One stage per wave (Stages 2–8, then the Live and Advanced Tracks). Each wave: review the Learner's attempt history → write the stage (CURRICULUM) → fact-check → verify videos → tune rubrics → publish to the **same** Artifact URL. Drills arrive with their stage:
- Wave 2: session drills ("which session is it?", "is this a No-Trade Window?")
- Wave 3: "hot or soft surprise → which way does gold usually go?"
- Wave 4: **candle reconstruction** (given open, high, low and close, pick the matching candle and say who won it; added from the second review)
- Wave 5: tap the level, name the structure (HH/HL/LH/LL, BOS/CHoCH), confirmation candle at a location
- Wave 6: **Fibonacci drills**: pick the valid impulse leg, place the grid, find the confluence zone
- Wave 7: ATR stop placement and sizing speed drill (target: size from any stop in under 30 seconds)

Stage numbers above refer to the stage, not the lesson number; from Phase 9 every lesson after Lesson 1 moves up by one.

## Open items
1. The Learner's live journal check for Phase 7 (a real verdict and a dispute). Test entries can now be deleted: "Delete this entry" on any entry for 24 hours after logging (two-step, permanent); if that empties the latest open Readiness Run, the run is removed too (added 2026-11-11, Artifact version 17).
