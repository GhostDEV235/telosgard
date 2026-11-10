# Implementation Plan: Wave 1

> Build order for Wave 1 (the app, all tools, Stage 1). Each phase ends with an **exit check**. A failed check stops the line rather than being worked around.
> References: [PRD](PRD.md) · [Architecture](ARCHITECTURE.md) · [Design System](DESIGN-SYSTEM.md) · [Curriculum](CURRICULUM.md)

## Phase 0: Toolchain *(needs the Learner's OK: installs software on the Mac)*
- [ ] Install Rust with `rustup` (stable) and add the `wasm32-unknown-unknown` target
- [ ] `cargo install trunk` (and `wasm-bindgen-cli` if Trunk doesn't fetch it)
- [ ] Tailwind v4 CLI via npm, local to the project (`npm i -D tailwindcss @tailwindcss/cli`)
- [ ] Check crates.io and pin exact versions of `leptos`, `wasm-bindgen`, `serde`, `pulldown-cmark` and `ulid` in `Cargo.toml`
- [ ] `git init`, add a `.gitignore` (`target/`, `dist/`, `node_modules/`)

**Exit check:** `cargo --version`, `trunk --version` and `npx tailwindcss --help` all run.

## Phase 1: Proof of concept *(the go/no-go on ADR 0001)*
A throwaway Leptos page (`spike/`) with four buttons, published as a private Artifact:
- [ ] **WASM loads:** a Leptos counter renders inside the claude.ai viewer
- [ ] **Bridge → sample:** a button calls `sampleJson` and shows `{ "ok": true }` returned by Claude
- [ ] **Bridge → db/user:** a button writes `data/users/<id>/ping` and reads it back after a reload
- [ ] **YouTube:** one embedded iframe and one thumbnail-card fallback; record which works
- [ ] **Bundle size:** record the compressed `.wasm` size in release mode with `opt-level="z"`

**Exit check:** all four work in the viewer. If WASM is blocked → **stop and ask the Learner.** If only YouTube iframes are blocked → use the thumbnail fallback and note it in ARCHITECTURE §11.

## Phase 2: Domain core (`crates/core`), test-first
Write each item as a red → green → refactor cycle (`cargo test`):
- [ ] `money`: `DollarMove`, `Price`, `Lots`, `Usd` newtypes; points ↔ pips ↔ dollars conversions
- [ ] `sizing`: lot formula, **round down** to 0.01, a minimum 0.01 lot with a "risk exceeds plan" warning, contract-size parameter. Cases: $10,000 · 1% · $5 stop → 0.20 lots; $500 · 1% · $8 stop → 0.00 → "too small to trade safely"
- [ ] `gate`: `passed = quiz_best >= 80 && latest_grade == GotIt`; the unlock order of lessons
- [ ] `plan`: `TradingPlan` with `HardRules` + `SoftRules`; version increments; immutability of old versions
- [ ] `journal`: `evaluate_hard_rules(entry, plan) -> Vec<HardRuleResult>`, covering risk %, R:R, session, trades/day and stop present
- [ ] `readiness`: counts only final verdicts; first→latest ≥ 21 days; ≥ 90% adherent; drawdown against max loss; reset behaviour per PRD open question 1
- [ ] `drills`: seeded candle generator (same seed → same candles), answer checkers

**Exit check:** `cargo test -p core` is green, and every PRD FR-4/5/6/7 rule has at least one test.

## Phase 3: App shell & design system
- [ ] Workspace + Trunk + Tailwind pipeline; tokens from DESIGN-SYSTEM §6
- [ ] Theme: system default, toggle, `data-theme`, persisted
- [ ] Hash router: `#/`, `#/lesson/:n`, `#/tools/size`, `#/plan`, `#/journal`, `#/readiness`, `#/drill/:id`
- [ ] Components: StageCard, LessonRow, Reader, VideoCard, QuizCard, ExplainBackSheet, GateStatus, SegmentedControl, NumberField, CandleChart, RingProgress, Toast
- [ ] Home "Today": card stack for Stages 1–5 + the Live and Advanced Tracks (locked)
- [ ] First-launch disclaimer sheet

**Exit check:** both themes pass a visual check at 360 px and 1280 px; no horizontal scroll; keyboard focus is visible everywhere.

## Phase 4: Platform bridge & persistence
- [ ] `bridge.js` + `platform::{sample_json, user_id, db_*}` with typed errors
- [ ] `store::` repositories: profile, progress, attempts, plans, journal, drills, readiness
- [ ] Degraded mode: banners when `use()` resolves `null`; content is still fully readable

**Exit check:** in the published preview, progress survives a reload, and with the platform absent the app still renders all content.

## Phase 5: Learning loop
- [ ] Quiz flow: shuffle, instant feedback, best score saved
- [ ] Explain-Back: prompt builder (ARCHITECTURE §6.1), JSON validation, single retry on bad JSON, the three Grade UIs, attempt history
- [ ] GateStatus wiring + unlocking the next lesson; a gold "Lesson passed" moment
- [ ] **Rubric tuning:** for each Stage 1 lesson, run three fixture answers (correct / gap / misconception) and adjust rubrics until the Grades match expectations

**Exit check:** for all 9 lessons, the correct fixture → Got it, the gap fixture → Almost, and the misconception fixture → Almost or Not yet.

## Phase 6: Content (Stage 1)
- [ ] `crates/content` with `build.rs` validation + pre-rendering (ARCHITECTURE §7)
- [ ] Write lessons 1–9 per CURRICULUM's authoring rules
- [ ] Find and **verify** 1–2 videos per lesson (open each, record title, channel and the date checked)
- [ ] Stage 1 Chart Drills defined

**Exit check:** the build passes validation; each lesson takes 5–8 minutes to read; every video link was opened and confirmed within the last 7 days.

## Phase 7: Tools
- [ ] Position-size calculator (standalone + embedded in the journal form)
- [ ] Trading Plan builder with versioning (marked "you'll complete this in Lesson 24")
- [ ] Journal: form, Hard Rule evaluation, Claude Adherence Verdict (ARCHITECTURE §6.2), optional screenshot via `assets`, one Dispute, result fields after close
- [ ] Live Readiness dashboard (RingProgress) + permanent unlock record
- [ ] Chart Drill player

**Exit check:** an end-to-end scripted run: create plan v1 → log an adherent trade → log a 3%-risk trade (must be "not adherent" from the Hard Rule, with no dispute allowed) → dispute a Soft Rule verdict → the dashboard numbers match a hand calculation.

## Phase 8: Ship Wave 1
- [ ] Accessibility pass (contrast measured, reduced motion, screen-reader labels on charts)
- [ ] Publish the Artifact with capabilities (ARCHITECTURE §5); verify with one `ArtifactData list`
- [ ] Hand over the link and a 2-minute "how to use this" note

## Later waves
Each later wave runs a shorter loop: review the Learner's attempt history → write the stage (Curriculum) → verify the videos → tune the rubrics → publish to the **same** Artifact URL.

## Decisions still needed from the Learner
1. Phase 0 install approval.
2. PRD §8 Q1: does a max-loss breach reset the readiness count? (Proposed: yes.)
3. PRD §8 Q2: does a plan change keep earlier counted trades? (Proposed: yes.)
