+++
id = 33
stage = 7
title = "Position sizing"
objective = "turn a balance, a risk of at most 1% and a stop distance into a lot size, rounded down, in under 30 seconds, and know when a trade is too small to take."
hook = "Two traders take the same trade with the same stop. One loses $96 when it fails, the other loses $1,600. What did they do differently?"
analogy = "Think of a recipe that serves four when you're cooking for two. You don't change the oven or the timing; you scale the ingredients. In trading, the chart sets the stop, and you scale the size so the loss fits your plan."
analogy_tag = "scaling a recipe"
example = "Lesson 32's trade: the stop is $16 from the entry. Your balance is $10,000 and you risk 1%: $100. One lot loses $16 × 100 oz = $1,600 if the stop is hit. $100 ÷ $1,600 = 0.0625, rounded down to 0.06 lots. If the stop is hit you lose $96, just under your 1%. (Illustrative prices.)"
mistake = "Doing it backwards: picking a size you like (\"0.50 lots feels right\") and then moving the stop to make the loss look acceptable. The stop comes from the chart; the size is the only number you're allowed to change."
takeaways = [
  "The stop decides the size, never the reverse.",
  "Lots = (balance × risk %) ÷ (stop distance × 100 oz), using your broker's contract size.",
  "Round down to the nearest 0.01 lot, so you never risk more than planned.",
  "This course's ceiling is 1% of the balance per trade; if even 0.01 lot risks more, the trade is too small to take.",
]
drills = ["size_it"]

[[steps]]
heading = "The stop comes first"
text = "Sizing starts after the stop is placed (Lesson 32). You need two numbers: the **stop distance** in dollars, and the **money you're willing to lose**. The size is whatever makes those two agree. A wide stop gets a small size; a tight stop, a bigger one."
why = "CME Group's position-size lesson puts it the same way: to size a position you must know where your stop is and what share of the account you're willing to risk."

[[steps]]
heading = "Risk at most 1%"
text = "Decide the money at risk as a share of the balance. This course's ceiling is **1%**: $100 on $10,000, $50 on $5,000. Smaller is always allowed. Because it's a percentage, your risk shrinks when the account shrinks and grows only as it grows."
why = "CME Group suggests new traders risk a modest 1% to 3% per trade. This course takes the bottom of that range, so a losing streak stays survivable; Lesson 35 shows why."
widget = "one_percent"

[[steps]]
heading = "The sizing sum"
text = "**Lots = money at risk ÷ (stop distance × 100)**. The 100 is the ounces in one lot (Lesson 8). $100 at risk with a $16 stop: one lot would lose $1,600, so $100 ÷ $1,600 = **0.0625 lots**."
why = "Check your broker's contract size first: if one lot is 10 oz at your broker, the sum uses 10 instead of 100. The Tools tab's calculator lets you set it."

[[steps]]
heading = "Round down, always"
text = "Platforms trade in steps of **0.01 lots**, so 0.0625 becomes **0.06**, never 0.07. Rounding down risks a little less than planned ($96); rounding up risks more ($112). The calculator in the **Tools** tab does the sum and the rounding for you."
why = "0.06 lots × 100 oz = 6 oz; × $16 = $96. At 0.07 lots, 7 oz × $16 = $112, which breaks the 1% ceiling."

[[steps]]
heading = "Too small to trade"
text = "On a small account a wide stop can fail the test even at **0.01 lots**, the smallest size. $500 at 1% is $5, but 0.01 lots with an $8 stop loses $8 (1.6%). Then the answer is **no trade**, not a tighter stop."
why = "The calculator flags this as too small to trade safely. A setup with a closer, honest stop may come along; this one isn't yours."

[[videos]]
id = "VFVcvWpnbd4"
title = "How to Measure LOT SIZE For GOLD Trading. XAUUSD FREE Position Size Calculator Explained"
channel = "VasilyTrader"
duration = "4:27"
watch_for = "Working out a gold lot size from account size, risk and stop. He measures the stop in his platform's own units; turn it into dollars of price first, as in Lesson 7."
fallback_search = "xauusd lot size calculation"
checked = "2026-11-16"

[[videos]]
id = "q5UiDAk1740"
title = "Forex Trading: Risk Management And Position Sizing (Video 6 of 13)"
channel = "Rayner Teo"
duration = "9:49"
watch_for = "Why the stop comes first and the size second. His examples are currency pairs; the logic is the same for gold with 100 oz per lot."
fallback_search = "risk management position sizing explained"
checked = "2026-11-16"

