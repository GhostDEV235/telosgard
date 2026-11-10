# PRD: Gold School

> Product requirements for an interactive Gold (XAUUSD) trading course for one complete beginner.
> Vocabulary follows [`CONTEXT.md`](../CONTEXT.md). Bold terms are defined there.

## 1. Problem

The **Learner** wants to trade gold and knows nothing about it. Free material (YouTube, Telegram, "signal" groups) is:
- **out of order:** it assumes knowledge the Learner doesn't have;
- **unchecked:** nobody confirms the Learner understood before moving on;
- **inconsistent:** the same word, such as "pip", means different things from one source to the next;
- **risk-last:** strategy is taught before risk management, so beginners lose money before they learn to protect it.

## 2. Goal

Take the **Learner** from zero to placing disciplined **Demo Trades** under a written **Trading Plan**. Every concept is understood, and the understanding verified, before the next is introduced. Real-money trading is unlocked only by demonstrated discipline (**Live Readiness**).

### Success measures
| Measure | Target |
|---|---|
| Core Track completed | All **Lessons** passed through their **Gate** |
| Explain-Back quality | ≥ 80% of **Explain-Backs** graded **Got it** within 2 attempts |
| Demo discipline | **Live Readiness** reached: 20 **Journal Entries**, ≥ 3 weeks, ≥ 90% **Plan Adherence**, no max-loss breach |
| Terminology | Learner converts any outside "pips" claim into a **Dollar Move** correctly (checked in a Stage 1 Quiz) |

### Non-goals
- No live market data, signals or trade recommendations. The app teaches; it never says "buy now."
- No broker integration. Demo Trades are placed by the Learner in MT5 and then journaled by hand.
- No multi-user features, leaderboards or social features. There is one Learner.
- No guarantee of profitability, and no financial advice. This is stated on first launch and in the Live Track.

## 3. User

One Learner. They are a complete beginner, motivated to trade, and they know a professional trader whose main method is price action. They will use TradingView for charts and an MT5 demo account for orders. They will use the app on a Mac and possibly a phone.

## 4. Scope

### 4.1 Course structure
| Track | Contents | Unlocks |
|---|---|---|
| **Core Track** | Stage 1 Foundations (L1–9), Stage 2 What Moves Gold (L10–13), Stage 3 Reading Price Action (L14–18), Stage 4 Risk (L19–22), Stage 5 Plan & Practise (L23–25) | Sequentially, by **Gate** |
| **Live Track** | L26 Psychology, L27 Journal Review, L28 Brokers & Scams, L29 Going Live Small | **Live Readiness** |
| **Advanced Track** | Supply & Demand, then **SMC**, both layered on **Classic Price Action** | **Live Readiness** |
| Optional side lesson | Indicators (moving averages, RSI) | After Stage 3 |

The full lesson map is in [`CURRICULUM.md`](CURRICULUM.md).

### 4.2 Wave 1 (this build)
- The complete app: card-stack home, lesson reader, **Gates**, Claude grading, saved progress, theming.
- All four tools: position-size calculator, **Trading Plan** builder, Journal + Live Readiness dashboard, **Chart Drills** engine.
- **Stage 1 (L1–9) fully written**, with verified videos. Stages 2–5, the Live Track and the Advanced Track appear as locked cards with titles.

Later waves add one stage each, written when the Learner is near it and informed by their Wave 1 results (which analogies landed, and where they got **Almost**).

## 5. Functional requirements

### FR-1 Lesson
- FR-1.1 Each **Lesson** has: a title, a learning objective, a plain-language body (5–8 minutes of reading), at least one analogy, at least one real-life example, a "Key takeaways" list, and 0–2 videos.
- FR-1.2 Videos appear *after* the written body. Each has a one-line "what to watch for" note and an optional start timestamp.
- FR-1.3 All prices and distances in lesson text are written as **Dollar Moves**.

### FR-2 Gate
- FR-2.1 A **Lesson** is passed when the **Quiz** score is ≥ 80% **and** the latest **Explain-Back** is graded **Got it**. The two can be completed in either order.
- FR-2.2 The next **Lesson** stays locked until its predecessor is passed. Lessons cannot be skipped.
- FR-2.3 Retries are unlimited for both parts, with no penalty and no cooldown.
- FR-2.4 Quiz questions and their options are shuffled on each attempt.

### FR-3 Explain-Back grading
- FR-3.1 The Learner writes a free-text answer to the Lesson's Explain-Back prompt.
- FR-3.2 Claude grades it against that Lesson's rubric (the key ideas required and the known misconceptions) and returns a **Grade**:
  - **Got it:** a short affirmation, plus one optional sharpening note.
  - **Almost:** names the specific missing or incorrect idea. The Learner rewrites.
  - **Not yet:** re-teaches the concept with an analogy *different* from the lesson's. The Learner retries.
- FR-3.3 Every attempt is stored (answer, Grade, feedback, timestamp), so later waves can be adjusted to the Learner.
- FR-3.4 If Claude is unavailable (not granted, rate-limited, or an error), the Explain-Back shows a clear message and a retry button. It never auto-passes.

### FR-4 Position-size calculator
- FR-4.1 Inputs: account balance (USD), risk % per trade, stop distance (**Dollar Move**), and contract size (default 100 oz, editable).
- FR-4.2 Output: lot size rounded **down** to 0.01, the actual money at risk, and the actual risk %.
- FR-4.3 Formula: `lots = floor_0.01( balance × risk% / (stop_distance × contract_size) )`.
- FR-4.4 Available standalone and embedded in the Journal Entry form.

