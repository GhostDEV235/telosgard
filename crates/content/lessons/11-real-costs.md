+++
id = 11
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
id = "q1"
q = "You buy 0.10 lots with a $0.30 spread. What does the spread cost you?"
options = ["$0.30", "$3", "$30", "Nothing until you close"]
answer = 1
why = "0.10 × 100 × $0.30 = $3, paid as you open."

[[quiz]]
id = "q2"
q = "Your stop-loss fills $2 worse than its level on 0.20 lots. How much extra did slippage cost?"
options = ["$2", "$4", "$40", "$400"]
answer = 2
why = "0.20 × 100 × $2 = $40."

[[quiz]]
id = "q3"
q = "How often is swap charged?"
options = ["Once, when you open", "For each night the trade stays open", "Only on profitable trades", "Never on gold"]
answer = 1
why = "Swap is an overnight charge: hold for three nights and it’s applied three times."

[[quiz]]
id = "q4"
q = "Which cost happens because price moved fast past your order?"
options = ["Spread", "Commission", "Slippage", "Swap"]
answer = 2
why = "Slippage is the gap between the price you expected and the price you got."

[[quiz]]
id = "q5"
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

[[steps]]
heading = "1. Spread"
text = "You pay the spread every time you open a trade, and like everything else it scales with size: **spread cost = lots × 100 × spread**. A $0.30 spread on 0.10 lots is $3; on 1 lot, $30, every single trade."
[[steps]]
heading = "2. Commission"
text = "Some brokers charge a **commission** per lot, often in exchange for a narrower spread. Others charge none and earn through the spread. Neither is automatically cheaper: add both together to compare brokers."
[[steps]]
heading = "3. Slippage"
text = "**Slippage** is the gap between the price you expected and the one you got. It happens when price moves faster than orders can fill: around big news, after a weekend, in sudden spikes. Plan for it to cost you."
why = "Slippage can occasionally work in your favour, but when you hold trades through news, expect it to cost you."
[[steps]]
heading = "4. Swap"
text = "Hold a trade past the end of the trading day and your broker applies a **swap**: a small charge, or occasionally a credit, for **each night** it stays open. MetaTrader 5 shows the swap on every open position."
why = "Some brokers apply a larger swap on one particular weekday to cover the weekend, so check the schedule. For trades lasting minutes or hours, swap barely matters; for days or weeks, it can become a real part of the result."
[[steps]]
heading = "Add it up before you trade"
text = "Before a trade, tally the cost to get in, hold and get out: spread × size, plus any commission, plus swap if you'll hold overnight, plus room for slippage if news is due. If that eats much of your target, skip it."
why = "Costs don't make trading impossible. They mean small, frequent trades need especially good reasons, and that your journal should record the real result, after costs."
[[recall]]
id = "r1"
q = "0.20 lots with a $0.30 spread costs you…"
options = ["$6", "$0.60", "$60"]
answer = 0
why = "0.20 × 100 × $0.30 = $6."
[[recall]]
id = "r2"
q = "Swap is charged…"
options = ["For each night a trade stays open", "Once, when you open"]
answer = 0
why = "It repeats every night you hold."
[[recall]]
id = "r3"
q = "Slippage is most likely…"
options = ["Around big news and sudden spikes", "On quiet, slow days"]
answer = 0
why = "It happens when price moves faster than orders can fill."
+++
