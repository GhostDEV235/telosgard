+++
id = 22
stage = 4
title = "Timeframes & top-down analysis"
objective = "explain what a timeframe is, use a higher timeframe for direction and a lower one for entries, and say what to do when they disagree."
hook = "The 15-minute chart says gold is falling. The 4-hour chart says it's rising. Which one is right?"
analogy = "Think of a map app. Zoomed out, you see which way the road runs across the country. Zoomed in, you see the next turn. You plan the route on the big map and steer on the small one. If the close-up seems to point the other way, it's a bend in the road: the big map decides where you're going."
analogy_tag = "map zoom"
example = "Gold's 4-hour chart has climbed for two weeks, from 4,020.00 to 4,110.00. On the 1-hour chart, price dips to 4,092.00 and starts to turn back up. Read top-down, that's a dip inside an uptrend: a place to look for a buy, with a stop below the dip. Selling it would fight the bigger trend. (Illustrative prices.)"
mistake = "Flicking through timeframes until one agrees with the trade you already want. Choose your two timeframes before you look, read the higher one first, and let it decide the direction."
takeaways = [
  "A timeframe is how much time each candle covers: M5, H1, H4, D1. Bigger candles are smaller ones grouped together.",
  "Top-down: read the higher timeframe first for direction (bias), then the lower one for the entry.",
  "This course's starting pair is H4 for direction and H1 for entries; swing traders use D1 and H4.",
  "When timeframes disagree, the higher one wins: trade with it, or stand aside.",
]

[[steps]]
heading = "One market, many timeframes"
text = "The same gold prices can be drawn with one candle every 5 minutes, every hour, every 4 hours or every day. These are **timeframes**, written **M5, H1, H4 and D1**. A higher timeframe shows more history with fewer, bigger candles."
why = "Four 15-minute candles make one 1-hour candle: its open is the first open, its close the last close, and its high and low are the extremes of all four. Nothing new happened; the same trades are just grouped differently."
widget = "candle_merge"

[[steps]]
heading = "Top-down: big picture first"
text = "**Top-down analysis** means reading the higher timeframe first, then stepping down. The higher chart gives your **bias**: the direction you'd trade in. The lower chart times the **entry**. CME Group's lessons describe using shorter charts for trades that **align with the broader trend**."
why = "Bias is simply your preferred direction for the session: up, down or none. With no clear bias, the plan for the day can be to wait, and that's a good plan."

[[steps]]
heading = "The beginner pair: H4 and H1"
text = "This course starts you with two charts: **H4 for direction** and **H1 for entries**. Swing traders, who hold trades for days, step up to **D1 and H4**. Two charts are enough: every extra timeframe is another way to talk yourself into a trade."
why = "The pair is this course's choice, not a law of the market. Very low timeframes like M5 show more noise, and costs such as the spread (Lesson 6) eat a bigger share of small moves."

[[steps]]
heading = "When timeframes disagree"
text = "They often do. H4 rising while H1 falls is normal: that's a **pullback** inside the bigger move. The higher timeframe wins: in an H4 uptrend you look for buys on H1, not sells. If H4 has no clear direction, standing aside is a valid plan."
why = "A pullback inside a trend is exactly where Classic Price Action looks for its entry (Lesson 27). Fighting the higher timeframe means trading against most of the market's recent momentum."

[[videos]]
id = "5N3X2WPnG2Q"
title = "How to Analyze Multiple Timeframes (Without Getting Confused)"
channel = "Rayner Teo"
duration = "3:39"
watch_for = "How a higher timeframe sets the direction and a lower one the timing. His exact pair may differ; this course starts with H4 and H1."
fallback_search = "multiple timeframe analysis beginners"
checked = "2026-11-13"

[[videos]]
id = "POJFoQYVXnM"
title = "Which is the best timeframe to trade (for beginners)?"
channel = "Rayner Teo"
duration = "5:17"
watch_for = "Why beginners usually do better on higher timeframes than on very short ones."
fallback_search = "best timeframe to trade for beginners"
checked = "2026-11-13"

[[sources]]
claim = "Charts can be set to different time frames; a longer-term trader may track weekly or monthly charts while a shorter-term trader may use 60-minute or 5-minute charts."
name = "CME Group Education: Chart Types: candlestick, line, bar"
url = "https://www.cmegroup.com/education/courses/technical-analysis/chart-types-candlestick-line-bar.html"
checked = "2026-11-13"

