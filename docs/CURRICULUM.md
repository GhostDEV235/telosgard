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

### Stage 4: Reading the Chart *(Wave 4)*
21. **Candlesticks:** open, high, low, close, wicks and bodies
22. **Timeframes & top-down analysis:** bias → setup → entry; the beginner stack is H4 for direction, H1 for entries (D1/H4 for swing)
23. **The five-step chart read:** choose timeframes → mark swing highs/lows → name the structure → draw levels and the session range → only then read the latest candles

### Stage 5: Price Action *(Wave 5)*
24. **Support & resistance zones:** zones not lines; a level is real after two reactions; round numbers ($50/$100) as magnets; trade the reaction, not the level
25. **Market structure:** higher highs/lows, lower highs/lows, ranges; break of structure (BOS) vs change of character (CHoCH)
26. **Confirmation candles:** pin bar, engulfing, inside bar, *only at a location*; wait for the candle to close
27. **The pullback play:** Classic Price Action in a trend: pullback to a zone → confirmation → stop beyond the wick → target the next zone
28. **The bounce play:** Classic Price Action at a zone tested twice or more

*Optional side lesson (after 28):* **Indicators, lightly:** EMAs as a direction filter (21/50 on H1, 50/200 on H4/D1), RSI; why price action comes first.

### Stage 6: Fibonacci & Confluence *(Wave 6)*
29. **Drawing Fibonacci correctly:** a clean impulse leg that broke structure; swing low → high in an uptrend
30. **What the levels mean:** 38.2 / 50 / 61.8, the 78.6 invalidation, 127.2 / 161.8 extensions as targets
31. **Confluence:** Fib + zone + round number; the combined Classic + Fib setup; never against the higher-timeframe trend

### Stage 7: Risk & Psychology *(Wave 7)*
32. **Stops beyond structure, sized by ATR:** at least 1–1.5× ATR beyond structure, never at an obvious round number; the chart decides the stop
33. **Position sizing:** the calculator, the 1% ceiling, rounding down; the stop decides the size, never the reverse
34. **Risk-to-reward & expectancy:** break-even win rates (1:2 needs > 33%); ten losses at 1% leaves ~90%
35. **Daily loss limit & drawdown:** stopping at 2–3R for the day; why −50% needs +100% to recover
36. **Bad trade vs losing trade:** FOMO, revenge trading, moving stops, overtrading after wins; a bad trade breaks rules, a losing trade follows them

### Stage 8: Plan & Practise *(Wave 8)*
37. **The Classic Price Action setup, end to end:** H4 trend from structure → H1 zone (with optional Fib confluence) → confirmation candle → stop, size and target from the plan
38. **The pre-session routine:** the in-app **Pre-Session Checklist**: calendar, Asian range, macro mood, bias, zones, written **Scenarios**, risk for the day
39. **Writing your Trading Plan:** the builder (Hard and Soft Rules)
40. **Your first Demo Trade & the weekly review:** journaling it, skipped setups, and the **Weekly Review**

## Live Track *(unlocks with Live Readiness)*
41. **Reviewing your journal:** finding your leaks in your own data
42. **Brokers & scams:** regulation, checking a licence, red flags (signal groups, "account managers", guaranteed returns)
43. **Going live, small:** 0.01 lots, 0.25–0.5% risk, the same plan, and different emotions
44. **Scaling up:** only after three live months with ≥ 90% adherence, and in small steps

## Advanced Track *(unlocks with Live Readiness)*
- **A1 Breakout & Retest:** break, retest, rejection candle; false breaks on quiet days
- **A2 Asian-Range Breakout:** marking the Asian high/low, trading the break as London opens, a fixed window and clear invalidation
- **A3 News Follow-Through:** waiting for the release whipsaw to settle, then trading the established direction with the top-down framework
- **A4–A6 Supply & Demand:** zones vs lines, fresh vs tested zones, combining zones with Classic structure
- **A7–A10 Smart Money Concepts:** liquidity and sweeps, order blocks, fair value gaps, an SMC rulebook (SMC terms vary by teacher, so each lesson states its definition)

Adopting an Advanced strategy requires a new **Trading Plan** version.

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
