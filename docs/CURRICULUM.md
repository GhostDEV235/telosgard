# Curriculum: Gold School

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

## Video policy
- At most 2 per Lesson, placed after the written lesson and before the Quiz.
- **Allowed:** educational channels whose main product is teaching. Candidate channels to check per video: Rayner Teo, The Trading Channel, Trading Rush, Babypips (written + video), CME Group education, World Gold Council.
- **Excluded:** channels selling signals, copy-trading, "account management" or profit screenshots as their main content.
- Every URL is **opened and verified** (title, channel, still available) before it's committed, and the verification date goes in the file. Each video also stores `watch_for`, an optional `start`, and a `fallback_search` phrase.
- If a video uses "pips", the `watch_for` note states which pip size they mean in Dollar Moves.

---

## Core Track

### Stage 1: Foundations *(Wave 1, full spec)*

| # | Lesson | Objective: the Learner can… | Analogy | Real-life example | Explain-Back prompt | must_include | misconceptions |
|---|---|---|---|---|---|---|---|
| 1 | What is trading? | explain trading as profiting from a change in price, not from owning the thing | Reselling concert tickets: buy before demand rises, sell after | Buying gold at 4,100, selling at 4,120 → profit from the $20 move | "What is a trader actually trying to profit from?" | the **change** in price; buy and sell at different prices; it can lose as well as gain | trading = investing for the long term; you need to own physical gold |
| 2 | What is gold, and why does the world want it? | name 3 reasons people buy gold and connect them to price | Gold as the world's "emergency savings jar" | In periods of uncertainty (e.g. 2020, 2022), demand for "safe" assets rose (narrated generally, no invented figures) | "Why do people rush to buy gold when they're scared?" | gold holds value over time; it's a safe haven in fear or crisis; more demand → higher price | gold only goes up; gold pays interest/dividends |
| 3 | Reading "XAUUSD" | read the symbol and the quote as USD per troy ounce | A price tag at a shop: "1 ounce of gold costs $X" | XAUUSD 4,150.00 = 1 oz costs $4,150; if it moves to 4,155 gold got **$5 dearer** | "What does XAUUSD = 4,150 mean, and what does it mean when it rises?" | XAU = gold, USD = US dollar; price of 1 troy ounce in dollars; rising = gold more expensive in dollars | the number is the price of a gram or kilo; XAU is a company or stock |
| 4 | Buying vs selling | explain long and short and when each profits | Long = buying an umbrella before rain; short = selling a borrowed umbrella now and buying it back cheaper after the rainy season | Buy at 4,150 → close 4,160 = +$10 move; Sell at 4,150 → close 4,140 = +$10 move | "How can you make money when gold's price falls?" | **sell** first (go short), buy back later; profit if price falls; loss if it rises | you must own gold to sell; selling only means closing a buy |
| 5 | Bid, Ask & Spread | explain why every trade starts slightly negative and what it costs | Airport currency booth: "We buy at / We sell at" | Bid 4,150.20 / Ask 4,150.50 → spread $0.30; widens at news and in the Asian session | "Why do you start every trade slightly 'in the red'?" | buy at the ask, sell at the bid; ask > bid; the spread is the cost paid on entry | the spread is charged on close; the spread is always the same |
| 6 | Dollar Moves, points and pips | measure any move in dollars and convert others' "pips" | A ruler with three unit markings (cm/mm/m): same distance, different numbers | 4,150.00 → 4,153.00 = **$3 move** = 300 points = 30 pips ($0.10 pips). "30 pips" could mean $3, $0.30 or $30! | "A YouTuber says '50-pip stop on gold.' What do you do before trusting that?" | convert to a Dollar Move; pip size differs by source; point = $0.01, pip = $0.10 in this course | a pip is the same everywhere; pips are a money amount |
| 7 | Lots: how big is your trade? | turn a lot size and a Dollar Move into money | Buying in boxes: 1 lot = a box of 100 oz | 1 lot × $3 move = $300; 0.10 lot × $3 = $30; 0.01 lot × $3 = $3. Check your broker's contract size | "You buy 0.20 lots and gold rises $4. How much did you make, and why?" | 1 lot = 100 oz (check the broker); money = lots × 100 × Dollar Move; small lot sizes are allowed | lots are a fixed dollar amount; profit depends only on the price move |
| 8 | Leverage & margin | explain margin as a deposit and why leverage magnifies losses equally | Buying a house with a mortgage: a small deposit controls a large asset, and losses hit your deposit | 1 lot of gold at 4,150 ≈ $415,000 of gold; with 1:100 leverage the margin is ≈ $4,150. A $41.50 move against you = 1% of the position = 100% of that margin | "Why is leverage dangerous for a beginner even though it 'lets you trade bigger'?" | margin is a deposit, not the cost; leverage magnifies gains **and losses**; it can wipe the account fast; position size, not leverage, sets risk | leverage is free money; more leverage = more profit |
| 9 | Order types, stop-loss & take-profit | choose a market, limit or stop order for a given intent, with a stop-loss and take-profit attached | A shop: pay at the till now (market); leave a written offer for a lower price (limit); "buy only if it breaks above this" (stop) | Gold at 4,100: a market buy fills near the ask now; a buy limit at 4,080 waits for a dip; a buy stop at 4,120 triggers only on a rise. SL 4,090 / TP 4,130 close it automatically | "You only want to buy if gold dips to 4,080. Which order do you use, and why not a market order?" | market = now at the current price; limit = a better price, may never fill; stop = triggers once price passes a level; SL/TP close the trade automatically | a limit order always fills; a stop-loss guarantees the exact exit price |
| 10 | The real costs: spread, commission, slippage, swap | name all four costs and estimate them on a trade | A taxi ride: flag fall (spread), booking fee (commission), a traffic surprise (slippage), overnight parking (swap) | 0.10 lots with a $0.30 spread costs $3 on entry; a stop hit during news that fills $1 worse costs another $10; holding over three nights is charged swap each night at the broker's published rate | "Your trade moved $5 your way, but your account grew less than you expected. What could explain it?" | spread on entry; commission per lot (broker-dependent); slippage when price jumps; swap for each night held | costs only matter for big accounts; swap is a one-off fee; stops always fill exactly |
| 11 | Your demo account & platforms | open an MT5 demo, read XAUUSD's contract spec, and place and close a 0.01-lot order with SL and TP; open a TradingView chart | A flight simulator: real instruments, no real crash | In the MT5 symbol spec, check contract size (often 100 oz), minimum lot (0.01), swap rates and trading hours; place a 0.01-lot buy with SL and TP, then close it | "Why do you practise on a demo first, and what must you check in your broker's XAUUSD spec?" | demo = real prices, simulated money; check contract size, minimum lot, swap and hours; always attach a stop; use a regulated broker (e.g. FCA, ASIC, CFTC/NFA) | demo results guarantee live results; every broker's gold contract is the same |

