+++
id = 6
stage = 1
title = "Dollar moves, points and pips"
objective = "measure any move in dollars and convert other people’s “pips” before trusting them."
hook = "A video says “I only risk 50 pips on gold.” Is that a lot or a little?"
analogy = "Think of a ruler with three scales: centimetres, millimetres and metres. A pencil is 15 cm, 150 mm or 0.15 m: the same length, three different numbers. Dollar moves, points and pips are three scales on the same ruler. Trouble starts when two people use different scales but say the same word."
analogy_tag = "ruler with three scales"
example = "Gold moves from 4,150.00 to 4,153.00. That’s a $3 move. In this course that’s also 300 points (at $0.01 each) or 30 pips (at $0.10 each). But someone who calls $1 a pip would say “3 pips”, and someone who calls $0.01 a pip would say “300 pips”. Same move, three different numbers. (Illustrative prices.)"
mistake = "Assuming a pip means the same thing everywhere, or that pips are an amount of money. A pip is only a distance, and for gold different people use different sizes. Always convert to a dollar move first."
takeaways = [
  "This course measures every distance as a Dollar Move.",
  "Here, a point is $0.01 and a pip is $0.10.",
  "Other people may mean $0.01, $0.10 or $1 when they say “pip” on gold.",
  "Convert any “pips” claim into dollars before you trust or copy it.",
]
drills = ["pip_convert"]

[[videos]]
id = "3jwixTmgHUg"
title = "How to Calculate Pips in Forex"
channel = "The Moving Average"
duration = "6:27"
watch_for = "The idea that a pip is a distance, not money. The video uses currency pairs, whose pip sizes differ from gold’s; convert any gold figures using this lesson’s rule."
fallback_search = "how to calculate pips explained"
checked = "2026-11-10"

[[sources]]
claim = "Gold is quoted in US dollars per troy ounce, so a move can always be measured in dollars."
name = "LBMA: About LBMA daily auction prices"
url = "https://www.lbma.org.uk/prices-and-data/about-lbma-daily-auction-prices"
checked = "2026-11-10"

[[quiz]]
q = "Gold moves from 4,150.00 to 4,153.00. What is the Dollar Move?"
options = ["$0.30", "$3", "$30", "$300"]
answer = 1
why = "4,153 − 4,150 = $3."

[[quiz]]
q = "In this course, how big is one pip on gold?"
options = ["$0.01", "$0.10", "$1", "$10"]
answer = 1
why = "We use a pip = $0.10 and a point = $0.01."

[[quiz]]
q = "A $5 move is how many pips in this course?"
options = ["5", "50", "500", "0.5"]
answer = 1
why = "$5 ÷ $0.10 per pip = 50 pips."

[[quiz]]
q = "Someone says “a 30-pip stop on gold”. What is the first thing you should do?"
options = ["Copy it exactly", "Ask or work out what pip size they mean, then convert to dollars", "Multiply by 100", "Ignore stops entirely"]
answer = 1
why = "“30 pips” could mean $0.30, $3 or $30 depending on the speaker. Convert first."

[[quiz]]
q = "Which statement is true?"
options = ["A pip is the same size on every chart", "A pip is an amount of money", "A pip is a distance, and its size depends on who is using the word", "Pips only apply to stocks"]
answer = 2
why = "Pips measure distance, and for gold the size varies by source."

[explain_back]
prompt = "A YouTuber says “50-pip stop on gold.” What do you do before trusting that?"
must_include = ["convert it into a dollar move", "pip size differs between sources", "in this course a point is $0.01 and a pip is $0.10"]
misconceptions = ["a pip means the same everywhere", "pips are an amount of money"]

[[explain_back.fixtures]]
answer = "I'd turn it into a dollar move first, because people use different pip sizes for gold. In this course a pip is $0.10 and a point is $0.01, so if they mean that, 50 pips is a $5 stop, but they might mean something else."
expect = "got_it"

[[explain_back.fixtures]]
answer = "I'd convert it into dollars, because 50 pips could mean different things depending on who's talking."
expect = "almost"

[[explain_back.fixtures]]
answer = "Nothing, 50 pips is 50 pips on every chart. It means you'd lose $50."
expect = "almost_or_not_yet"
+++

You already know how to measure a move: subtract one price from the other. From 4,150.00 to 4,153.00 is a **$3 move**. This course calls that a **Dollar Move**, and it’s the unit we use everywhere: for stops, targets, journal entries and your Trading Plan.

So why learn other units at all? Because the rest of the trading world uses them, and they cause real confusion.

### Points and pips

- A **point** is the smallest step a gold quote normally shows: **$0.01**, one cent.
- A **pip** is a bigger step. **In this course, a pip is $0.10.**

So a $3 move is 300 points, or 30 pips. A $5 move is 500 points, or 50 pips.

| Dollar Move | Points ($0.01) | Pips ($0.10) |
|---|---|---|
| $0.30 | 30 | 3 |
| $3 | 300 | 30 |
| $5 | 500 | 50 |
| $10 | 1,000 | 100 |

### The problem with “pips”

The word *pip* comes from currency trading, where it has a clear meaning. Gold doesn’t have one agreed pip size, so different people use different ones:

- Some call **$0.01** a pip.
- Some, like this course, call **$0.10** a pip.
- Some call **$1** a pip.

That means “50 pips” on gold could be **$0.50**, **$5** or **$50**. That’s a hundredfold difference, and it matters enormously when someone is telling you where to put a stop-loss.

### Your rule

Whenever you hear or read a distance in pips or points for gold, **convert it into a Dollar Move before you do anything with it.** If the source doesn’t say what size they mean, look at a chart or a number they quote alongside it and work it out. If you can’t, don’t trust the number.

### Pips are distance, not money

One more trap: a pip is a **distance in price**, not an amount of money. How much a $5 move is worth to you depends on how big your trade is, which is the subject of the next lesson. “I made 50 pips” tells you nothing about how much money someone made.

### Why we use Dollar Moves

Because dollars can’t be misunderstood. “My stop is $5 away” means the same thing to everyone, on every platform. You’ll use the pip vocabulary to understand other people, and Dollar Moves to plan your own trades.
