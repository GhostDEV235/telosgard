+++
id = 21
stage = 4
title = "Candlesticks"
objective = "read a candle's open, high, low and close, say who won its period, and explain why one candle is a record, not a forecast."
hook = "One candle on your chart packs four prices into a single shape. Can you read all four at a glance?"
analogy = "A candle is like a match report. It tells you the score at kick-off (the open), the final score (the close), and the biggest lead each side had during the game (the high and the low). It tells you who won and how hard it was fought. It doesn't tell you who wins the next match."
analogy_tag = "match report"
example = "On a 1-hour chart, one candle opens at 4,100.00, climbs to 4,112.00, dips to 4,094.00 and closes at 4,108.00. The body runs from 4,100 to 4,108: buyers won the hour by $8. The upper wick shows price reached 4,112 but couldn't hold there; the lower wick shows sellers pushed down to 4,094 and lost. (Illustrative prices.)"
mistake = "Reading a candle as a prediction: “a big buyers' candle, so price will keep rising.” A candle records what already happened in its period. The next one can go either way; where a candle forms matters more than its size, and Stage 5 shows why."
takeaways = [
  "Each candle shows four prices for one period: open, high, low and close (OHLC).",
  "The body runs from open to close: close above open means buyers won the period, below means sellers won.",
  "The wicks reach the high and the low: how far price travelled before being pushed back.",
  "A candle is a record of what happened, not a forecast of what happens next.",
]
drills = ["build_candle"]

[[steps]]
heading = "Four prices in one shape"
text = "Each candle covers one period: one hour on a 1-hour chart, one day on a daily chart. It records four prices: the **open** (the first price of the period), the **high**, the **low** and the **close** (the last price). Traders call them **OHLC**."
why = "CME Group's chart lessons describe the candle in two parts: the body for the open and close, and the wick for the highest and lowest prices traded. A bar chart shows the same four prices with small ticks instead of a body."

[[steps]]
heading = "The body: open to close"
text = "The thick part, the **body**, runs from the open to the close. Close **above** the open: buyers won the period. Close **below**: sellers won. In this app, hollow bodies closed higher and filled ones lower; most platforms colour them green and red instead."
why = "Colours are a setting, so check your platform's legend. When the open and close are almost the same, the body shrinks to a thin line: neither side won. Traders call that a doji."
widget = "candle_builder"

[[steps]]
heading = "The wicks: how far price travelled"
text = "The thin lines above and below the body are the **wicks** (also called shadows). The tip of the upper wick is the **high**; the tip of the lower wick is the **low**. A long wick means price went there and was pushed back before the close."
why = "CME Group puts it this way: a short or missing wick means one side was in control for the whole period; long wicks mean both buyers and sellers were active. A long upper wick on a sellers' candle: buyers pushed up first, then sellers took over."

[[steps]]
heading = "Size shows strength"
text = "A long body means price moved a lot from open to close: one side pushed hard. A short body means the open and close were near each other: not much strength either way. Measure both in Dollar Moves: a body from 4,100.00 to 4,108.00 is an $8 move."
why = "“Long” only means long compared with the candles around it on the same chart. An $8 body is big on a 5-minute chart and ordinary on a daily one."

[[steps]]
heading = "A record, not a forecast"
text = "A candle tells you what happened inside its period. It doesn't tell you what happens next: a strong buyers' candle can be followed by a sellers' one. That's why you never trade a candle on its own; Stage 5 shows where candles start to matter: at levels."
why = "Remember the drill from Lesson 2: calling the next candle is close to a coin flip. A confirmation candle only counts at a location, with the trend (Lesson 26)."

[[videos]]
id = "jmoOrgTP5XQ"
title = "Candlestick charts: The ULTIMATE beginners guide to reading a candlestick chart"
channel = "The Trading Channel"
duration = "16:25"
watch_for = "How the body and wicks are built from the open, high, low and close. Skip the pattern names for now: Stage 5 comes back to them."
fallback_search = "how to read candlestick charts beginners"
checked = "2026-11-13"

[[videos]]
id = "zEBvX_El3LY"
title = "Forex Trading for Beginners #7: How to Read a Forex Chart"
channel = "Rayner Teo"
duration = "5:30"
watch_for = "How to read a candle chart and switch timeframes. It's a currency chart, but candles read the same on gold; where he talks in pips, this course uses Dollar Moves."
fallback_search = "how to read a price chart candlesticks beginners"
checked = "2026-11-13"

