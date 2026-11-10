+++
id = 7
stage = 1
title = "Lots: how big is your trade?"
objective = "turn a lot size and a dollar move into money gained or lost."
hook = "Two traders catch the same $4 move in gold. One makes $4 and the other makes $400. How?"
analogy = "Think of buying in boxes. One lot is a box of 100 ounces of gold. A tenth of a lot (0.10) is 10 ounces, and a hundredth (0.01) is a single ounce. When gold’s price moves, every ounce in your box moves with it, so a bigger box means bigger gains and bigger losses."
analogy_tag = "boxes of ounces"
example = "Gold rises from 4,150.00 to 4,154.00, a $4 move. With 1 lot (100 oz) that’s 100 × $4 = $400. With 0.20 lots (20 oz) it’s 20 × $4 = $80. With 0.01 lots (1 oz) it’s $4. Same move, very different money. Had gold fallen $4 instead, those would all be losses. (Illustrative prices; assumes 100 oz per lot.)"
mistake = "Thinking a lot is a fixed amount of money, or that profit depends only on how far price moved. The money depends on both the move and the size of your trade."
takeaways = [
  "A lot is a size: in this course, 1 lot = 100 ounces of gold.",
  "Money = lots × 100 × Dollar Move.",
  "0.01 lots is usually the smallest trade you can place.",
  "Your broker sets the contract size, so check it before you trade.",
]
drills = ["dollar_distance"]

[[videos]]
id = "GJ-Bjq1Xvks"
title = "Forex Leverage for Beginners Explained (lot sizes and pips)"
channel = "The Moving Average"
duration = "7:17"
watch_for = "How the size of a trade changes the money made or lost on the same move. It uses currency pairs; for gold, use this lesson’s 100-ounce box and Dollar Moves."
fallback_search = "lot size explained beginners"
checked = "2026-11-10"

[[sources]]
claim = "In MetaTrader 5, the minimum trade volume and its step are set by the broker."
name = "MetaTrader 5 Help: Trade"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade"
checked = "2026-11-10"

[[quiz]]
q = "In this course, how many ounces of gold is 1 lot?"
options = ["1", "10", "100", "1,000"]
answer = 2
why = "We use the common setting of 1 lot = 100 ounces. Always confirm it in your broker’s spec."

[[quiz]]
q = "You hold 0.20 lots and gold rises $4. How much do you make?"
options = ["$0.80", "$8", "$80", "$800"]
answer = 2
why = "0.20 × 100 × $4 = $80."

[[quiz]]
q = "You hold 1 lot and gold falls $3 against you. What is the result?"
options = ["−$3", "−$30", "−$300", "−$3,000"]
answer = 2
why = "1 × 100 × $3 = $300, and the move went against you, so it’s a $300 loss."

[[quiz]]
q = "Which trade size makes $1 for every $1 that gold moves?"
options = ["0.01 lots", "0.10 lots", "1 lot", "10 lots"]
answer = 0
why = "0.01 lots is 1 ounce: 0.01 × 100 × $1 = $1."

[[quiz]]
q = "Two traders catch the same $5 move. Why might one make far more than the other?"
options = ["Their brokers show different prices", "They traded different lot sizes", "One used a better chart", "Pips are worth more on Fridays"]
answer = 1
why = "Same move, different size: money = lots × 100 × Dollar Move."

[explain_back]
prompt = "You buy 0.20 lots and gold rises $4. How much did you make, and why?"
must_include = ["1 lot is usually 100 ounces (check the broker)", "money = lots × 100 × dollar move", "so 0.20 lots × $4 = $80"]
misconceptions = ["a lot is a fixed dollar amount", "profit depends only on the price move, not the size"]

[[explain_back.fixtures]]
answer = "A standard lot is 100 ounces, though you should check your broker, so 0.20 lots is 20 ounces. Each ounce made $4, so that's 0.20 × 100 × $4 = $80."
expect = "got_it"

[[explain_back.fixtures]]
answer = "$80, because 0.20 lots times $4 times 100 is 80."
expect = "almost"

[[explain_back.fixtures]]
answer = "$4, because the price went up $4 and that's what you make no matter how many lots."
expect = "almost_or_not_yet"
+++

You now know how to measure a move in dollars. But a $4 move doesn’t tell you how much money you made. For that you need one more number: **how big your trade was**. In trading, size is measured in **lots**.

### What a lot is

A **lot** is a standard trade size. For gold, this course uses the common setting:

> **1 lot = 100 ounces of gold**

Smaller sizes are written as fractions of a lot:

| Lots | Ounces | Money per $1 move |
|---|---|---|
| 1.00 | 100 | $100 |
| 0.10 | 10 | $10 |
| 0.01 | 1 | $1 |

The smallest size most platforms allow is **0.01 lots**. In MetaTrader 5, the minimum size and the steps you can increase it by are set by your broker, which is why you’ll check them when you open your demo account in Lesson 11.

### The money formula

Every gold trade’s result comes from three numbers multiplied together:

> **money = lots × 100 × Dollar Move**

- 0.20 lots, gold up $4 → 0.20 × 100 × 4 = **$80**.
- 0.05 lots, gold down $6 → 0.05 × 100 × 6 = **$30 loss**.
- 1 lot, gold up $0.50 → 1 × 100 × 0.50 = **$50**.

Notice how quickly money adds up at 1 lot. A $10 move, which gold can make in minutes on a busy day, is $1,000 at 1 lot. That’s why size, not just direction, decides whether a trading mistake is a lesson or a disaster.

### Check your broker’s contract size

Not every broker uses 100 ounces for one lot of XAUUSD. Some use different contract sizes, and the formula changes if they do. The **contract size** is listed in your broker’s specification for the symbol. Until you’ve checked it, treat “1 lot = 100 ounces” as an assumption, not a fact.

### Why this matters

Later in the course, you’ll turn this formula around. Instead of asking “how much will I make at this size?”, you’ll ask “**what size can I trade so that, if I’m wrong, I lose only what I planned?**”. That’s position sizing, and it’s the single most important habit in trading. It starts with being completely comfortable with lots × 100 × Dollar Move.
