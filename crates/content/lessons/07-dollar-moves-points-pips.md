+++
id = 7
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
id = "q1"
q = "Gold moves from 4,150.00 to 4,153.00. What is the Dollar Move?"
options = ["$0.30", "$3", "$30", "$300"]
answer = 1
why = "4,153 − 4,150 = $3."

[[quiz]]
id = "q2"
q = "In this course, how big is one pip on gold?"
options = ["$0.01", "$0.10", "$1", "$10"]
answer = 1
why = "We use a pip = $0.10 and a point = $0.01."

[[quiz]]
id = "q3"
q = "A $5 move is how many pips in this course?"
options = ["5", "50", "500", "0.5"]
answer = 1
why = "$5 ÷ $0.10 per pip = 50 pips."

[[quiz]]
id = "q4"
q = "Someone says “a 30-pip stop on gold”. What is the first thing you should do?"
options = ["Copy it exactly", "Ask or work out what pip size they mean, then convert to dollars", "Multiply by 100", "Ignore stops entirely"]
answer = 1
why = "“30 pips” could mean $0.30, $3 or $30 depending on the speaker. Convert first."

[[quiz]]
id = "q5"
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

[[steps]]
heading = "Dollar Moves first"
text = "You already measure a move by subtraction: 4,150.00 to 4,153.00 is a **$3 move**. This course calls that a **Dollar Move**, and uses it everywhere: stops, targets, your journal and your Trading Plan."
[[steps]]
heading = "Points and pips"
text = "A **point** is **$0.01**, the smallest step a gold quote shows. **In this course, a pip is $0.10.** So a $3 move is 300 points or 30 pips, and a $5 move is 500 points or 50 pips."
why = "| Dollar Move | Points ($0.01) | Pips ($0.10) |\n|---|---|---|\n| $0.30 | 30 | 3 |\n| $3 | 300 | 30 |\n| $5 | 500 | 50 |\n| $10 | 1,000 | 100 |"
[[steps]]
heading = "The problem with pips"
text = "Gold has no agreed pip size. Some people mean **$0.01**, some **$0.10**, some **$1**. So “50 pips” could be **$0.50, $5 or $50**: a hundredfold difference when someone is telling you where to put your stop-loss."
why = "The word pip comes from currency trading, where it has one clear meaning. Gold never got an agreed one."
widget = "pip_convert"
[[steps]]
heading = "Your rule"
text = "Whenever you hear a distance in pips or points for gold, **convert it to a Dollar Move first**. If you can't tell which size they mean, don't trust the number. And remember: a pip is a distance, not an amount of money."
why = "How much a $5 move is worth to you depends on your trade size, which is the next lesson. “I made 50 pips” tells you nothing about how much money someone made."
[[recall]]
id = "r1"
q = "Gold moves from 4,150.00 to 4,153.00. The Dollar Move is…"
options = ["$3", "$30", "$0.30"]
answer = 0
why = "4,153 − 4,150 = $3."
[[recall]]
id = "r2"
q = "In this course, one pip on gold is…"
options = ["$0.10", "$0.01", "$1"]
answer = 0
why = "A point is $0.01 and a pip is $0.10."
[[recall]]
id = "r3"
q = "Someone says “a 30-pip stop on gold”. That's…"
options = ["$0.30, $3 or $30, depending on what they mean", "Always $3", "Always $30"]
answer = 0
why = "Pip size isn't agreed for gold, so convert before you trust it."
+++