**Stage 1 Chart Drills:** candle direction (up/down), find bid vs ask on a quote panel, measure the Dollar Move between two marked prices, convert "X pips" claims → Dollar Moves, pick the order type for a stated intent.

### Stage 2: Sessions & Timing *(Wave 2)*
12. **The trading week & sessions:** Sunday open to Friday close (New York time), the daily break, Asian / London / New York sessions, the LBMA auctions (10:30 and 15:00 London)
13. **The overlap, in your local time:** why 13:00–17:00 UTC is gold's prime window; the app's session clock shows it in the Learner's own timezone; the Asian high and low as levels London tests
14. **Windows to avoid:** the daily rollover, the Sunday open, the minutes around tier-1 releases (these become the **No-Trade Windows** Hard Rule)

### Stage 3: What Moves Gold *(Wave 3)*
15. **The US dollar:** the seesaw relationship (DXY) and why it isn't perfect
16. **Real yields & the Fed:** gold pays no interest, so rising real yields usually weigh on it
17. **Tier-1 news:** CPI, NFP, FOMC statement and press conference; tiers 2–3; reading the economic calendar
18. **Surprise vs forecast:** hot data → hawkish expectations → gold usually falls, and vice versa; why beginners don't trade the release candle
19. **Fear, war & central banks:** safe-haven spikes you can't schedule, and central bank buying