[[sources]]
claim = "The 1-hour chart is useful for setting up trades that align with the broader market trend, the daily chart gives the bigger picture, and combining time frames covers both short- and long-term movements."
name = "CME Group Education: Trading Simulator How-To Guide: Chart Time Frames"
url = "https://www.cmegroup.com/education/courses/using-the-trading-simulator/trading-sim-in-30-seconds-chart-time-frames.html"
checked = "2026-11-13"

[[quiz]]
id = "q1"
q = "On an H1 chart, what does each candle cover?"
options = ["One hour", "One day", "One minute", "One trade"]
answer = 0
why = "H1 = one hour per candle; H4 = four hours; D1 = one day."

[[quiz]]
id = "q2"
q = "In this course's beginner pair, which timeframe sets the direction?"
options = ["H4", "H1", "M5", "Whichever looks best today"]
answer = 0
why = "H4 for direction, H1 for entries."

[[quiz]]
id = "q3"
q = "H4 is in a clear uptrend. On H1, price has been falling for six candles. What is that, read top-down?"
options = ["Probably a pullback inside the uptrend: look for buys, not sells", "A signal to sell immediately", "Proof the H4 trend is over", "A reason to switch to M5"]
answer = 0
why = "The higher timeframe wins; a dip on the lower one is often a pullback, where buyers may return."

[[quiz]]
id = "q4"
q = "Four 15-minute candles: opens at 4,100, 4,104, 4,109, 4,101; highs 4,105, 4,112, 4,110, 4,106; lows 4,098, 4,102, 4,099, 4,100; the last closes at 4,105. What is the 1-hour candle?"
options = ["Open 4,100, high 4,112, low 4,098, close 4,105", "Open 4,101, high 4,106, low 4,100, close 4,105", "Open 4,100, high 4,105, low 4,098, close 4,104", "Open 4,105, high 4,112, low 4,098, close 4,100"]
answer = 0
why = "First open, highest high, lowest low, last close."

[[quiz]]
id = "q5"
q = "A swing trader holds trades for several days. Which pair suits them?"
options = ["D1 for direction, H4 for entries", "M1 and M5", "H1 for direction, M5 for entries", "Only the weekly chart"]
answer = 0
why = "Step both timeframes up: D1 and H4."

[[quiz]]
id = "q6"
q = "You want to sell. H4 is rising, so you check H1, then M15, then M5 until one is falling. What went wrong?"
options = ["You searched for a chart to agree with you instead of letting the higher timeframe decide", "Nothing: more timeframes is always better", "You should have started on M1", "You should have used a bigger lot"]
answer = 0
why = "Pick the timeframes first, read the higher one first, and follow it."

[[recall]]
id = "r1"
q = "Top-down analysis starts with…"
options = ["The higher timeframe", "The lowest timeframe", "The latest candle"]
answer = 0
why = "Big picture first, then the entry."

[[recall]]
id = "r2"
q = "This course's beginner pair is…"
options = ["H4 for direction, H1 for entries", "M1 and M5", "D1 only"]
answer = 0
why = "Swing traders step up to D1 and H4."

[[recall]]
id = "r3"
q = "When the timeframes disagree…"
options = ["The higher one wins, or you stand aside", "The lower one wins", "You trade both ways"]
answer = 0
why = "A lower-timeframe move against the trend is often just a pullback."

[explain_back]
prompt = "How do you use two timeframes together, and what do you do when they disagree?"
must_include = ["the higher timeframe sets the direction (bias) and the lower one times the entry: H4 then H1 in this course", "you read the higher timeframe first, before looking at the lower one", "when they disagree, the higher timeframe wins: you trade with it or stand aside, never against it"]
misconceptions = ["the lowest timeframe is the most accurate", "you should switch timeframes until one agrees with your trade"]

[[explain_back.fixtures]]
answer = "I look at H4 first to decide the direction, then use H1 to find the entry. If they disagree, say H4 is going up and H1 is going down, I treat it as a pullback and only look for buys, or I stay out. I don't sell against the H4 trend."
expect = "got_it"

[[explain_back.fixtures]]
answer = "I use H4 to see the trend and H1 to time my entry, looking at H4 first."
expect = "almost"

[[explain_back.fixtures]]
answer = "The smallest timeframe is the most accurate, so I check M1 and M5 and change charts until one shows the move I want."
expect = "almost_or_not_yet"
+++
