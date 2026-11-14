+++
id = 26
stage = 5
title = "Confirmation candles"
objective = "recognise a pin bar, an engulfing candle and an inside bar, and explain why one only counts at a location, after it closes."
hook = "Price reaches your support zone. Do you buy now, or wait for something first?"
analogy = "Think of a handshake. Walking into a meeting, you wait for the other person to offer their hand before you shake it. A confirmation candle is the market offering its hand at your level: proof that buyers or sellers actually showed up there, not just a hope that they will."
analogy_tag = "handshake"
example = "Gold falls into your support zone at 4,080 to 4,084. An H1 candle dips to 4,076, then closes at 4,089 with a long lower wick: a bullish pin bar. Sellers pushed below the zone and were rejected. You'd look to buy after the close, with a stop below the wick, around 4,074. (Illustrative prices.)"
mistake = "Trading a pattern in the middle of nowhere. A pin bar between levels, or against the trend, is just a candle. Location first, then the candle."
takeaways = [
  "A confirmation candle shows the other side was rejected at your level.",
  "Pin bar: a long wick and a small body at the other end. Engulfing: a body that covers the previous body the other way. Inside bar: within the previous candle's range.",
  "Act only after the candle closes, never while it's forming.",
  "A pattern only counts at a location: at a zone, with the higher-timeframe trend.",
]
drills = ["name_candle"]

[[steps]]
heading = "Why wait for confirmation"
text = "At a zone, price can bounce or break. A **confirmation candle** is evidence that the other side showed up: price came into the zone and was pushed back. You act after it **closes**, never while it's still forming."
why = "A candle can look like a perfect pin bar at minute 50 and close as something else at minute 60. Only the close counts."

[[steps]]
heading = "The pin bar"
text = "A **pin bar** has a long wick on one side, about two thirds of the candle or more, and a small body at the other end. A long lower wick at support: buyers rejected lower prices (**bullish**). A long upper wick at resistance: **bearish**."
why = "CME Group explains it through the body: a small body near the top of the candle means buyers had control by the close; near the bottom, sellers did. The long wick shows where the other side failed."

[[steps]]
heading = "The engulfing candle"
text = "An **engulfing** candle's body covers the whole body of the candle before it, in the other direction. A bullish engulfing at support: sellers had one candle, then buyers took all of it back, and more."
why = "Compare bodies, not wicks: the engulfing candle's open-to-close must cover the previous candle's open-to-close."

[[steps]]
heading = "The inside bar"
text = "An **inside bar** sits entirely within the previous candle's high and low: the market paused. At a level, traders wait for price to break out of the bigger candle's range, in the trend's direction."
why = "On its own an inside bar only says “pause”. Its direction comes from the break that follows, so it needs more patience than a pin bar."

[[steps]]
heading = "Location first"
text = "A pattern only counts **at a location**: at a zone, in the higher-timeframe direction (Lessons 22–24). The same pin bar between levels, or against the trend, is just a candle. Location, then confirmation, then the trade, with a stop beyond the wick."
why = "This is the core of Classic Price Action: a confirmation candle at a level, in the trend's direction. Lessons 27 and 28 turn it into two complete plays."

[[videos]]
id = "DC-ghSmJy9s"
title = "A pinbar trading strategy that works"
channel = "Rayner Teo"
duration = "13:13"
watch_for = "How he reads a pin bar only at a level, in context. His exact entries are examples, not this course's rules."
fallback_search = "pin bar candlestick explained"
checked = "2026-11-13"

[[videos]]
id = "k98oyKtGUns"
title = "Candlestick Pattern Trading #3: What is a Bullish Engulfing Pattern"
channel = "Rayner Teo"
duration = "4:05"
watch_for = "What makes a candle engulfing: the body covering the previous body."
fallback_search = "bullish engulfing pattern explained"
checked = "2026-11-13"

