+++
id = 15
stage = 2
title = "Windows to avoid"
objective = "name the three No-Trade Windows and explain why each is dangerous for a beginner."
hook = "What do gold's daily break, its Sunday reopening and a big US data release have in common?"
analogy = "Think of crossing a road. Most of the time you look both ways and cross. But at a blind corner, or when a lorry is already coming, you wait, however much of a hurry you're in. No-Trade Windows are gold's blind corners: a few minutes when you can't see what's coming."
analogy_tag = "blind corner"
example = "It's Wednesday, 8:27 am New York time, and US inflation (CPI) comes out at 8:30. You spot a setup, but the rule says wait until 8:35. At 8:30 gold jumps $15 in seconds, then falls back $10. Waiting cost you nothing; a trade in those seconds could have filled dollars away from your price. (Illustrative prices.)"
mistake = "Believing a stop-loss fully protects you in these windows. When price jumps, your stop can fill well beyond its level. That's called slippage, and it can turn a planned $100 loss into $150."
takeaways = [
  "No new trades during the daily rollover, 5–6 pm New York time.",
  "No new trades in the first hour after the Sunday open.",
  "No new trades from 5 minutes before to 5 minutes after a tier-1 US release (CPI, NFP, FOMC).",
  "These are Hard Rules: the journal checks every Demo Trade's entry time against them.",
]
drills = ["no_trade"]

[[steps]]
heading = "The daily rollover"
text = "At **5 pm New York time** the trading day ends and open positions roll over to the next day. Many traders step away, the market thins out and **spreads often widen**. Brokers also charge or pay **swap** on positions held through it. Don't open new trades from 5 to 6 pm."
widget = "session_clock"
why = "Gold futures pause for an hour at 5 pm New York. CFD brokers may keep quoting, but prices are thin. Some brokers charge three days of swap on Wednesday night to cover the weekend; check your broker's rollover table."

[[steps]]
heading = "The Sunday open"
text = "Over the weekend news keeps happening, but gold doesn't trade. When it reopens at **6 pm New York time on Sunday**, price can **gap**: jump straight from Friday's close to a new level. Spreads are often wide for a while. Wait one hour before trading."
why = "A gap means there were no trades in between, so a stop-loss can't fill at its level; it fills at the first price available after the reopening."

[[steps]]
heading = "Tier-1 US releases"
text = "The biggest US reports (inflation **CPI**, jobs **NFP** and the Fed's **FOMC** rate decision) can move gold many dollars in seconds. Spreads widen and orders slip. Stay out from **5 minutes before to 5 minutes after** each one; your Pre-Session Checklist lists them."
why = "CPI and NFP are released at 8:30 am New York time; FOMC decisions at 2:00 pm, with a press conference at about 2:30 pm. You'll learn what these reports mean, and why they move gold, in Stage 3."

[[steps]]
heading = "Slippage: why stops aren't magic"
text = "A stop-loss becomes a market order when it's hit. If price jumps past it, it fills at the **next available price**, which can be worse. That's **slippage**. It's the reason these windows are rules, not suggestions."
why = "Some brokers sell guaranteed stops for a fee. This course doesn't rely on them; it simply avoids the moments when slippage is most likely."

[[steps]]
heading = "They're Hard Rules"
text = "The three **No-Trade Windows** are part of your Trading Plan's **Hard Rules**. When you journal a Demo Trade, the app checks its entry time automatically. A trade inside a window breaks your plan, even if it makes money."
why = "Profit doesn't make a trade disciplined. Live Readiness counts trades that followed your plan, not trades that happened to win."

[[videos]]
id = "bLLj88oGlDs"
title = "What is a Rollover Event When Trading?"
channel = "Plus500"
duration = "1:35"
watch_for = "What happens to an open position at the daily rollover, and why it can be charged or paid."
fallback_search = "what is rollover in trading"
checked = "2026-11-12"

[[videos]]
id = "DiL2LF57znE"
title = "Trading guide - What is a Slippage?"
channel = "IG Singapore"
duration = "1:39"
watch_for = "Why an order can fill at a different price from the one you asked for, and when that's most likely."
fallback_search = "what is slippage in trading"
checked = "2026-11-12"

[[sources]]
claim = "Gold futures trade Sunday–Friday 6:00 pm–5:00 pm ET with a 60-minute break each day beginning at 5:00 pm ET."
name = "CME Group: E-mini Gold contract specs"
url = "https://cmegroup.com/markets/metals/precious/e-mini-gold.contractSpecs.html"
checked = "2026-11-12"

[[sources]]
claim = "Rollover is applied to positions open at the 5 pm New York close; a swap is charged or credited."
name = "FOREX.com: Rollover FAQs"
url = "https://forex.com/en-us/help-and-support/rollover"
checked = "2026-11-12"