### FR-5 Trading Plan
- FR-5.1 The guided builder is introduced in Lesson 24. It is available in Wave 1 so the tool can be tested, but marked "you'll fill this in at Lesson 24."
- FR-5.2 A plan has **Hard Rules** (numeric, checked by calculation) and **Soft Rules** (judgement, checked by Claude):
  - Hard Rules: risk % per trade (max), max total loss % of starting demo balance, min risk-to-reward, allowed sessions, max trades per day, stop-loss required (always on).
  - Soft Rules: setup definition (for **Classic Price Action**: higher-timeframe trend, level, confirmation candle), plus free-text rules.
- FR-5.3 Plans are **versioned**. Saving creates a new version; old versions are read-only. Each **Journal Entry** records the plan version in force.

### FR-6 Journal & Adherence
- FR-6.1 Journal Entry fields: date/time, session, direction, entry price, stop-loss price, take-profit price, lot size, account balance before, setup reason (free text), optional chart screenshot, and result (exit price, P/L in $, balance after) once closed.
- FR-6.2 When an entry is submitted, the app:
  1. evaluates every **Hard Rule** in Rust, deterministically;
  2. sends the entry, the Soft Rules and the Hard Rule results to Claude;
  3. stores an **Adherence Verdict**: yes or no, with each failed rule named and explained.
- FR-6.3 If any Hard Rule fails, the verdict is "not adherent" without depending on Claude. Claude still writes the explanation.
- FR-6.4 **Dispute**: the Learner may dispute a verdict once, with written reasoning. Claude re-judges, and the second verdict is final. Hard Rule failures cannot be disputed.
- FR-6.5 The trade fields of a Journal Entry are locked once a verdict exists. The result fields can be filled in later.

### FR-7 Live Readiness dashboard
- FR-7.1 Shows: counted entries (x/20), adherence % (final verdicts only), the days between the first and latest counted entry (target ≥ 21), and current drawdown against the plan's max total loss.
- FR-7.2 **Live Readiness** is reached when all four conditions hold at once. Reaching it unlocks the Live Track and the Advanced Track permanently.
- FR-7.3 Entries without a final verdict don't count.

### FR-8 Chart Drills
- FR-8.1 Drills are rendered on in-app SVG candle charts drawn from **illustrative** data that is generated deterministically from a seed. Every chart is labelled "Illustrative, not real market data."
- FR-8.2 Drill types: tap the candle (pattern recognition), tap the level (support or resistance), name the structure (higher high, higher low, lower high, lower low), and trend direction.
- FR-8.3 Drills give instant right/wrong feedback with an explanation. They are practice, not part of any **Gate**.
- FR-8.4 Wave 1 ships the engine plus drills for Stage 1 concepts (reading a candle's direction, bid/ask on a chart, dollar-move distance between two prices). Price-action drills come with Stage 3.

### FR-9 Look & feel
- FR-9.1 Apple-style card stack home ("Today" tab style): one card per stage, with progress, Continue and lock state.
- FR-9.2 Dark-first theme from the Learner's palette, plus a light theme; follows the system setting, with a manual toggle that is remembered.
- FR-9.3 Up/profit = leaf green, down/loss = clay red, gold = achievements only. See [`DESIGN-SYSTEM.md`](DESIGN-SYSTEM.md).
- FR-9.4 Works from 360 px phone width to desktop, with no horizontal scrolling.

### FR-10 Persistence
- FR-10.1 All progress, attempts, plans and journal entries persist across sessions and devices for the signed-in Learner.
- FR-10.2 The Learner's data is private to them.

## 6. Non-functional requirements
- **Honesty:** never fabricate prices, charts or videos. Illustrative data is labelled. Every video link is verified to exist before shipping.
- **Safety:** a first-launch disclaimer (education, not advice). The Live Track covers scams. No content recommends a specific trade.
- **Performance:** first meaningful paint in under 2 s on a typical connection. WebAssembly bundle ≤ 2 MB compressed.
- **Accessibility:** WCAG AA contrast in both themes, full keyboard navigation, respects reduced motion, colour is never the only signal (up/down also uses ▲/▼ and labels).
- **Resilience:** the app renders and the content is readable even if Claude or the database is unavailable. Only grading and saving degrade.

## 7. Risks
| Risk | Mitigation |
|---|---|
| WebAssembly blocked on claude.ai | Phase 1 proof of concept tests it before any content is built ([ADR 0001](adr/0001-rust-wasm-on-claude-artifact.md)). YouTube embeds are confirmed blocked by the platform, so videos link out |
| Claude grading is inconsistent | Per-lesson rubric, structured JSON output, and Hard Rules computed in Rust, not by Claude |
| The Learner games their own grading | Accepted. See [ADR 0002](adr/0002-client-side-grading-trust-model.md) |
| Video links rot | Each video stores a fallback search phrase; the link is checked again whenever its stage is revised |

## 8. Resolved questions
1. **Max-loss breach** → the Live Readiness count resets to 0 and the Learner starts a fresh demo balance. Lessons stay unlocked. *(Decided 2026-11-09.)*
2. **Plan change mid-count** → earlier trades keep counting; each was judged against the plan version in force at the time. *(Decided 2026-11-09.)*
