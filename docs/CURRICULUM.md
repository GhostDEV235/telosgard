# Curriculum: Gold School

> The lesson map and the content spec for authors. Terms follow [`CONTEXT.md`](../CONTEXT.md). Stage 1 is specified in full because Wave 1 ships it. Later stages are outlines, fleshed out in their own wave.

## Authoring rules (every Lesson)
1. **One concept per Lesson.** If a lesson needs a new term, either that term was taught earlier or it's defined inline in one sentence.
2. **Order inside a Lesson:** hook question → plain explanation → 💡 Analogy → 🌍 Real-life example (with gold numbers) → common mistake → Key takeaways → 🎥 Videos → Quiz → Explain-Back.
3. **Numbers:** every distance is a **Dollar Move**. Example prices are written as round illustrative numbers (2,650.00) and labelled illustrative when shown on a chart. Never present them as real historical quotes.
4. **Rubric:** each Explain-Back has 2–4 `must_include` ideas and 1–3 known `misconceptions`. The rubric checks *meaning*, not wording.
5. **Re-teach readiness:** each lesson records its `analogy_tag`, so a **Not yet** re-teach uses a different one.
6. **Quiz:** 5–8 questions, at least 2 of them applied (numbers or scenarios), not just definitions.

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
| 1 | What is trading? | explain trading as profiting from a change in price, not from owning the thing | Reselling concert tickets: buy before demand rises, sell after | Buying gold at 2,600, selling at 2,620 → profit from the $20 move | "What is a trader actually trying to profit from?" | the **change** in price; buy and sell at different prices; it can lose as well as gain | trading = investing for the long term; you need to own physical gold |
| 2 | What is gold, and why does the world want it? | name 3 reasons people buy gold and connect them to price | Gold as the world's "emergency savings jar" | In 2020 and 2022 uncertainty, demand for "safe" assets rose (narrated generally, no invented figures) | "Why do people rush to buy gold when they're scared?" | gold holds value over time; it's a safe haven in fear or crisis; more demand → higher price | gold only goes up; gold pays interest/dividends |
| 3 | Reading "XAUUSD" | read the symbol and the quote as USD per troy ounce | A price tag at a shop: "1 ounce of gold costs $X" | XAUUSD 2,650.00 = 1 oz costs $2,650; if it moves to 2,655 gold got **$5 dearer** | "What does XAUUSD = 2,650 mean, and what does it mean when it rises?" | XAU = gold, USD = US dollar; price of 1 troy ounce in dollars; rising = gold more expensive in dollars | the number is the price of a gram or kilo; XAU is a company or stock |
| 4 | Buying vs selling | explain long and short and when each profits | Long = buying an umbrella before rain; short = selling a borrowed umbrella now and buying it back cheaper after the rainy season | Buy at 2,650 → close 2,660 = +$10 move; Sell at 2,650 → close 2,640 = +$10 move | "How can you make money when gold's price falls?" | **sell** first (go short), buy back later; profit if price falls; loss if it rises | you must own gold to sell; selling only means closing a buy |
| 5 | Bid, Ask & Spread | explain why every trade starts slightly negative and what it costs | Airport currency booth: "We buy at / We sell at" | Bid 2,650.20 / Ask 2,650.50 → spread $0.30; widens at news and in the Asian session | "Why do you start every trade slightly 'in the red'?" | buy at the ask, sell at the bid; ask > bid; the spread is the cost paid on entry | the spread is charged on close; the spread is always the same |
| 6 | Dollar Moves, points and pips | measure any move in dollars and convert others' "pips" | A ruler with three unit markings (cm/mm/m): same distance, different numbers | 2,650.00 → 2,653.00 = **$3 move** = 300 points = 30 pips ($0.10 pips). "30 pips" could mean $3, $0.30 or $30! | "A YouTuber says '50-pip stop on gold.' What do you do before trusting that?" | convert to a Dollar Move; pip size differs by source; point = $0.01, pip = $0.10 in this course | a pip is the same everywhere; pips are a money amount |
| 7 | Lots: how big is your trade? | turn a lot size and a Dollar Move into money | Buying in boxes: 1 lot = a box of 100 oz | 1 lot × $3 move = $300; 0.10 lot × $3 = $30; 0.01 lot × $3 = $3. Check your broker's contract size | "You buy 0.20 lots and gold rises $4. How much did you make, and why?" | 1 lot = 100 oz (check the broker); money = lots × 100 × Dollar Move; small lot sizes are allowed | lots are a fixed dollar amount; profit depends only on the price move |
| 8 | Leverage & margin | explain margin as a deposit and why leverage magnifies losses equally | Buying a house with a mortgage: a small deposit controls a large asset, and losses hit your deposit | 1 lot of gold at 2,650 ≈ $265,000 of gold; with 1:100 leverage the margin is ≈ $2,650. A $26.50 move against you = 1% of the position = 100% of that margin | "Why is leverage dangerous for a beginner even though it 'lets you trade bigger'?" | margin is a deposit, not the cost; leverage magnifies gains **and losses**; it can wipe the account fast; position size, not leverage, sets risk | leverage is free money; more leverage = more profit |
| 9 | Market hours & sessions | name the 3 sessions and when gold is most active | A city's rush hour: busy in the London–New York overlap, quiet at night | Gold trades ~23 h/day, 5 days a week; the London–NY overlap and US data releases bring the biggest moves; spreads are widest at the daily open | "When is gold usually most active, and why does that matter to you?" | Asia / London / New York; most movement in the London–NY overlap; wider spreads in quiet hours | gold trades the same all day; gold trades on weekends |

