+++
id = 4
stage = 1
title = "Reading “XAUUSD”"
objective = "read the symbol and the quote as US dollars per troy ounce of gold."
hook = "You open a trading app and see “XAUUSD 4,150.00”. What are you actually looking at?"
analogy = "It’s a price tag in a shop. The tag on a loaf of bread says what one loaf costs. XAUUSD is the price tag on one troy ounce of gold, written in US dollars. When the tag changes, the price of that ounce has changed."
analogy_tag = "shop price tag"
example = "XAUUSD 4,150.00 means one troy ounce of gold costs $4,150. If the quote moves to 4,155.00, gold has become $5 more expensive in dollars. If it moves to 4,140.00, gold has become $10 cheaper. (Illustrative prices.)"
mistake = "Thinking the number is the price of a gram, a kilo or a whole bar, or that XAU is a company. It is the price of one troy ounce of gold, in US dollars, and nothing else."
takeaways = [
  "XAU is the code for gold; USD is the US dollar.",
  "XAUUSD is the price of one troy ounce of gold in dollars.",
  "When the number rises, gold costs more dollars; when it falls, it costs fewer.",
  "A troy ounce is the standard weight for precious metals.",
]
drills = ["dollar_distance"]

[[videos]]
id = "K72sISknAH4"
title = "Troy oz VS Regular oz | What is the difference?"
channel = "Chards Coin and Bullion Dealer"
duration = "1:37"
watch_for = "That a troy ounce is its own unit for precious metals, not the ounce on your kitchen scales."
fallback_search = "troy ounce vs regular ounce"
checked = "2026-11-10"

[[sources]]
claim = "XAU is the ISO 4217 code for gold (numeric code 959)."
name = "SIX (ISO 4217 Maintenance Agency): current currency code list"
url = "https://www.six-group.com/dam/download/financial-information/data-center/iso-currrency/lists/list-one.xml"
checked = "2026-11-10"

[[sources]]
claim = "The benchmark LBMA Gold Price is set in US dollars per troy ounce."
name = "LBMA: About LBMA daily auction prices"
url = "https://www.lbma.org.uk/prices-and-data/about-lbma-daily-auction-prices"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "What does XAU stand for?"
options = ["A gold mining company", "Gold", "The US dollar", "An exchange in London"]
answer = 1
why = "XAU is the official code for gold. “Au” comes from aurum, Latin for gold."

[[quiz]]
id = "q2"
q = "XAUUSD = 4,150.00. What does that mean?"
options = ["One gram of gold costs $4,150", "One troy ounce of gold costs $4,150", "One kilo of gold costs $4,150", "One share of gold costs $4,150"]
answer = 1
why = "The quote is the price of one troy ounce of gold in US dollars."

[[quiz]]
id = "q3"
q = "XAUUSD moves from 4,150.00 to 4,162.00. What happened?"
options = ["Gold got $12 cheaper", "Gold got $12 more expensive", "The dollar got $12 more expensive", "Nothing: that’s a normal rounding"]
answer = 1
why = "A higher number means one ounce of gold now costs more dollars: 4,162 − 4,150 = $12."

[[quiz]]
id = "q4"
q = "XAUUSD moves from 4,150.00 to 4,135.50. How far did it move, and which way?"
options = ["$14.50 down", "$14.50 up", "$1.45 down", "$145 down"]
answer = 0
why = "4,150.00 − 4,135.50 = $14.50, and the number went down, so gold got cheaper."

[[quiz]]
id = "q5"
q = "Which unit of weight is XAUUSD priced in?"
options = ["Kilograms", "Grams", "Troy ounces", "Regular kitchen ounces"]
answer = 2
why = "Precious metals are priced in troy ounces, a different unit from a regular ounce."

[explain_back]
prompt = "What does XAUUSD = 4,150 mean, and what does it mean when it rises?"
must_include = ["XAU is gold and USD is the US dollar", "the number is the price of one troy ounce in dollars", "a rise means gold costs more dollars"]
misconceptions = ["the number is the price of a gram or a kilo", "XAU is a company or a stock"]

[[explain_back.fixtures]]
answer = "XAU means gold and USD means US dollars, so 4,150 means one troy ounce of gold costs $4,150. If it rises to 4,155, gold has become $5 more expensive in dollars."
expect = "got_it"

[[explain_back.fixtures]]
answer = "It means gold costs 4,150 US dollars, and if the number goes up gold is more expensive."
expect = "almost"

[[explain_back.fixtures]]
answer = "XAUUSD is the share price of a big gold mining company, and 4,150 is what one share costs."
expect = "almost_or_not_yet"

[[steps]]
heading = "Splitting the symbol"
text = "**XAU** is the official code for gold; **USD** is the US dollar. Together, XAUUSD means **gold priced in US dollars**. You'll also see it written XAU/USD; it's the same thing."
why = "XAU comes from the international standard for currency codes, the same one that gives the dollar USD and the euro EUR. “Au” is gold's chemical symbol, from the Latin *aurum*."
[[steps]]
heading = "What the number means"
text = "The number is the price of **one troy ounce** of gold, in dollars. XAUUSD **4,150.00** means one troy ounce costs **$4,150**. A troy ounce is the standard unit for precious metals, not the ounce on kitchen scales."
why = "The main benchmark gold price, set twice a day in London, uses exactly the same unit: US dollars per troy ounce. You don't need to memorise its weight; just know the quote is always per troy ounce."
[[steps]]
heading = "Reading a change"
text = "If the number rises, gold costs more dollars; if it falls, fewer. The distance is just subtraction: 4,150.00 to 4,155.00 is a **$5 move**. This course always measures distance this way, as a **Dollar Move**."
widget = "price_drag"
[[steps]]
heading = "Two decimal places"
text = "Quotes usually show two decimals, like 4,150.37. Those are cents, so the smallest move you'll normally see is **$0.01**. Lesson 7 shows the other units people use for moves, and why they confuse beginners."
why = "Try these: 4,120.00 → 4,131.00 is up $11. 4,131.00 → 4,118.50 is down $12.50. 4,118.50 → 4,118.75 is up $0.25. A quote is just a price, and a move is just the difference between two prices."
[[recall]]
id = "r1"
q = "XAU stands for…"
options = ["Gold", "Silver", "The US dollar"]
answer = 0
why = "XAU is the official code for gold."
[[recall]]
id = "r2"
q = "XAUUSD 4,150 means…"
options = ["One troy ounce costs $4,150", "One gram costs $4,150", "One kilo costs $4,150"]
answer = 0
why = "The quote is per troy ounce, in dollars."
[[recall]]
id = "r3"
q = "Gold goes from 4,120.00 to 4,131.00. The move is…"
options = ["$11 up", "$11 down", "$1.10 up"]
answer = 0
why = "4,131 − 4,120 = $11, and the number rose."
+++
