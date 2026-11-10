+++
id = 9
stage = 1
title = "Leverage & margin"
objective = "explain margin as a deposit, and why leverage magnifies losses just as much as gains."
hook = "How can a small account control hundreds of thousands of dollars of gold, and why is that dangerous?"
analogy = "It’s like buying a house with a mortgage. A small deposit lets you control a large asset. If the house rises 10%, your deposit grows a lot. But if it falls 10%, that loss comes out of your deposit first, and a big enough fall can wipe the deposit out entirely."
analogy_tag = "mortgage deposit"
example = "1 lot of gold at 4,150.00 is 100 ounces, worth about $415,000. With leverage of 100:1 your broker might ask for a deposit (margin) of about $4,150. Now a $41.50 move against you is only 1% of the position’s value, but it’s $4,150 of loss: 100% of that deposit. (Illustrative prices and leverage.)"
mistake = "Believing leverage is free money, or that more leverage means more profit. Leverage makes every move bigger in both directions. Your position size, not the leverage number, decides how much you can lose."
takeaways = [
  "Margin is a deposit your broker holds while a trade is open, not the cost of the trade.",
  "Leverage lets a small deposit control a large position.",
  "It magnifies losses exactly as much as gains, and can empty an account fast.",
  "Position size, not leverage, sets your real risk.",
]

[[videos]]
id = "b8j6s0Z8s18"
title = "Forex Leverage: 90% Of Beginners Make This Mistake When Trading With Margin..."
channel = "The Trading Channel (The Trading Channel)"
duration = "15:16"
watch_for = "Why the leverage number itself isn’t your risk, and how position size is. It uses currency examples; the logic is the same for gold."
fallback_search = "leverage and margin explained beginners mistake"
checked = "2026-11-10"

[[sources]]
claim = "UK rules limit leverage on CFDs sold to retail clients to between 30:1 and 2:1, depending on the market."
name = "FCA: FCA confirms permanent restrictions on the sale of CFDs to retail consumers"
url = "https://www.fca.org.uk/news/press-releases/fca-confirms-permanent-restrictions-sale-cfds-and-cfd-options-retail-consumers"
checked = "2026-11-10"

[[sources]]
claim = "High-risk products such as CFDs carry a high chance of losing all your money."
name = "FCA: Understanding high-risk investments"
url = "https://www.fca.org.uk/investsmart/understanding-high-risk-investments"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "What is margin?"
options = ["The fee for opening a trade", "A deposit held while your trade is open", "Your profit on a trade", "Free money from the broker"]
answer = 1
why = "Margin is a deposit. You get it back when you close, minus any loss."

[[quiz]]
id = "q2"
q = "With 100:1 leverage, roughly how much margin does a position worth $415,000 need?"
options = ["$415", "$4,150", "$41,500", "$415,000"]
answer = 1
why = "$415,000 ÷ 100 = $4,150."

[[quiz]]
id = "q3"
q = "You hold 1 lot of gold and price moves $41.50 against you. How much do you lose?"
options = ["$41.50", "$415", "$4,150", "Nothing, thanks to leverage"]
answer = 2
why = "1 × 100 × $41.50 = $4,150. Leverage doesn’t change the money formula; it only lets you open the trade with a smaller deposit."

[[quiz]]
id = "q4"
q = "What does leverage do to losses?"
options = ["Reduces them", "Magnifies them just as much as gains", "Removes them", "Only affects profits"]
answer = 1
why = "Leverage magnifies both directions equally."

[[quiz]]
id = "q5"
q = "What actually sets how much money you can lose on a trade?"
options = ["The leverage number", "Your position size and the distance to your stop", "The broker’s logo", "The time of day"]
answer = 1
why = "Lots × 100 × Dollar Move to your stop: size and stop distance decide the loss."

[explain_back]
prompt = "Why is leverage dangerous for a beginner even though it “lets you trade bigger”?"
must_include = ["margin is a deposit that lets you control a much bigger position (it is not the full cost of the trade)", "leverage magnifies losses as much as gains", "it can wipe out an account fast", "position size, not leverage, sets your risk"]
misconceptions = ["leverage is free money", "more leverage means more profit"]

[[explain_back.fixtures]]
answer = "Margin is just a deposit that lets you control a much bigger position. Because the position is big, a small move against you is a big loss compared with your deposit, just as wins are bigger, so an account can be wiped out quickly. What really sets your risk is your position size, not the leverage number."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Because leverage makes your losses as big as your wins, and a small move against you can wipe out your account."
expect = "almost"

[[explain_back.fixtures]]
answer = "It isn't dangerous really. More leverage means more profit, it's like the broker giving you extra money to trade with."
expect = "almost_or_not_yet"

[[steps]]
heading = "Margin is a deposit"
text = "Your broker doesn't ask for a trade's full value. It holds a **margin**: a deposit set aside while the trade is open, released when you close, plus any profit or minus any loss."
why = "The ratio between the position's full value and the deposit is the **leverage**. 1 lot near 4,150 is about $415,000 of gold. With 100:1 leverage it needs about $4,150 of margin; with 20:1, about $20,750."
[[steps]]
heading = "Leverage magnifies both ways"
text = "Leverage doesn't change money = lots × 100 × Dollar Move; it only lets a small deposit open a big trade. At 100:1, 1 lot needs about $4,150, and a **$41.50** move against you costs all of it."
why = "That move is about 1% of gold's price, the kind it can make in a day. The same move your way would double the deposit, which is why leverage tempts people, and why it empties so many accounts."
widget = "leverage_toggle"
[[steps]]
heading = "Limits and close-outs"
text = "Regulators cap leverage for ordinary customers; in the UK it runs from 30:1 down to 2:1 depending on the market. If losses grow too far, your broker closes your trades automatically. You don't get to wait for a recovery."
[[steps]]
heading = "The real control: your size"
text = "High leverage doesn't force you to trade big; it allows it. What decides your risk is **how many lots** and **how far away your stop** is. At 100:1, trading 0.01 lots risks far less than trading 1 lot at 20:1."
why = "So never ask “how much leverage can I get?”. Ask “how big can this trade be so that, if my stop is hit, I lose only what I planned?”. Lesson 33 turns that into a calculator."
[[recall]]
id = "r1"
q = "Margin is…"
options = ["A deposit held while a trade is open", "The fee for a trade", "Your profit"]
answer = 0
why = "You get it back when you close, plus or minus the result."
[[recall]]
id = "r2"
q = "Leverage magnifies…"
options = ["Gains and losses equally", "Only gains", "Only losses"]
answer = 0
why = "It makes every move bigger, in both directions."
[[recall]]
id = "r3"
q = "What actually sets how much you can lose?"
options = ["Your size and stop distance", "The leverage number", "The time of day"]
answer = 0
why = "Lots × 100 × the distance to your stop."
+++
