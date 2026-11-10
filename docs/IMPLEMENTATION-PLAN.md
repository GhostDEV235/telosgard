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
- [x] Quiz flow: shuffle, instant feedback, best score saved
- [x] Explain-Back: prompt builder + strict reply check in `gold_core::grading` (7 tests), the three Grade UIs (Got it + optional tip / Almost + "what's missing" / Not yet + re-teach with a different analogy), errors with "Try again" (no auto-retry, per the platform), attempt history saved and listed
- [x] GateStatus wiring + unlocking the next lesson; a gold "Lesson passed" banner and toast
- [x] Rubrics + 3 fixtures for each of L1–11 (`rubrics.rs`; moves into `crates/content` in Phase 6)
- [ ] **Rubric tuning:** the Learner runs `#grader-lab` once (Claude needs their permission); Claude Code reads `lab/grader-latest` and adjusts rubrics until the Grades match

**Exit check:** for all 11 Stage 1 lessons, correct → Got it, gap → Almost, misconception → Almost or Not yet.

## Phase 6: Content (Stage 1, L1–11) ✅
- [x] `crates/content` with `build.rs` validation + Markdown pre-rendering (ARCHITECTURE §7); 4 tests; a planted rule break fails the build
- [x] Lessons 1–11 written per CURRICULUM's authoring rules (prices near 4,100; ~5 min each at 130 wpm)
- [x] **Fact check:** each lesson lists its sources with the date checked. Confirmed pages: MT5 Help (order types, execution, trade), SIX ISO 4217 list (XAU = 959), LBMA (USD per troy ounce), World Gold Council (demand), FCA (CFD leverage 30:1–2:1; high-risk investments), NFA BASIC, ASIC registers, MT5 platform (demo). Unconfirmable claims were dropped or reworded: troy-ounce grams, CME's 100-oz contract (site unreachable; 100 oz is stated as the course's working assumption), FCA's 20:1 for gold
- [x] 15 videos, 1–2 per lesson, each confirmed via YouTube oEmbed on 2026-11-10 (title, channel)
- [x] Stage 1 drill kinds tagged per lesson (`candle_direction`, `bid_ask`, `dollar_distance`, `pip_convert`, `order_type`); the players arrive in Phase 7
- [x] App reads lessons, rubrics and grader fixtures from `gold-content`; the Phase 3 Lesson 1 draft and `rubrics.rs` are gone

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

**Still open (Learner checks):** Phase 4 reload check · Phase 5 grader lab · Phase 7 live verdict and dispute.

## Later waves
One stage per wave (Stages 2–8, then the Live and Advanced Tracks). Each wave: review the Learner's attempt history → write the stage (CURRICULUM) → fact-check → verify videos → tune rubrics → publish to the **same** Artifact URL. Drills arrive with their stage:
- Wave 2: session drills ("which session is it?", "is this a No-Trade Window?")
- Wave 3: "hot or soft surprise → which way does gold usually go?"
- Wave 5: tap the level, name the structure (HH/HL/LH/LL, BOS/CHoCH), confirmation candle at a location
- Wave 6: **Fibonacci drills**: pick the valid impulse leg, place the grid, find the confluence zone
- Wave 7: ATR stop placement and sizing speed drill (target: size from any stop in under 30 seconds)

## Open items
1. The Learner's reload check for Phase 4.
2. The Learner runs `#grader-lab` once for Phase 5's rubric tuning.
3. The Learner's live journal check for Phase 7 (a real verdict and a dispute).