[[sources]]
claim = "A small body near the low of a candle means sellers had taken control by its end; a small body near the top means buyers had; long wicks mean both sides were active."
name = "CME Group Education: Chart Types: candlestick, line, bar"
url = "https://www.cmegroup.com/education/courses/technical-analysis/chart-types-candlestick-line-bar.html"
checked = "2026-11-13"

[[quiz]]
id = "q1"
q = "At support, an H1 candle dips well below the zone, then closes near its high, leaving a long lower wick. What is it?"
options = ["A bullish pin bar", "A bearish pin bar", "An inside bar", "A bearish engulfing"]
answer = 0
why = "A long lower wick and a small body near the top: lower prices were rejected."

[[quiz]]
id = "q2"
q = "A bearish candle, then a bullish candle whose body covers all of the bearish body. What is that?"
options = ["A bullish engulfing", "An inside bar", "A bearish pin bar", "No pattern"]
answer = 0
why = "The new body engulfs the previous one, the other way."

[[quiz]]
id = "q3"
q = "A candle's high and low are both inside the previous candle's range. What is it?"
options = ["An inside bar: the market paused", "A bullish engulfing", "A guaranteed breakout", "A pin bar"]
answer = 0
why = "Inside the previous range is a pause; the break that follows gives direction."

[[quiz]]
id = "q4"
q = "It's minute 45 of an H1 candle at your zone, and it looks like a pin bar. What do you do?"
options = ["Wait for the candle to close", "Enter now before it's too late", "Move the stop closer", "Switch to M1"]
answer = 0
why = "Only the close counts; the candle can still change shape."

[[quiz]]
id = "q5"
q = "A perfect bullish pin bar forms halfway between two zones, in an H4 downtrend. Is it a setup?"
options = ["No: it's not at a location and it's against the trend", "Yes, every pin bar is a buy", "Yes, if the wick is long", "Only if it's green"]
answer = 0
why = "Location first. Elsewhere, it's just a candle."

[[quiz]]
id = "q6"
q = "After a bullish pin bar closes at support with its low at 4,076, where does the stop go?"
options = ["Below the wick and the zone, around 4,074", "Just above the entry", "In the middle of the zone", "No stop needed: it's confirmed"]
answer = 0
why = "Beyond the wick: if price goes back below it, the rejection failed."

[[recall]]
id = "r1"
q = "A pin bar has…"
options = ["A long wick and a small body at the other end", "No wicks at all", "Two bodies"]
answer = 0
why = "The long wick shows the rejected side."

[[recall]]
id = "r2"
q = "You act on a confirmation candle…"
options = ["After it closes", "While it's forming", "Before price reaches the zone"]
answer = 0
why = "The close decides what it really is."

[[recall]]
id = "r3"
q = "A confirmation candle counts…"
options = ["Only at a location, with the trend", "Anywhere on the chart"]
answer = 0
why = "Location first, then the candle."

[explain_back]
prompt = "What is a confirmation candle, and when does one count?"
must_include = ["a confirmation candle shows the other side was rejected at a level: a pin bar, an engulfing candle or an inside bar", "you wait for the candle to close before acting", "it only counts at a location (a zone, with the higher-timeframe trend); anywhere else it's just a candle"]
misconceptions = ["any pin bar is a signal to trade", "you can enter while the candle is still forming"]

[[explain_back.fixtures]]
answer = "A confirmation candle is a candle at my level that shows the other side got rejected, like a pin bar with a long wick, an engulfing candle or an inside bar. I wait for it to close before doing anything. And it only counts at a location, at a zone and with the higher-timeframe trend; the same pattern in the middle of nowhere is just a candle."
expect = "got_it"

[[explain_back.fixtures]]
answer = "It's a pin bar, an engulfing or an inside bar that shows buyers or sellers were rejected, and you wait for it to close."
expect = "almost"

[[explain_back.fixtures]]
answer = "Any time I see a pin bar I buy straight away, even before the candle closes, because it means price will go up."
expect = "almost_or_not_yet"
+++
