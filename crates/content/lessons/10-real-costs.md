+++
id = 10
stage = 1
title = "The real costs: spread, commission, slippage, swap"
objective = "name all four costs of a trade and estimate them before you trade."
hook = "Your trade moved $5 in your favour. Why did your account grow by less than you expected?"
analogy = "A taxi ride has more than one charge: the flag fall when you get in (the spread), a booking fee (commission), the meter running in a surprise traffic jam (slippage), and overnight parking if you keep the car (swap). The fare on the sign is never quite the whole bill."
analogy_tag = "taxi fare"
example = "You buy 0.10 lots. The spread is $0.30, so getting in costs 0.10 × 100 × $0.30 = $3. During a news spike your stop-loss fills $1 worse than planned, another 0.10 × 100 × $1 = $10. You hold for three nights, and swap is charged each night at your broker’s published rate. None of these show up as the price move, but all of them come out of your account. (Illustrative figures.)"
mistake = "Thinking costs only matter for big accounts, that swap is a one-off fee, or that stops always fill exactly at their price. Costs scale with your size, swap repeats every night, and fast markets cause slippage."
takeaways = [
  "Spread: paid every time you open a trade.",
  "Commission: a fee per lot that some brokers charge.",
  "Slippage: a worse fill than expected when price moves fast.",
  "Swap: charged (or occasionally paid) for each night a trade stays open.",
]

[[videos]]
id = "Z4z3aTgfWLg"
title = "Lesson 6.1:  What is swap in forex trading?"
channel = "Financial Wars"
duration = "5:37"
watch_for = "That swap is charged per night a position stays open. The video uses currency pairs; check your broker’s swap rate for gold."
fallback_search = "what is swap in forex trading"
checked = "2026-11-10"

[[videos]]
id = "DiL2LF57znE"
title = "Trading guide - What is a Slippage?"
channel = "IG Singapore"
duration = "1:39"
watch_for = "Why the price you get can differ from the price you asked for when the market moves quickly."
fallback_search = "what is slippage in trading"
checked = "2026-11-10"

[[sources]]
claim = "MetaTrader 5 reports swap charges on open positions, and the minimum trade volume is set by the broker."
name = "MetaTrader 5 Help: Trade"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade"
checked = "2026-11-10"

[[sources]]
claim = "With market execution the broker sets the execution price, so a fill can differ from the price requested."
name = "MetaTrader 5 Help: Types of Execution"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade/general_concept/execution_types"
checked = "2026-11-10"

[[quiz]]
q = "You buy 0.10 lots with a $0.30 spread. What does the spread cost you?"
options = ["$0.30", "$3", "$30", "Nothing until you close"]
answer = 1
why = "0.10 × 100 × $0.30 = $3, paid as you open."

[[quiz]]
q = "Your stop-loss fills $2 worse than its level on 0.20 lots. How much extra did slippage cost?"
options = ["$2", "$4", "$40", "$400"]
answer = 2
why = "0.20 × 100 × $2 = $40."

[[quiz]]
q = "How often is swap charged?"
options = ["Once, when you open", "For each night the trade stays open", "Only on profitable trades", "Never on gold"]
answer = 1
why = "Swap is an overnight charge: hold for three nights and it’s applied three times."

[[quiz]]
q = "Which cost happens because price moved fast past your order?"
options = ["Spread", "Commission", "Slippage", "Swap"]
answer = 2
why = "Slippage is the gap between the price you expected and the price you got."

[[quiz]]
q = "Which statement is true?"
options = ["Costs only matter for big accounts", "Bigger trades mean bigger costs in money", "Commission is the same as swap", "The spread is refunded if you lose"]
answer = 1
why = "Every cost is multiplied by your size, just like profits and losses."

[explain_back]
prompt = "Your trade moved $5 your way, but your account grew less than you expected. What could explain it?"
must_include = ["the spread paid on entry", "commission per lot, depending on the broker", "slippage when price jumps past your order", "swap charged for each night the trade is held"]
misconceptions = ["costs only matter for big accounts", "swap is a one-off fee", "stops always fill exactly at their price"]

[[explain_back.fixtures]]
answer = "Costs ate into it: the spread you pay when you open, any commission per lot, slippage if the price jumped past my order, and swap if I held it overnight, which is charged every night."
expect = "got_it"

[[explain_back.fixtures]]
answer = "The spread and the broker's commission would take some of the profit."
expect = "almost"

[[explain_back.fixtures]]
answer = "Probably a one-off swap fee the broker takes, but costs like that only really matter for big accounts anyway."
expect = "almost_or_not_yet"
+++

A price move is not the same as money in your account. Between the two sit **costs**, and a trader who ignores them can follow every rule and still slowly lose. There are four to know.

### 1. Spread

You met this in Lesson 5. You buy at the ask and sell at the bid, so every trade starts behind by the width of the spread. Like everything else, its cost in money depends on your size:

> spread cost = lots × 100 × spread

A $0.30 spread on 0.10 lots is $3. On 1 lot it’s $30. Every single trade.

### 2. Commission

Some brokers charge a **commission** per lot traded, often in exchange for a narrower spread. Others charge no commission and earn everything through the spread. Neither is automatically cheaper; you have to add both together to compare. Your broker’s pricing page will tell you which model it uses.

### 3. Slippage

**Slippage** is the difference between the price you expected and the price you actually got. It happens when the market moves faster than orders can be filled: around big news, at the open after a weekend, or in any sudden spike. On a market order, or a stop-loss that triggers during a jump, the platform fills you at the next available price, which can be worse than you planned.

Slippage can occasionally work in your favour, but you should plan for it to cost you, especially when holding trades through news.

### 4. Swap

If a trade stays open past the end of the trading day, your broker applies a **swap** (also called rollover): a small charge, or occasionally a small credit, for holding the position overnight. It is applied **for each night** you hold, so a trade held for a week pays it several times. Some brokers apply a larger swap on one particular day of the week to cover the weekend, so check your broker’s schedule. MetaTrader 5 shows the swap on each open position, so you can see it build up.

For short trades lasting minutes or hours, swap barely matters. For trades held for days or weeks, it can become a real part of the result.

### Putting it together

Before a trade, ask: *what will it cost me to get in, to hold, and to get out?* A rough tally is enough: spread × size, plus any commission, plus swap if you’ll hold overnight, plus a little room for slippage if news is due. If those costs swallow a large share of what you’re hoping to make, the trade isn’t worth taking.

Costs don’t mean trading is impossible. They mean small, frequent trades need especially good reasons, and that your journal should record the real result, after costs, not just the price move.