### Stage 4: Reading the Chart *(Wave 4)*
20. **Candlesticks:** open, high, low, close, wicks and bodies
21. **Timeframes & top-down analysis:** bias → setup → entry; the beginner stack is H4 for direction, H1 for entries (D1/H4 for swing)
22. **The five-step chart read:** choose timeframes → mark swing highs/lows → name the structure → draw levels and the session range → only then read the latest candles

### Stage 5: Price Action *(Wave 5)*
23. **Support & resistance zones:** zones not lines; a level is real after two reactions; round numbers ($50/$100) as magnets; trade the reaction, not the level
24. **Market structure:** higher highs/lows, lower highs/lows, ranges; break of structure (BOS) vs change of character (CHoCH)
25. **Confirmation candles:** pin bar, engulfing, inside bar, *only at a location*; wait for the candle to close
26. **The pullback play:** Classic Price Action in a trend: pullback to a zone → confirmation → stop beyond the wick → target the next zone
27. **The bounce play:** Classic Price Action at a zone tested twice or more

*Optional side lesson (after 27):* **Indicators, lightly:** EMAs as a direction filter (21/50 on H1, 50/200 on H4/D1), RSI; why price action comes first.

### Stage 6: Fibonacci & Confluence *(Wave 6)*
28. **Drawing Fibonacci correctly:** a clean impulse leg that broke structure; swing low → high in an uptrend
29. **What the levels mean:** 38.2 / 50 / 61.8, the 78.6 invalidation, 127.2 / 161.8 extensions as targets
30. **Confluence:** Fib + zone + round number; the combined Classic + Fib setup; never against the higher-timeframe trend

### Stage 7: Risk & Psychology *(Wave 7)*
31. **Stops beyond structure, sized by ATR:** at least 1–1.5× ATR beyond structure, never at an obvious round number; the chart decides the stop
32. **Position sizing:** the calculator, the 1% ceiling, rounding down; the stop decides the size, never the reverse
33. **Risk-to-reward & expectancy:** break-even win rates (1:2 needs > 33%); ten losses at 1% leaves ~90%
34. **Daily loss limit & drawdown:** stopping at 2–3R for the day; why −50% needs +100% to recover
35. **Bad trade vs losing trade:** FOMO, revenge trading, moving stops, overtrading after wins; a bad trade breaks rules, a losing trade follows them

### Stage 8: Plan & Practise *(Wave 8)*
36. **The Classic Price Action setup, end to end:** H4 trend from structure → H1 zone (with optional Fib confluence) → confirmation candle → stop, size and target from the plan
37. **The pre-session routine:** the in-app **Pre-Session Checklist**: calendar, Asian range, macro mood, bias, zones, written **Scenarios**, risk for the day
38. **Writing your Trading Plan:** the builder (Hard and Soft Rules)
39. **Your first Demo Trade & the weekly review:** journaling it, skipped setups, and the **Weekly Review**

## Live Track *(unlocks with Live Readiness)*
40. **Reviewing your journal:** finding your leaks in your own data
41. **Brokers & scams:** regulation, checking a licence, red flags (signal groups, "account managers", guaranteed returns)
42. **Going live, small:** 0.01 lots, 0.25–0.5% risk, the same plan, and different emotions
43. **Scaling up:** only after three live months with ≥ 90% adherence, and in small steps

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
| 1–2 | Stage 1 Foundations (open the demo account in Lesson 11) |
| 2–3 | Stage 2 Sessions & Timing |
| 3–5 | Stage 3 What Moves Gold |
| 5–7 | Stage 4 Reading the Chart |
| 7–10 | Stage 5 Price Action |
| 10–12 | Stage 6 Fibonacci & Confluence |
| 12–16 | Stage 7 Risk & Psychology |
| 16+ | Stage 8, then 50 Demo Trades over at least 8 weeks towards **Live Readiness** |
