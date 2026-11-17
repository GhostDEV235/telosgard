# Curriculum: Telosgard

> The lesson map and the content spec for authors. Terms follow [`CONTEXT.md`](../CONTEXT.md). Stage 1 is specified in full because Wave 1 ships it. Revised 2026-11-10 to fold in an outside 7-phase lesson plan (sessions, news, Fibonacci, risk routine) per the Learner's decisions. Later stages are outlines, fleshed out in their own wave.

## Authoring rules (every Lesson)
1. **One concept per Lesson.** If a lesson needs a new term, either that term was taught earlier or it's defined inline in one sentence.
2. **Order inside a Lesson:** hook question → plain explanation → 💡 Analogy → 🌍 Real-life example (with gold numbers) → common mistake → Key takeaways → 🎥 Videos → Quiz → Explain-Back.
3. **Numbers:** every distance is a **Dollar Move**. Example prices are round illustrative numbers near the price level at the time of writing (e.g. 4,100.00 in late 2026) and labelled illustrative when shown on a chart. Never present them as real historical quotes.
4. **Rubric:** each Explain-Back has 2–4 `must_include` ideas and 1–3 known `misconceptions`. The rubric checks *meaning*, not wording.
5. **Re-teach readiness:** each lesson records its `analogy_tag`, so a **Not yet** re-teach uses a different one.
6. **Quiz:** 5–8 questions, at least 2 of them applied (numbers or scenarios), not just definitions.
7. **Times:** written in UTC and New York time; the app renders them in the **Learner's device timezone** (with daylight saving). Never hard-code one city.
8. **Facts:** every factual claim (session hours, event times, contract specs, typical moves) is checked against a primary source before it ships: CME Group, LBMA, the World Gold Council, the Federal Reserve, or the US statistics agencies' release calendars. Broker and prop-firm blogs are leads, not sources. A figure that can't be checked becomes a range or is dropped.
9. **Steps (from Wave 1.5):** a Lesson is read as **Lesson Steps**: a hook question, 3–6 explanation steps (one idea each, at most 60 words of main text, deeper detail behind "Why?"), then the analogy, example, mistake and takeaways as their own screens. Lessons have up to 2 **Widgets** (move one number, watch one result; any widget that takes a trade asks for the stop first) and ends with a 3-question **Recall Set** before the Quiz.
10. **Risk as a reflex:** from the Lots lesson onwards, every example that opens a trade names its stop and what it would cost if hit.

## Video policy
- At most 2 per Lesson, placed after the written lesson and before the Quiz.
- **Allowed:** educational channels whose main product is teaching. Candidate channels to check per video: Rayner Teo, The Trading Channel, Trading Rush, Babypips (written + video), CME Group education, World Gold Council.
- **Excluded:** channels selling signals, copy-trading, "account management" or profit screenshots as their main content.
- Every URL is **opened and verified** (title, channel, still available) before it's committed, and the verification date goes in the file. Each video also stores `watch_for`, an optional `start`, and a `fallback_search` phrase.
- If a video uses "pips", the `watch_for` note states which pip size they mean in Dollar Moves.

---

## Core Track

### Stage 1: Foundations *(Wave 1, full spec; renumbered 2026-11-10 for the new Lesson 2)*

