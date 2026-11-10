+++
id = 6
stage = 1
title = "Bid, ask & spread"
objective = "explain why every trade starts slightly negative, and what the spread costs you."
hook = "Why does a brand-new trade show a small loss the moment you open it?"
analogy = "Picture an airport currency booth. The board shows two prices: “We buy at” and “We sell at”. If you swap $100 for euros and immediately swap them back, you get less than $100. You didn’t make a bad decision; you paid the gap between the two prices. Gold trading works the same way."
analogy_tag = "airport currency booth"
example = "Your platform shows Bid 4,150.20 / Ask 4,150.50. The spread is 4,150.50 − 4,150.20 = $0.30. If you buy, you pay the ask, 4,150.50, but your position is valued at the bid, 4,150.20. So the trade opens $0.30 in the red, and gold must rise $0.30 just for you to break even. (Illustrative prices.)"
mistake = "Thinking the spread is charged only when you close, or that it’s always the same size. You pay it the moment you open, and it can widen sharply when the market is quiet or around big news."
takeaways = [
  "Every quote has two prices: the bid (lower) and the ask (higher).",
  "You buy at the ask and sell at the bid.",
  "The gap between them is the spread, a cost you pay when you open a trade.",
  "The spread can widen, so check it before you trade.",
]
drills = ["bid_ask"]

[[videos]]
id = "r1N22WzhzPM"
title = "Forex Spread Explained (Video 8 of 13)"
channel = "Rayner Teo"
duration = "7:53"
watch_for = "How the two prices appear on a platform and why the spread is a cost. The video uses currency pairs; the idea is identical for gold."
fallback_search = "forex spread explained beginner"
checked = "2026-11-10"

[[sources]]
claim = "In MetaTrader 5, buy orders execute at the Ask price and sell orders at the Bid price."
name = "MetaTrader 5 Help: Types of Orders"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade/general_concept/order_types"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "Gold shows Bid 4,150.20 / Ask 4,150.50. What is the spread?"
options = ["$0.30", "$3.00", "$30", "$0.03"]
answer = 0
why = "4,150.50 − 4,150.20 = $0.30."

[[quiz]]
id = "q2"
q = "You want to buy gold right now. Which price do you pay?"
options = ["The bid", "The ask", "Halfway between them", "Yesterday’s close"]
answer = 1
why = "You buy at the ask, the higher of the two prices."

[[quiz]]
id = "q3"
q = "You want to sell (go short) right now. Which price do you get?"
options = ["The bid", "The ask", "The higher of the two", "Whichever you choose"]
answer = 0
why = "You sell at the bid, the lower of the two prices."

[[quiz]]
id = "q4"
q = "When do you pay the spread?"
options = ["Only when you close", "When you open the trade", "Once a month", "Only if the trade loses"]
answer = 1
why = "The spread is paid on entry: that’s why a new trade starts slightly negative."

[[quiz]]
id = "q5"
q = "The spread is $0.40 and you buy. How far must gold rise before you break even?"
options = ["$0.04", "$0.40", "$4.00", "It’s already at break-even"]
answer = 1
why = "You start $0.40 behind, so price needs to rise $0.40 to get back to zero."

[[quiz]]
id = "q6"
q = "Which is true about the spread?"
options = ["It never changes", "It can widen when the market is quiet or around big news", "It is always exactly $1", "It is paid back to you when you close"]
answer = 1
why = "Spreads move. They tend to be wider when trading is thin or when big news hits."

[explain_back]
prompt = "Why do you start every trade slightly “in the red”?"
must_include = ["you buy at the ask and sell at the bid", "the ask is higher than the bid", "the spread is a cost you pay when you enter"]
misconceptions = ["the spread is only charged when you close", "the spread is always the same size"]

[[explain_back.fixtures]]
answer = "Because you buy at the higher ask price but your position is valued at the lower bid. The gap between them is the spread, which is a cost you pay as soon as you open the trade."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Because of the spread: there are two prices and the gap between them is a cost."
expect = "almost"

[[explain_back.fixtures]]
answer = "Because the broker charges you the spread at the end when you close the trade, and it's always exactly the same amount."
expect = "almost_or_not_yet"

[[steps]]
heading = "Two prices, always"
text = "Every quote has two prices. The **bid** is what you can **sell** at: the lower one. The **ask** is what you can **buy** at: the higher one. Platforms often label them simply Sell and Buy."
why = "In MetaTrader 5, the platform you'll practise on, buy orders fill at the ask and sell orders fill at the bid."
widget = "bid_ask"
[[steps]]
heading = "The spread"
text = "The gap between them is the **spread**: ask − bid. With 4,150.20 / 4,150.50, the spread is **$0.30**. It's how many brokers earn their money, built into the prices you trade at rather than billed separately."
[[steps]]
heading = "Why a new trade starts negative"
text = "Buy, and you pay the ask; to get out immediately you'd sell at the lower bid. So the moment you open, the trade is worth exactly the spread less. Price must move at least that far your way before you make anything."
why = "The same happens on a short: you sell at the bid and would have to buy back at the ask."
[[steps]]
heading = "Spreads change"
text = "Spreads are narrow when many people are trading, and wider when the market is quiet or big news hits. A $0.30 spread can become several dollars in a news spike. Glance at both prices before every trade."
why = "Ask yourself: how much of my expected move will the spread eat? Aiming for $3 with a $0.30 spread, a tenth of your idea is gone before you start. If the spread suddenly widens to $2, the trade may not be worth taking."
[[recall]]
id = "r1"
q = "You buy now. Which price do you pay?"
options = ["The ask", "The bid", "Halfway between"]
answer = 0
why = "You buy at the ask, the higher price."
[[recall]]
id = "r2"
q = "Bid 4,150.20 / Ask 4,150.60. The spread is…"
options = ["$0.40", "$4.00", "$0.04"]
answer = 0
why = "4,150.60 − 4,150.20 = $0.40."
[[recall]]
id = "r3"
q = "When do you pay the spread?"
options = ["When you open", "When you close", "Once a month"]
answer = 0
why = "It's paid on entry, so a new trade starts slightly negative."
+++
