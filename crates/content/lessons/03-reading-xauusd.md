+++
id = 3
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
q = "What does XAU stand for?"
options = ["A gold mining company", "Gold", "The US dollar", "An exchange in London"]
answer = 1
why = "XAU is the official code for gold. “Au” comes from aurum, Latin for gold."

[[quiz]]
q = "XAUUSD = 4,150.00. What does that mean?"
options = ["One gram of gold costs $4,150", "One troy ounce of gold costs $4,150", "One kilo of gold costs $4,150", "One share of gold costs $4,150"]
answer = 1
why = "The quote is the price of one troy ounce of gold in US dollars."

[[quiz]]
q = "XAUUSD moves from 4,150.00 to 4,162.00. What happened?"
options = ["Gold got $12 cheaper", "Gold got $12 more expensive", "The dollar got $12 more expensive", "Nothing: that’s a normal rounding"]
answer = 1
why = "A higher number means one ounce of gold now costs more dollars: 4,162 − 4,150 = $12."

[[quiz]]
q = "XAUUSD moves from 4,150.00 to 4,135.50. How far did it move, and which way?"
options = ["$14.50 down", "$14.50 up", "$1.45 down", "$145 down"]
answer = 0
why = "4,150.00 − 4,135.50 = $14.50, and the number went down, so gold got cheaper."

[[quiz]]
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
+++

Every market has a symbol, and gold’s is **XAUUSD**. It looks like a password, but it reads like a sentence once you split it in two.

### Splitting the symbol

- **XAU** is the official code for gold. It comes from the international standard for currency codes, the same standard that gives the US dollar USD and the euro EUR. The “Au” is gold’s chemical symbol, from the Latin *aurum*.
- **USD** is the US dollar.

Put together, XAUUSD means **gold priced in US dollars**. You’ll sometimes see it written XAU/USD; it’s the same thing.

### What the number means

The number next to the symbol is the price of **one troy ounce** of gold, in dollars. So:

- XAUUSD **4,150.00** → one troy ounce of gold costs **$4,150**.

A **troy ounce** is the traditional unit of weight for precious metals. It is not the same as the ounce on kitchen scales, so don’t try to convert using your cookbook. You don’t need to memorise its weight for trading; you just need to know that the quote is always “per troy ounce”. The main benchmark price for gold, set twice a day in London, uses exactly the same unit: US dollars per troy ounce.

### Reading a change

Because the quote is a price, reading a change is simple:

- The number **rises** → gold costs **more** dollars.
- The number **falls** → gold costs **fewer** dollars.

The distance between two quotes is just subtraction. From 4,150.00 to 4,155.00 is a **$5 move**. From 4,150.00 to 4,135.50 is a **$14.50 move** down. In this course we always describe distances this way, as a **Dollar Move**, because it’s the clearest unit there is. Lesson 6 explains the other units you’ll hear people use, and why they cause confusion.

### Two decimal places

Gold quotes usually show two decimals, like 4,150.37. Those are cents. The smallest change you’ll normally see is one cent, $0.01. You’ll meet this tiny step again when we talk about measuring moves.

### Practice: reading three quotes

Try these in your head before reading the answers:

1. Gold moves from **4,120.00** to **4,131.00**. That’s up **$11**: one ounce now costs $11 more.
2. Gold moves from **4,131.00** to **4,118.50**. That’s down **$12.50**: one ounce now costs $12.50 less.
3. Gold moves from **4,118.50** to **4,118.75**. That’s up **$0.25**, a tiny move of 25 cents.

Notice that you never needed to know anything about lots, accounts or brokers to answer. A quote is just a price, and a move is just the difference between two prices. Keep it that simple and you’ll avoid most of the confusion beginners run into.

### Why this matters

Almost every number you’ll handle as a gold trader starts here: entry prices, stop-losses, targets and profits are all built on “dollars per ounce”. Get comfortable reading a quote and measuring the gap between two of them, and the maths in later lessons becomes simple arithmetic.