[[sources]]
claim = "Some brokers apply three days of rollover on Wednesdays for gold (XAU/USD) to cover the weekend."
name = "FXCM: CFD rollover calendar"
url = "https://www.fxcm.com/eu/help/cfd-trading-cfd-rollover-calendar/"
checked = "2026-11-12"

[[sources]]
claim = "US CPI and the Employment Situation (jobs report) are scheduled at 8:30 am Eastern Time."
name = "US Bureau of Labor Statistics: release schedule"
url = "https://www.bls.gov/schedule/2026/11_sched_list.htm"
checked = "2026-11-12"

[[sources]]
claim = "FOMC policy statements are released at 2:00 pm Eastern Time, and the Chair's news conference begins at about 2:30 pm."
name = "Federal Reserve: FOMC statement and press conference timing"
url = "https://www.federalreserve.gov/newsevents/pressreleases/monetary20130313a.htm"
checked = "2026-11-12"

[[quiz]]
id = "q1"
q = "Which of these is a No-Trade Window in this course?"
options = ["The first hour of the London session", "The first hour after the Sunday open", "Every Friday", "The LBMA auctions"]
answer = 1
why = "Price can gap and spreads are often wide just after gold reopens on Sunday, so you wait an hour."

[[quiz]]
id = "q2"
q = "US CPI is released at 8:30 am New York time. When is the earliest you can open a trade?"
options = ["8:30 am", "8:31 am", "8:35 am", "Not until the next day"]
answer = 2
why = "The window runs from 5 minutes before to 5 minutes after the release, so 8:35 am is the earliest."

[[quiz]]
id = "q3"
q = "Why can gold gap at the Sunday open?"
options = ["News happened over the weekend while it wasn't trading", "Brokers reset prices every Sunday", "The spread is zero then", "Tokyo opens first"]
answer = 0
why = "Two days of news are priced in at once when trading resumes, so the first price can be far from Friday's close."

[[quiz]]
id = "q4"
q = "You bought at 4,100.00 with a stop at 4,095.00. Price jumps straight from 4,100.00 to 4,090.00. Where can your stop fill?"
options = ["Exactly 4,095.00, always", "Around 4,090.00, worse than 4,095.00", "4,100.00", "It doesn't fill at all"]
answer = 1
why = "The stop becomes a market order and fills at the next available price, here around 4,090.00. That extra $5 is slippage."

[[quiz]]
id = "q5"
q = "A demo trade opened at 5:20 pm New York time on a Tuesday made a profit. Did it follow the plan?"
options = ["Yes: it made money", "No: it was opened inside the rollover window", "Yes: rollover only matters on Fridays", "Only if the spread was small"]
answer = 1
why = "5–6 pm New York is a No-Trade Window. Profit doesn't change whether the trade followed the rules."

[[quiz]]
id = "q6"
q = "What happens at the daily rollover?"
options = ["The trading day ends, swaps are applied and spreads often widen", "The market closes for the whole night", "Gold's price resets", "The LBMA auction runs"]
answer = 0
why = "At 5 pm New York the trading day rolls over: positions are charged or paid swap, and thin trading often widens spreads."

[[recall]]
id = "r1"
q = "When is the daily rollover window?"
options = ["5–6 pm New York time", "8–9 am London time", "Midnight UTC"]
answer = 0
why = "The trading day ends at 5 pm New York time."

[[recall]]
id = "r2"
q = "How long do you wait after the Sunday open?"
options = ["One hour", "Five minutes", "All of Monday"]
answer = 0
why = "The first hour after the Sunday reopening is a No-Trade Window."

[[recall]]
id = "r3"
q = "What is slippage?"
options = ["An order filling at a worse price than you asked for", "A broker's monthly fee", "The gap between two sessions"]
answer = 0
why = "When price jumps, orders, stops included, fill at the next available price."

[explain_back]
prompt = "Name the three No-Trade Windows and explain why a beginner should stay out of them."
must_include = ["the daily rollover around 5–6 pm New York time", "the first hour after the Sunday open, when price can gap", "5 minutes either side of a tier-1 US release such as CPI, NFP or FOMC", "spreads widen and stops can slip, so a loss can be bigger than planned"]
misconceptions = ["a stop-loss always fills exactly at its price", "news releases are the best time to trade because prices move fast"]

[[explain_back.fixtures]]
answer = "Don't open trades during the daily rollover from 5 to 6 pm New York time, in the first hour after gold reopens on Sunday because price can gap, or from 5 minutes before to 5 minutes after big US releases like CPI, NFP and the FOMC decision. In those moments spreads widen and stops can slip, so a loss can end up bigger than planned."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Avoid the rollover at 5 pm New York and big news like CPI, because spreads get wide."
expect = "almost"

[[explain_back.fixtures]]
answer = "News releases are the best time to trade because gold moves fast, and your stop-loss always protects you anyway."
expect = "almost_or_not_yet"
+++
