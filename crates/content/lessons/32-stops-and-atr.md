+++
id = 32
stage = 7
title = "Stops beyond structure, sized by ATR"
objective = "place a stop where the chart proves you wrong: beyond the structure, about 1 to 1.5 times the ATR, and clear of an obvious round number."
hook = "Your stop sat $1 under the candle's low. Gold dipped $2, took you out, then rallied to your target without you. Was the idea wrong, or the stop?"
analogy = "Think of parking beside a busy road. Park right on the line and passing traffic will clip your mirror, even though nobody is aiming at you. Park a sensible distance back and only a real crash reaches you. The ATR tells you how wide the traffic swings."
analogy_tag = "parking by a busy road"
example = "Long on a pullback. The H1 confirmation candle's low is 4,105 and the H1 ATR(14) is $6. One ATR beyond the low is 4,099, a hair above the round number 4,100, where many stops already sit. So the stop goes a few dollars past it, at 4,097: $8 beyond the low, about 1.3 times the ATR. Entry 4,113, so $16 at risk, and the target must be at least 4,145. (Illustrative prices.)"
mistake = "Choosing the stop from how much you want to lose (\"$5 feels fine\") and then squeezing it under the candle. A stop inside the normal noise gets hit by ordinary wiggles; the size is what you adjust, never the stop."
takeaways = [
  "The chart decides the stop: beyond the structure that proves you wrong, the confirmation candle's wick and the zone.",
  "ATR (Average True Range) measures normal movement; give the stop about 1 to 1.5 times the ATR beyond the structure.",
  "Don't put a stop on an obvious round number or a few cents past it; give it a few dollars more.",
  "If the stop this needs leaves no room for a 2:1 target, skip the trade rather than tighten the stop.",
]
drills = ["place_stop"]

[[steps]]
heading = "The chart decides the stop"
text = "A stop belongs where the trade idea is **proven wrong**: beyond the **structure**, meaning the confirmation candle's wick and the zone (Lessons 26 and 27). If price gets there, the setup has failed. A stop placed anywhere else is just a number you liked."
why = "CME Group's lesson on position size says stops shouldn't be set at random levels: a stop goes at a logical level that tells you you're wrong, and not where normal market movement could easily trigger it."

[[steps]]
heading = "ATR measures the noise"
text = "**ATR (Average True Range)** is the average size of recent candles, gaps included, usually over **14** candles. Add it from your platform's indicator list, on the timeframe you enter on (H1 here). An ATR of $6 means a normal H1 candle covers about $6."
why = "Fidelity's guide: the true range of a candle is the largest of high minus low, high minus the previous close, or the previous close minus low; ATR averages it, typically over 14 periods. ATR shows how much price moves, not which way."

[[steps]]
heading = "1 to 1.5 ATR beyond the structure"
text = "Put the stop about **1 to 1.5 × ATR beyond** the structure: past the low for a buy, past the high for a sell. Closer than that, ordinary wiggles take you out. Much further, and the size shrinks and the target drifts out of reach."
why = "Fidelity suggests a multiple of ATR, such as 1.5, to catch abnormal moves rather than normal ones. 1–1.5× is this course's range, not a law; when the market gets wilder, the ATR grows and your stop widens with it automatically."

[[steps]]
heading = "Stay off the round number"
text = "Many stops gather **just past round numbers** like 4,100 or 4,050. When price touches them, those stops fire together and can push price further, briefly. So don't park yours there: if the stop lands on or a few cents past one, move it **a few dollars further**."
why = "A Federal Reserve Bank of New York study of a bank's currency orders found stop-loss sell orders clustered just below round numbers and stop-loss buy orders just above them, and that prices tend to move quickly once those levels are crossed."

[[steps]]
heading = "Too wide? Skip, don't squeeze"
text = "With the stop placed, check the target: it still needs to be at least **twice** the stop distance (Lesson 27). If it isn't, **skip** the trade. Never pull the stop closer to make the numbers work; Lesson 33 shrinks the size instead."
why = "Squeezing the stop doesn't make a trade safer; it makes a normal wiggle into a loss. The stop's distance then decides the size, which is the next lesson."

[[videos]]
id = "zMT1Fs2u9nY"
title = "ATR - Average True Range indicator EXPLAINED in less than 2 minutes"
channel = "ForexSignals TV"
duration = "1:49"
watch_for = "What the ATR line measures and why a bigger number means wider candles. It shows how far price moves, never which way."
fallback_search = "average true range explained"
checked = "2026-11-16"

[[videos]]
id = "M79kxiOMJJg"
title = "How to Set a Stop loss in Trading - So You Don't Get Stopped Out Unnecessarily"
channel = "Rayner Teo"
duration = "8:49"
watch_for = "Placing the stop beyond the structure plus a buffer from the ATR, instead of a fixed distance. His exact multiples may differ from this course's 1 to 1.5."
fallback_search = "where to set stop loss ATR structure"
checked = "2026-11-16"

[[sources]]
claim = "ATR is the average of true ranges over a period, typically 14, and measures volatility including gaps; it is not directional. ATR stops adapt to volatility where fixed stops don't; a multiple of ATR, such as 1.5 × ATR, can be used to catch abnormal price moves."
name = "Fidelity Learning Center: Average True Range (ATR)"
url = "https://www.fidelity.com/learning-center/trading-investing/technical-analysis/technical-indicator-guide/atr"
checked = "2026-11-16"