[[sources]]
claim = "A candlestick has a body showing the open and close prices and a wick showing the highest and lowest prices traded over the time frame; the bar is generally green when the close is above the open and red when below, depending on the software's colour settings."
name = "CME Group Education: Chart Types: candlestick, line, bar"
url = "https://www.cmegroup.com/education/courses/technical-analysis/chart-types-candlestick-line-bar.html"
checked = "2026-11-13"

[[sources]]
claim = "The longer the body, the more price moved over the period; a short or missing wick means one side was in control for the whole period; long wicks mean buyers and sellers were both active."
name = "CME Group Education: Chart Types: candlestick, line, bar"
url = "https://www.cmegroup.com/education/courses/technical-analysis/chart-types-candlestick-line-bar.html"
checked = "2026-11-13"

[[quiz]]
id = "q1"
q = "A 1-hour candle: open 4,100.00, high 4,112.00, low 4,094.00, close 4,108.00. Where does its body run?"
options = ["From 4,100.00 to 4,108.00", "From 4,094.00 to 4,112.00", "From 4,108.00 to 4,112.00", "From 4,094.00 to 4,100.00"]
answer = 0
why = "The body runs from the open to the close. The high and low are the tips of the wicks."

[[quiz]]
id = "q2"
q = "A candle opens at 4,120.00 and closes at 4,111.00. Who won the period?"
options = ["Sellers: it closed $9 below its open", "Buyers: it closed $9 above its open", "Nobody: candles don't show that", "Buyers, because the high was above the open"]
answer = 0
why = "Close below open means sellers won; the body is a $9 move down."

[[quiz]]
id = "q3"
q = "What does the tip of the upper wick show?"
options = ["The highest price of the period", "The closing price", "The next price to come", "The average price"]
answer = 0
why = "The upper wick reaches the high; the lower wick reaches the low."

[[quiz]]
id = "q4"
q = "A candle has a small body near its low and a long upper wick. What happened during the period?"
options = ["Price rose a long way, then was pushed back down before the close", "Price never moved", "Buyers held their gains to the close", "The broker made an error"]
answer = 0
why = "The long upper wick shows how far price went up; the close near the low shows it didn't hold."

[[quiz]]
id = "q5"
q = "Two candles on the same chart: one has a $2 body, the other a $14 body. What does the bigger body tell you?"
options = ["Price moved much further from open to close in that period", "Price will keep moving that way", "That candle lasted longer", "That candle had a wider spread"]
answer = 0
why = "Body size shows how far price moved in the period. It says nothing certain about the next one."

[[quiz]]
id = "q6"
q = "Three strong buyers' candles in a row. What does that tell you about the next candle?"
options = ["Nothing certain: candles record what happened, not what happens next", "It will be a buyers' candle too", "It will be a sellers' candle", "It will be bigger"]
answer = 0
why = "A candle is a record, not a forecast. Where it forms matters, and that's Stage 5."

[[recall]]
id = "r1"
q = "The body of a candle runs from…"
options = ["The open to the close", "The high to the low", "The close to the next open"]
answer = 0
why = "The wicks cover the high and the low."

[[recall]]
id = "r2"
q = "A candle that closes below its open was won by…"
options = ["Sellers", "Buyers", "Nobody"]
answer = 0
why = "Close below open: sellers won the period."

[[recall]]
id = "r3"
q = "Does a big candle tell you where price goes next?"
options = ["No, it records what already happened", "Yes, always the same way"]
answer = 0
why = "A record, not a forecast."

[explain_back]
prompt = "What does one candle tell you, and what can't it tell you?"
must_include = ["a candle shows four prices for its period: open, high, low and close", "the body runs from open to close and shows who won (close above the open: buyers; below: sellers), while the wicks reach the high and the low", "it records what already happened in that period, not what happens next"]
misconceptions = ["a big candle predicts the next move", "the colour or the wicks decide who won rather than the open and close"]

[[explain_back.fixtures]]
answer = "A candle shows the open, high, low and close for its period. The body goes from the open to the close: if it closed above the open buyers won, below means sellers won. The wicks show how high and low price went. But it only tells me what already happened in that hour or day, not what the next candle will do."
expect = "got_it"

[[explain_back.fixtures]]
answer = "A candle shows the open, high, low and close. The body is from open to close and tells you if buyers or sellers won, and the wicks show the high and the low."
expect = "almost"

[[explain_back.fixtures]]
answer = "A green candle means price will keep going up, and the wicks show where buyers will step in next time."
expect = "almost_or_not_yet"
+++
