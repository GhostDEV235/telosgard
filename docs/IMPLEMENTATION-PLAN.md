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
- [ ] **Bridge → sample** (test 2b): first real use is Explain-Back grading in Phase 5
- [x] **YouTube:** iframes and thumbnails are blocked by the platform → link-out VideoCards
- [x] **Bundle size:** spike 37 KB gzipped; full app 175 KB gzipped

## Phase 2: Domain core (`crates/core`, package `gold-core`), test-first ✅
- [x] `money`, `sizing`, `gate`, `course`, `plan`, `journal`, `readiness`, `drills` (56 tests)

## Phase 3: App shell & design system ✅
- [x] Workspace + Trunk + Tailwind pipeline; tokens from DESIGN-SYSTEM §6
- [x] Theme: system default, toggle, `data-theme`, persisted (localStorage now; `profile.theme` in Phase 4)
- [x] Plain-hash router: `#today`, `#lesson-5`, `#practice`, `#drill-direction`, `#size`, `#plan`, `#journal`, `#readiness`
- [x] Components: StageCard, LessonRow, Reader, VideoCard, QuizCard, ExplainBackSheet, GateStatus, SegmentedControl, NumberField, CandleChart, RingProgress, Toast
- [x] Home "Today" card stack; first-launch disclaimer
- [x] Fix: `html`/`body` styles un-layered so the viewer's reset can't override them (ARCHITECTURE §9)

## Phase 3b: Realign to course map v2 ✅ *(test-first in `gold-core`)*
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
- [x] Race-safe saves: a lesson save merges with what's stored (`LessonProgress::merge`, tested in `gold-core`); progress made before the load finishes is saved once it lands
- [x] Wired now: lesson progress, theme + disclaimer in `profile`, drill totals. Attempts, plans, trades, checklists and runs are wired in Phases 5 and 7
- [x] Degraded mode: a status chip on Home (Connecting / Saved / Not saving in this view / sign in / couldn't save); with no platform the app renders all content (checked locally, no console errors)
- [x] Published with capabilities `db`, `user`, `sample{images}`, `assets`; the live page reports "Progress saved to your account"
- [ ] **Learner check:** pass the Lesson 1 quiz (or answer a drill), reload, and confirm it's still there

**Exit check:** in the published preview, progress survives a reload, and with the platform absent the app still renders all content.

## Phase 5: Learning loop
- [ ] Quiz flow: shuffle, instant feedback, best score saved
- [ ] Explain-Back: prompt builder (ARCHITECTURE §6.1), JSON validation, single retry on bad JSON, the three Grade UIs, attempt history
- [ ] GateStatus wiring + unlocking the next lesson; a gold "Lesson passed" moment
- [ ] **Rubric tuning:** for each Stage 1 lesson, three fixture answers (correct / gap / misconception); adjust rubrics until the Grades match

**Exit check:** for all 11 Stage 1 lessons, correct → Got it, gap → Almost, misconception → Almost or Not yet.

## Phase 6: Content (Stage 1, L1–11)
- [ ] `crates/content` with `build.rs` validation + pre-rendering (ARCHITECTURE §7)
- [ ] Write lessons 1–11 per CURRICULUM's authoring rules (prices near the current level; times in UTC/NY rendered locally)
- [ ] **Fact check:** every factual claim checked against a primary source (CURRICULUM rule 8) with the source and date recorded in the lesson file; the outside plan's links are leads only
- [ ] Find and **verify** 1–2 videos per lesson (open each, record title, channel and the date checked)
- [ ] Stage 1 Chart Drills defined (incl. "pick the order type")

**Exit check:** the build passes validation; each lesson takes 5–8 minutes to read; every fact has a recorded source; every video link was opened and confirmed within the last 7 days.

## Phase 7: Tools
- [ ] Position-size calculator (standalone + embedded in the journal form)
- [ ] Trading Plan builder with versioning, incl. Daily Loss Limit (marked "you'll complete this in Lesson 38")
- [ ] **Session clock:** current session, next overlap and next rollover in local time (FR-12.2); on Home and in the checklist
- [ ] **Pre-Session Checklist:** dated form (FR-11.1), link-out to an economic calendar, sizes pre-computed for planned stops
- [ ] Journal: form (links to the day's checklist; "skipped setup" entries), Hard Rule evaluation, Claude Adherence Verdict incl. the Scenario Soft Rule (ARCHITECTURE §6.2), optional screenshot via `assets`, one Dispute, result fields after close
- [ ] **Weekly Review** screen (FR-11.4)
- [ ] Live Readiness dashboard (rings x/50, %, x/56 days) + permanent unlock record
- [ ] Chart Drill player

**Exit check:** an end-to-end scripted run: create plan v1 → log a trade with no checklist (not adherent, no dispute) → save a checklist → log an adherent trade matching a Scenario → log a trade at 17:30 New York (No-Trade Window fail) → log trades until the Daily Loss Limit is hit, then one more (fail) → dispute a Soft Rule verdict → the dashboard and Weekly Review numbers match a hand calculation.

## Phase 8: Ship Wave 1
- [ ] Accessibility pass (contrast measured, reduced motion, screen-reader labels on charts and the session clock)
- [ ] Publish the Artifact with capabilities (ARCHITECTURE §5); verify with one `ArtifactData list`
- [ ] Hand over the link and a 2-minute "how to use this" note

## Later waves
One stage per wave (Stages 2–8, then the Live and Advanced Tracks). Each wave: review the Learner's attempt history → write the stage (CURRICULUM) → fact-check → verify videos → tune rubrics → publish to the **same** Artifact URL. Drills arrive with their stage:
- Wave 2: session drills ("which session is it?", "is this a No-Trade Window?")
- Wave 3: "hot or soft surprise → which way does gold usually go?"
- Wave 5: tap the level, name the structure (HH/HL/LH/LL, BOS/CHoCH), confirmation candle at a location
- Wave 6: **Fibonacci drills**: pick the valid impulse leg, place the grid, find the confluence zone
- Wave 7: ATR stop placement and sizing speed drill (target: size from any stop in under 30 seconds)

## Open items
1. The Learner's reload check for Phase 4.