[[sources]]
claim = "Stops should not be set at random levels: a stop needs to be placed at a logical level, where it will tell you if you're wrong about the direction of the trade, and not where it could easily be triggered by normal market movements."
name = "CME Group Education: Proper Position Size"
url = "https://www.cmegroup.com/education/courses/trade-and-risk-management/proper-position-size.html"
checked = "2026-11-16"

[[sources]]
claim = "In a large dealing bank's currency order book, order prices clustered at round numbers; stop-loss buy orders clustered just above round numbers and stop-loss sell orders just below them, and trends tend to gain momentum once those levels are crossed."
name = "Federal Reserve Bank of New York Staff Report 125 (C. L. Osler): Currency Orders and Exchange-Rate Dynamics"
url = "https://www.newyorkfed.org/research/staff_reports/sr125.html"
checked = "2026-11-16"

[[quiz]]
id = "q1"
q = "What decides where your stop goes?"
options = ["The chart: beyond the structure that proves the idea wrong", "How much money you feel like losing", "A fixed $5 on every trade", "The nearest round number"]
answer = 0
why = "The stop goes where the setup has failed; the size then makes the loss fit your plan."

[[quiz]]
id = "q2"
q = "What does an H1 ATR of $6 tell you?"
options = ["A normal H1 candle has recently covered about $6", "Price will rise $6", "Your stop must be exactly $6", "The spread is $6"]
answer = 0
why = "ATR measures how much price moves, not which way."

[[quiz]]
id = "q3"
q = "Long. The confirmation candle's low is 4,105 and the ATR is $6. Where does the stop go?"
options = ["4,097: about 1.3 ATR below the low, past the round number", "4,104.50: just under the wick", "4,100.00: right on the round number", "4,087: three ATRs away"]
answer = 0
why = "$8 below the low is about 1.3 × $6, and it sits a few dollars past 4,100 rather than on it."

[[quiz]]
id = "q4"
q = "Why not put a buy's stop at 4,099.90, just under 4,100?"
options = ["Many sell stops cluster just below round numbers, and price can run through them", "Brokers don't allow it", "It's too far from the entry", "Round numbers never matter"]
answer = 0
why = "The New York Fed study found stop-loss sell orders clustered just below round numbers."

[[quiz]]
id = "q5"
q = "Your structure-based stop is $20 away and the next zone is only $25 away. What do you do?"
options = ["Skip: the target isn't twice the risk", "Move the stop to $12 so it works", "Remove the stop", "Double the size"]
answer = 0
why = "Never squeeze the stop to rescue a trade; skip it."

[[quiz]]
id = "q6"
q = "The market gets wilder and the ATR doubles. What happens to your stop?"
options = ["It widens, and the size shrinks to keep the same money at risk", "It stays the same distance", "It gets tighter", "You stop using a stop"]
answer = 0
why = "A wider stop with a smaller size keeps the loss the same."

[[recall]]
id = "r1"
q = "A stop belongs…"
options = ["Beyond the structure that proves the idea wrong", "Wherever the loss feels comfortable", "On the nearest round number"]
answer = 0
why = "The chart decides the stop."

[[recall]]
id = "r2"
q = "This course places the stop about…"
options = ["1 to 1.5 × ATR beyond the structure", "Exactly on the candle's low", "10 × ATR away"]
answer = 0
why = "Far enough to survive ordinary noise, close enough to keep the trade worth taking."

[[recall]]
id = "r3"
q = "A stop on or a few cents past a round number…"
options = ["Sits where many stops cluster: move it a few dollars further", "Is the safest place"]
answer = 0
why = "Clusters of stops can make price run briefly once they fire."

[explain_back]
prompt = "How do you decide where a stop goes, and how does the ATR help?"
must_include = ["the stop goes beyond the structure that proves the trade wrong, the confirmation candle's wick and the zone, not at a money amount you picked", "ATR measures normal movement, and the stop goes about 1 to 1.5 times the ATR beyond the structure", "you keep it off an obvious round number, and if the stop leaves no room for a 2:1 target you skip the trade instead of tightening the stop"]
misconceptions = ["a tighter stop always means less risk", "you pick the stop from how much you want to lose"]

[[explain_back.fixtures]]
answer = "The chart decides it: the stop goes beyond the structure that shows I'm wrong, so past the confirmation candle's wick and the zone. ATR tells me how much gold normally moves, so I put the stop about 1 to 1.5 ATR beyond that point, and if that lands on or just past a round number like 4,100 I push it a few dollars further because stops cluster there. If the target isn't at least twice that stop, I skip the trade instead of squeezing the stop."
expect = "got_it"

[[explain_back.fixtures]]
answer = "I put the stop beyond the confirmation candle's wick and the zone, about 1 to 1.5 times the ATR away, because the ATR shows how much gold normally moves."
expect = "almost"

[[explain_back.fixtures]]
answer = "I decide I'm happy to lose $5, so I put the stop $5 from my entry. A tight stop is always safer."
expect = "almost_or_not_yet"
+++