**Stage 1 Chart Drills:** candle direction (up/down), find bid vs ask on a quote panel, measure the Dollar Move between two marked prices, convert "X pips" claims → Dollar Moves.

### Stage 2: What Moves Gold *(Wave 2)*
10. **The US dollar:** the seesaw relationship (DXY) and why it isn't perfect
11. **Interest rates and the Fed:** why higher real yields usually weigh on gold
12. **News events:** CPI and NFP; spikes and spread widening; why beginners stay flat around news
13. **Fear, war and central banks:** safe-haven flows and central bank buying

### Stage 3: Reading Price Action *(Wave 3)*
14. **Candlesticks:** open, high, low, close, wicks and bodies
15. **Timeframes:** top-down analysis (Daily → H4 → H1 → M15)
16. **Support & resistance:** levels as "floors and ceilings"; role reversal
17. **Market structure:** higher highs and higher lows, lower highs and lower lows; trend vs range; break of structure
18. **Confirmation candles:** pin bar, engulfing, inside bar, *only meaningful at a level*

*Optional side lesson (after 18):* Indicators, lightly (moving averages, RSI) and why price action comes first.

### Stage 4: Risk *(Wave 4)*
19. **Stop-loss & take-profit:** placing stops beyond structure, not at round numbers
20. **Position sizing:** the calculator, the 1% rule, worked gold examples
21. **Risk-to-reward & win rate:** the break-even win rate maths (1:2 needs > 33%)
22. **Drawdown:** why −50% needs +100% to recover; consecutive-loss streaks

### Stage 5: Plan & Practise *(Wave 5)*
23. **Classic Price Action setup:** H4 trend from structure → H1 key level → confirmation candle at the level, in the trend's direction → stop beyond the candle/level → target at the next level, with minimum risk-to-reward from the plan
24. **Writing your Trading Plan:** the builder (Hard and Soft Rules)
25. **Your first Demo Trade:** MT5 demo set-up, order ticket walkthrough, journaling it

## Live Track *(unlocks with Live Readiness)*
26. **Trading psychology:** fear of missing out, revenge trading, overconfidence after wins
27. **Reviewing your journal:** finding your leaks in your own data
28. **Brokers & scams:** regulation, checking a licence, red flags (signal groups, "account managers", guaranteed returns)
29. **Going live, small:** minimum size, the same plan, and expecting different emotions

## Advanced Track *(unlocks with Live Readiness)*
- **A1–A3 Supply & Demand:** zones vs lines, fresh vs tested zones, combining zones with Classic structure
- **A4–A7 Smart Money Concepts:** liquidity, order blocks, fair value gaps, a consistent definition set chosen for this course (SMC terms vary by teacher, so each lesson states its definition)

Adopting an Advanced method requires a new **Trading Plan** version.