Additions from the second review (Phase 9): Lesson 5 gains a step on shorts (price has no ceiling, so a short's loss keeps growing until a stop or the broker's margin close-out ends it; many retail CFD accounts have negative-balance protection, so don't claim losses beyond the account). Lesson 8 gains a step on deciding the loss first: about 1% of the account per trade. Lesson 2's facts are checked against the FCA/ESMA risk-warning rules before shipping.

| # | Lesson | Objective: the Learner can… | Analogy | Real-life example | Explain-Back prompt | must_include | misconceptions |
|---|---|---|---|---|---|---|---|
| 1 | What is trading? | explain trading as profiting from a change in price, not from owning the thing | Reselling concert tickets: buy before demand rises, sell after | Buying gold at 4,100, selling at 4,120 → profit from the $20 move | "What is a trader actually trying to profit from?" | the **change** in price; buy and sell at different prices; it can lose as well as gain | trading = investing for the long term; you need to own physical gold |
| 2 | The honest maths of trading | explain why most beginners lose, and why a trader can be wrong often and still make money | A shop that sells most items at a small loss but a few at a large profit can still make money overall; what matters is the totals, not each sale | Ten trades risking $100 each: 5 losses (−$500) and 5 wins of $200 (+$1,000) = +$500, despite being right only half the time | "Can you lose more trades than you win and still make money? How?" | most retail CFD accounts lose money (brokers must warn about this); a win rate alone doesn't decide results, the size of wins vs losses matters; small losses and a repeatable process beat being right | being right most of the time is what makes money; trading is a quick way to get rich |
| 3 | What is gold, and why does the world want it? | name 3 reasons people buy gold and connect them to price | Gold as the world's "emergency savings jar" | In periods of uncertainty (e.g. 2020, 2022), demand for "safe" assets rose (narrated generally, no invented figures) | "Why do people rush to buy gold when they're scared?" | gold holds value over time; it's a safe haven in fear or crisis; more demand → higher price | gold only goes up; gold pays interest/dividends |
| 4 | Reading "XAUUSD" | read the symbol and the quote as USD per troy ounce | A price tag at a shop: "1 ounce of gold costs $X" | XAUUSD 4,150.00 = 1 oz costs $4,150; if it moves to 4,155 gold got **$5 dearer** | "What does XAUUSD = 4,150 mean, and what does it mean when it rises?" | XAU = gold, USD = US dollar; price of 1 troy ounce in dollars; rising = gold more expensive in dollars | the number is the price of a gram or kilo; XAU is a company or stock |
| 5 | Buying vs selling | explain long and short and when each profits | Long = buying an umbrella before rain; short = selling a borrowed umbrella now and buying it back cheaper after the rainy season | Buy at 4,150 → close 4,160 = +$10 move; Sell at 4,150 → close 4,140 = +$10 move | "How can you make money when gold's price falls?" | **sell** first (go short), buy back later; profit if price falls; loss if it rises | you must own gold to sell; selling only means closing a buy |
| 6 | Bid, Ask & Spread | explain why every trade starts slightly negative and what it costs | Airport currency booth: "We buy at / We sell at" | Bid 4,150.20 / Ask 4,150.50 → spread $0.30; widens at news and in the Asian session | "Why do you start every trade slightly 'in the red'?" | buy at the ask, sell at the bid; ask > bid; the spread is the cost paid on entry | the spread is charged on close; the spread is always the same |
| 7 | Dollar Moves, points and pips | measure any move in dollars and convert others' "pips" | A ruler with three unit markings (cm/mm/m): same distance, different numbers | 4,150.00 → 4,153.00 = **$3 move** = 300 points = 30 pips ($0.10 pips). "30 pips" could mean $3, $0.30 or $30! | "A YouTuber says '50-pip stop on gold.' What do you do before trusting that?" | convert to a Dollar Move; pip size differs by source; point = $0.01, pip = $0.10 in this course | a pip is the same everywhere; pips are a money amount |
| 8 | Lots: how big is your trade? | turn a lot size and a Dollar Move into money | Buying in boxes: 1 lot = a box of 100 oz | 1 lot × $3 move = $300; 0.10 lot × $3 = $30; 0.01 lot × $3 = $3. Check your broker's contract size | "You buy 0.20 lots and gold rises $4. How much did you make, and why?" | 1 lot = 100 oz (check the broker); money = lots × 100 × Dollar Move; small lot sizes are allowed | lots are a fixed dollar amount; profit depends only on the price move |
| 9 | Leverage & margin | explain margin as a deposit and why leverage magnifies losses equally | Buying a house with a mortgage: a small deposit controls a large asset, and losses hit your deposit | 1 lot of gold at 4,150 ≈ $415,000 of gold; with 1:100 leverage the margin is ≈ $4,150. A $41.50 move against you = 1% of the position = 100% of that margin | "Why is leverage dangerous for a beginner even though it 'lets you trade bigger'?" | margin is a deposit, not the cost; leverage magnifies gains **and losses**; it can wipe the account fast; position size, not leverage, sets risk | leverage is free money; more leverage = more profit |
| 10 | Order types, stop-loss & take-profit | choose a market, limit or stop order for a given intent, with a stop-loss and take-profit attached | A shop: pay at the till now (market); leave a written offer for a lower price (limit); "buy only if it breaks above this" (stop) | Gold at 4,100: a market buy fills near the ask now; a buy limit at 4,080 waits for a dip; a buy stop at 4,120 triggers only on a rise. SL 4,090 / TP 4,130 close it automatically | "You only want to buy if gold dips to 4,080. Which order do you use, and why not a market order?" | market = now at the current price; limit = a better price, may never fill; stop = triggers once price passes a level; SL/TP close the trade automatically | a limit order always fills; a stop-loss guarantees the exact exit price |
| 11 | The real costs: spread, commission, slippage, swap | name all four costs and estimate them on a trade | A taxi ride: flag fall (spread), booking fee (commission), a traffic surprise (slippage), overnight parking (swap) | 0.10 lots with a $0.30 spread costs $3 on entry; a stop hit during news that fills $1 worse costs another $10; holding over three nights is charged swap each night at the broker's published rate | "Your trade moved $5 your way, but your account grew less than you expected. What could explain it?" | spread on entry; commission per lot (broker-dependent); slippage when price jumps; swap for each night held | costs only matter for big accounts; swap is a one-off fee; stops always fill exactly |
| 12 | Your demo account & platforms | open an MT5 demo, read XAUUSD's contract spec, and place and close a 0.01-lot order with SL and TP; open a TradingView chart | A flight simulator: real instruments, no real crash | In the MT5 symbol spec, check contract size (often 100 oz), minimum lot (0.01), swap rates and trading hours; place a 0.01-lot buy with SL and TP, then close it | "Why do you practise on a demo first, and what must you check in your broker's XAUUSD spec?" | demo = real prices, simulated money; check contract size, minimum lot, swap and hours; always attach a stop; use a regulated broker (e.g. FCA, ASIC, CFTC/NFA) | demo results guarantee live results; every broker's gold contract is the same |

**Stage 1 Chart Drills:** candle direction (up/down), find bid vs ask on a quote panel, measure the Dollar Move between two marked prices, convert "X pips" claims → Dollar Moves, pick the order type for a stated intent.

### Stage 2: Sessions & Timing *(Wave 2, written 2026-11-12)*
13. **The trading week & sessions:** Sunday open to Friday close (New York time), the daily break, Asian / London / New York sessions, the LBMA auctions (10:30 and 15:00 London)
14. **The overlap, in your local time:** why the London–New York overlap (roughly 8 am–noon New York time: 13:00–17:00 UTC in winter, 12:00–16:00 UTC in summer, shifting for a few weeks when US and UK clocks change on different dates) is usually gold's best window for a beginner; the app's session clock shows it in the Learner's own timezone; the Asian high and low as levels London tests
15. **Windows to avoid:** the daily rollover, the Sunday open, the minutes around tier-1 releases (these become the **No-Trade Windows** Hard Rule)

### Stage 3: What Moves Gold *(Wave 3, written 2026-11-12)*
16. **The US dollar:** the seesaw relationship (DXY: six currencies, fixed weights, about 58% European) and why it isn't perfect; context for the checklist, never a signal
17. **Real yields & the Fed:** gold pays no interest (opportunity cost); real yield ≈ yield − expected inflation, measured by the 10-year TIPS yield; rising real yields usually weigh on gold; eight FOMC meetings a year move yields
18. **Tier-1 news:** CPI and NFP at 8:30 am New York, the FOMC statement at 2:00 pm and press conference about 2:30 pm; tier 2 (GDP, PCE, retail sales at 8:30); reading the economic calendar first thing in the Pre-Session Checklist
19. **Surprise vs forecast:** previous / forecast / actual; the surprise is actual vs forecast, not vs last month; hot data → higher rates expected → gold usually falls, soft data the reverse; higher unemployment than forecast is *soft*; the reaction can flip; never trade the release candle. Widget `surprise_meter`, drill "Hot or soft?"
20. **Fear, war & central banks:** safe-haven spikes you can't schedule (the stop is the only protection), and central-bank buying above 1,000 tonnes a year in 2022–2024 (World Gold Council) as background, not a signal

Written from the Learner's attempt history: their first answers named only the winning side, so every Stage 3 rubric asks for the counter-case ("usually, not always"), and each "almost" fixture is the one-sided answer.

### Stage 4: Reading the Chart *(Wave 4, written 2026-11-13)*
21. **Candlesticks:** open, high, low and close; the body (open to close: who won the period) and the wicks (the high and low); size as strength; a candle is a record, not a forecast. Widget `candle_builder`, drill "Build the candle"
22. **Timeframes & top-down analysis:** M5/H1/H4/D1 as the same trades grouped differently (four M15 candles make one H1); bias → entry; the beginner pair is H4 for direction, H1 for entries (D1/H4 for swing), a course choice, not a market law; when timeframes disagree the higher one wins or you stand aside. Widget `candle_merge`
23. **The five-step chart read:** choose timeframes → mark swing highs/lows → name the structure (up, down, sideways; Lesson 25 adds the names) → draw levels and the Asian range → only then read the latest candles; the read ends in a plan, not a signal

Written from the attempt history: the reviewer's first drafts in Stages 1–2 left out the limit or the downside (the stop, the loss if price rises, "may never fill") and added it on the second try, as the first Learner did in Lesson 1. So each Stage 4 rubric asks for the limit ("a record, not a forecast"; "the higher timeframe wins"; "context, not a signal"), and each "almost" fixture is the answer without it. Facts checked against CME Group Education (candle anatomy and chart timeframes, read in the browser on 2026-11-13).

### Stage 5: Price Action *(Wave 5, written 2026-11-13)*
24. **Support & resistance zones:** zones a few dollars wide, not lines; worth trusting after two reactions; round numbers ($50/$100) as levels; broken levels flip; trade the reaction, not the level. Drill "Find the level"
25. **Market structure:** HH/HL and LH/LL, ranges; BOS (a break with the trend, judged on closes) vs CHoCH (a break against it: a warning, not yet a new trend). Drill "Name the structure"
26. **Confirmation candles:** pin bar (long wick about two thirds of the range or more, small other wick), engulfing (body covers the previous body), inside bar; *only at a location*; wait for the close. Drill "Name the candle"
27. **The pullback play:** trend on H4 → pullback to a zone → confirmation closes → stop beyond the wick and zone → target the next zone, at least 2:1 → size for about 1%; and when to skip
28. **The bounce play:** a zone tested twice (double bottom/top), best in a range or with the H4 trend; same discipline; a close beyond the zone ends it

*Optional side lesson (after 28):* **Indicators, lightly:** EMAs as a direction filter (21/50 on H1, 50/200 on H4/D1), RSI; why price action comes first. *Not written yet: it has no lesson number, so it needs a small change to how lessons are stored first.*

Written from the attempt history (still no attempts past Stage 2): each rubric asks for the limit (levels can break; a CHoCH is only a warning; a pattern only counts at a location; skip below 2:1; a level can break on the third test), and each "almost" fixture leaves it out. Facts checked against CME Group Education (Support and Resistance; Technical Patterns: Reversals; Chart Types), read in the browser on 2026-11-13.

### Stage 6: Fibonacci & Confluence *(Wave 6, written 2026-11-13)*
29. **Drawing Fibonacci correctly:** the last clean impulse leg that broke structure, on the trend timeframe; swing low → high in an uptrend (high → low in a downtrend), wick to wick; redraw on a new extreme, delete when the leg's start breaks. Drill "Pick the leg"
30. **What the levels mean:** 38.2 / 50 / 61.8 as shallow, medium and deep (CME Group's wording), 50–61.8 watched as a zone; a close beyond 78.6 ends the pullback play (a course rule); 127.2 / 161.8 extensions, measured from the leg's start, as targets; levels work only because many traders watch them. Drill "Place the level"
31. **Confluence:** two or more independent reasons at one zone (Fib + old support/resistance + round number); the Classic + Fib setup end to end; never against the higher-timeframe trend; more lines isn't more truth. Drill "Find the confluence"

Written from the attempt history (still no attempts past Stage 2): each rubric asks for the limit (a backwards grid gives wrong levels; a level is not a signal and 78.6 ends the play; confluence never overrides the trend), and each "almost" fixture leaves it out. Facts checked against CME Group Education (Fibonacci Retracements and Extensions; Support and Resistance) and Fidelity's Learning Center (Fibonacci Retracement), read in the browser on 2026-11-13. CME lists extension ratios of 1.38, 1.5 and 1.61; 127.2% is kept as this course's first target and labelled as such.

### Stage 7: Risk & Psychology *(Wave 7, written 2026-11-16)*
32. **Stops beyond structure, sized by ATR:** the chart decides the stop (beyond the confirmation wick and the zone); ATR(14) on the entry timeframe measures the noise; about 1–1.5× ATR beyond the structure (a course range); off obvious round numbers, where stops cluster; too wide for 2:1 means skip, never squeeze. Drill "Place the stop"
33. **Position sizing:** the stop decides the size; lots = (balance × risk %) ÷ (stop × 100 oz), the broker's contract size checked; 1% ceiling; round down to 0.01; too small to trade means no trade. Drill "Size it in 30 seconds" (timed against a 30-second target)
34. **Risk-to-reward & expectancy:** break-even win rate 1 ÷ (1 + ratio): 50% at 1:1, ~33% at 2:1, 25% at 3:1; expectancy in R; ten losses at 1% leave ~90%, at 5% ~60%
35. **Daily Loss Limit & drawdown:** 2R, never more than 3R, set in the Pre-Session Checklist; −10% needs +11%, −20% +25%, −50% +100%; drawdown and the Readiness Run's maximum total loss
36. **Bad trade vs losing trade:** a losing trade followed the plan, a bad trade broke a rule whatever it made; FOMO, revenge trading, moving the stop away, overtrading after wins; grade the process

Written from the attempt history (still no attempts past Stage 2): each rubric asks for the limit (skip rather than squeeze the stop; too small means no trade; expectancy only shows over many trades; the limit holds however good the next setup looks; a winning rule-break is still a bad trade), and each "almost" fixture leaves it out. Facts checked against CME Group Education (Proper Position Size; Controlling Risk; Misconceptions of Taking Losses), Fidelity's Learning Center (Average True Range) and Federal Reserve Bank of New York Staff Report 125 (Osler: stop-loss orders cluster just beyond round numbers; the disposition effect), read on 2026-11-16. 10 videos verified by oEmbed.

### Stage 8: Plan & Practise *(Wave 8, written 2026-11-16)*
37. **The Classic Price Action setup, end to end:** H4 trend from structure → an H1 zone written in that day's Scenarios (Fibonacci confluence optional) → a confirmation candle closes → stop 1–1.5 ATR beyond the structure, target at least 2:1, size from the calculator → last checks (No-Trade Windows, Daily Loss Limit, trades per day); any no means skip and log it
38. **The pre-session routine:** the in-app **Pre-Session Checklist** (Journal tab): news and its No-Trade Windows, the Asian range and a one-line mood, the H4 bias, 3–5 zones, 1–3 written **Scenarios**, the day's risk; a Hard Rule on any day you trade
39. **Writing your Trading Plan:** the builder (Tools tab): Hard Rules with the course's values (1%, 2:1, 2R, 2 trades a day, London and New York, ~10% max total loss), the always-on rules, Soft Rules judged by Professor Aurel, versions changed only after a review. Drill "Spot the broken rule"
40. **Your first Demo Trade & the Weekly Review:** logging a trade, the verdict and disputes, the exit, Emotion Tag and lesson, skipped setups, the **Weekly Review** note (one thing to keep, one to stop), and the four conditions of **Live Readiness**

Written from the attempt history (still no attempts past Stage 2): each rubric asks for the limit (one missing link means no trade; a vague Scenario isn't one; the plan is never edited to excuse a trade; Live Readiness doesn't need profit), and each "almost" fixture leaves it out. Facts checked against CME Group Education's "Building a Trade Plan" course (Risk Management and Your Trade Plan; Trading Strategies in Your Trade Plan; Keep a Trade Log) and the BLS and Federal Reserve release calendars, read on 2026-11-16; app details checked against the Checklist, Plan, Trade, Journal and Weekly Review screens. 8 videos verified by oEmbed. The Trading Plan builder's defaults now match the course (minimum reward-to-risk 2, Daily Loss Limit 2R; they were 1.5 and 3R).

## Live Track *(unlocks with Live Readiness; written 2026-11-16)*
41. **Reviewing your journal:** group trades by session, Scenario type, Emotion Tag and rule broken; compare average R; a leak is a clearly negative group or one holding the rule breaks; about 20 trades before concluding; one rule change at a time, as a new plan version
42. **Brokers & scams:** trading against the dealer (CFTC); checking a licence on the regulator's own register with the contact details listed there (FCA, NFA BASIC, ASIC); the CFTC's red flags (social media and dating apps, private chat, guaranteed returns, crypto-only deposits, no address, fees to withdraw); never share a login or let an "account manager" trade
43. **Going live, small:** only after Live Readiness, with money you could lose entirely; the same plan as a new version with risk at 0.25–0.5%, 0.01 lots; real costs and emotions; an early test withdrawal
44. **Scaling up:** after three live months at ≥ 90% adherence, +0.25% steps each held a month at ≥ 90%, a 1% ceiling; step down one level after a ~10% drawdown or if adherence slips

Stored as stage 9 (the dashboard labels it "Live Track"). Facts checked against the CFTC's customer advisory "Eight Things You Should Know Before Trading Forex", the FCA Warning List page, NFA BASIC, ESMA and CME Group Education (Proper Position Size; Controlling Risk; Keep a Trade Log), read on 2026-11-16. 7 videos verified by oEmbed, including the CFTC's own.

## Advanced Track *(unlocks with Live Readiness; A1–A3 written 2026-11-17)*
- **A1 Breakout & retest:** a break is an H1 close beyond a zone respected at least twice; no chasing: wait for the retest of the broken zone and a confirmation candle; stop 1–1.5 × ATR beyond its wick (back inside the old range), next zone at ≥ 2:1; distrust quiet-day breaks and breaks against H4; a close back inside the range kills it
- **A2 Asian-range breakout:** mark the H1 high and low from the Asian session's start to the London open (3 am New York); trade only an H1 close beyond it from 3 to 6 am New York, preferably with H4; stop just beyond the range's middle, target ≥ 2:1 before the next H4 zone; skip ranges too wide for that, breaks against H4, London bank holidays
- **A3 News follow-through:** after the No-Trade Window, wait for the first H1 candle after the release to close (FOMC: let the 2:30 press conference start); a direction holds one way and breaks H1 structure, ideally with H4 and the surprise; join it on a pullback with confirmation; stop on the current (bigger) ATR, so a smaller size; still ≥ 2:1
- **A4–A6 Supply & Demand:** zones vs lines, fresh vs tested zones, combining zones with Classic structure
- **A7–A10 Smart Money Concepts:** liquidity and sweeps, order blocks, fair value gaps, an SMC rulebook (SMC terms vary by teacher, so each lesson states its definition)

Adopting an Advanced strategy requires a new **Trading Plan** version.

Stored as stage 10 under content numbers 101–110 (A1–A10; the Indicators side lesson is 100), so the app, the dashboard and the Worker can key them like numbered lessons. A1–A3 checked against CME Group Education (Trend and Continuation Patterns; Support and Resistance), the World Gold Council, the Federal Reserve, the *Journal of Futures Markets* (2001, "What moves the gold market?") and the *International Review of Financial Analysis* (2024, gold's intraday reaction to FOMC shocks), read on 2026-11-17. 6 videos verified by oEmbed.

## Suggested pace *(shown on Home; never a deadline)*
| Weeks | Stages |
|---|---|
| 1–2 | Stage 1 Foundations (open the demo account in Lesson 12) |
| 2–3 | Stage 2 Sessions & Timing |
| 3–5 | Stage 3 What Moves Gold |
| 5–7 | Stage 4 Reading the Chart |
| 7–10 | Stage 5 Price Action |
| 10–12 | Stage 6 Fibonacci & Confluence |
| 12–16 | Stage 7 Risk & Psychology |
| 16+ | Stage 8, then 50 Demo Trades over at least 8 weeks towards **Live Readiness** |