[[sources]]
claim = "To determine position size you must know where you're placing your stop and the percentage or dollar amount of the account you're willing to risk; new traders are best off risking a modest 1%–3% of the account on a single trade; once the stop is set, the trade's risk per contract is known and the number of contracts follows."
name = "CME Group Education: Proper Position Size"
url = "https://www.cmegroup.com/education/courses/trade-and-risk-management/proper-position-size.html"
checked = "2026-11-16"

[[sources]]
claim = "With a fixed-percentage rule, the money at risk on each trade shrinks as the account shrinks: at 1%, $500 on $50,000, then $495, $490 and so on through a losing streak, which slows the account's decline."
name = "CME Group Education: Controlling Risk"
url = "https://www.cmegroup.com/education/courses/trade-and-risk-management/controlling-risk.html"
checked = "2026-11-16"

[[quiz]]
id = "q1"
q = "Which number do you change to make the loss fit your plan?"
options = ["The size", "The stop", "The entry", "The target"]
answer = 0
why = "The stop comes from the chart; only the size is yours to adjust."

[[quiz]]
id = "q2"
q = "Balance $5,000, risk 1%, stop $10 away. What size?"
options = ["0.05 lots", "0.50 lots", "0.10 lots", "0.01 lots"]
answer = 0
why = "$50 at risk; one lot loses $10 × 100 = $1,000; $50 ÷ $1,000 = 0.05."

[[quiz]]
id = "q3"
q = "The sum gives 0.0875 lots. What do you trade?"
options = ["0.08 lots", "0.09 lots", "0.10 lots", "0.0875 lots"]
answer = 0
why = "Round down to the 0.01 step so you never risk more than planned."

[[quiz]]
id = "q4"
q = "Balance $10,000, risk 1%, stop $7 away. How much do you lose if the stop is hit?"
options = ["$98, at 0.14 lots", "$100 exactly", "$700", "$105, at 0.15 lots"]
answer = 0
why = "$100 ÷ $700 = 0.1428… → 0.14 lots; 14 oz × $7 = $98."

[[quiz]]
id = "q5"
q = "A $500 account, 1% risk, and the chart needs an $8 stop. What's the answer?"
options = ["No trade: even 0.01 lots would lose $8, 1.6%", "Tighten the stop to $5", "Trade 0.01 lots anyway", "Trade without a stop"]
answer = 0
why = "When the smallest size breaks the ceiling, the trade is too small to take."

[[quiz]]
id = "q6"
q = "Your broker's gold contract is 10 oz per lot. Balance $10,000, 1%, stop $5. What size?"
options = ["2.00 lots", "0.20 lots", "20.00 lots", "0.02 lots"]
answer = 0
why = "One lot loses $5 × 10 = $50; $100 ÷ $50 = 2.00. Always check the contract size."

[[recall]]
id = "r1"
q = "Which comes first?"
options = ["The stop, then the size", "The size, then the stop", "Neither: guess both"]
answer = 0
why = "The stop decides the size, never the reverse."

[[recall]]
id = "r2"
q = "Lots = …"
options = ["(balance × risk %) ÷ (stop distance × 100)", "balance ÷ stop distance", "risk % × 100"]
answer = 0
why = "Money at risk divided by what one lot loses at the stop."

[[recall]]
id = "r3"
q = "When the sum isn't a whole 0.01 step, you…"
options = ["Round down", "Round up", "Round to the nearest"]
answer = 0
why = "Down, so the loss stays at or under the plan."

[explain_back]
prompt = "How do you work out your position size, and what do you do when it comes out too small?"
must_include = ["the stop comes from the chart first and decides the size, never the reverse", "lots equal the money at risk, at most 1% of the balance, divided by the stop distance times the ounces per lot, usually 100, rounded down to 0.01", "if even 0.01 lots would risk more than the plan, the trade is too small and you skip it instead of tightening the stop"]
misconceptions = ["you pick a size first and then fit the stop around it", "rounding up is fine because it's close"]

[[explain_back.fixtures]]
answer = "First the chart gives me the stop, then I size from it, never the other way round. I take at most 1% of my balance as the money at risk and divide it by the stop distance times 100 oz, or whatever my broker's contract is, then round down to 0.01 lots. If even 0.01 lots would lose more than 1%, the trade is too small for my account and I skip it rather than squeeze the stop."
expect = "got_it"

[[explain_back.fixtures]]
answer = "I take 1% of my balance and divide it by the stop distance times 100, then round down to 0.01 lots."
expect = "almost"

[[explain_back.fixtures]]
answer = "I always trade 0.50 lots and just move my stop closer if the loss looks too big."
expect = "almost_or_not_yet"
+++
