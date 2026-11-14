+++
id = 27
stage = 5
title = "The pullback play"
objective = "trade a pullback in a trend step by step: zone, confirmation, stop, target, and say when to skip it."
hook = "Gold is trending up nicely, but you missed the start. Do you chase it, or wait for it to come back to you?"
analogy = "Think of the tide coming in. Even as the water rises, each wave slides back before the next one reaches further up the sand. You don't chase a wave up the beach; you wait for it to slide back, then step in as the next one starts."
analogy_tag = "tide and waves"
example = "H4 uptrend, last higher low at 4,090. On H1, price pulls back into the 4,088 to 4,093 zone, old resistance now acting as support, and a bullish engulfing closes at 4,096. Entry 4,096, stop below the pattern at 4,085: $11 at risk. Target the last high at 4,118: $22 away, twice the risk. Size it so a loss is 1% of the account. (Illustrative prices.)"
mistake = "Buying the pullback before it stops. Price falling into a zone isn't a setup until a confirmation candle closes there; some pullbacks keep going and become reversals (a CHoCH, Lesson 25)."
takeaways = [
  "The pullback play needs a clear higher-timeframe trend.",
  "Wait for price to pull back into a zone, then for a confirmation candle in the trend's direction to close.",
  "Stop beyond the candle's wick and the zone; target the next zone; at least twice the risk.",
  "No clear trend, no zone, no confirmation or not enough reward: skip it, and log it as skipped.",
]
drills = ["name_candle"]

[[steps]]
heading = "Trend first"
text = "The pullback play only exists in a **trend**: on H4, higher highs and higher lows, or the reverse (Lesson 25). No clear trend, no pullback play. It's the first of Classic Price Action's two plays."
why = "Beginners usually learn it first because it trades with the bigger move instead of against it."

[[steps]]
heading = "Wait for the pullback to a zone"
text = "Instead of chasing, wait for price to **pull back** against the trend into a zone: often old resistance turned support (Lesson 24), or near the last higher low. Watch it arrive on H1."
why = "Chasing means buying after the move, with your stop far away. Waiting puts your entry near a level, so the stop can be closer and the trade can be sized properly."

[[steps]]
heading = "Confirmation at the zone"
text = "Once price is in the zone, wait for a **confirmation candle** in the trend's direction (Lesson 26) to **close**. No confirmation, no trade: the pullback might keep going and turn into a reversal."
why = "If price breaks the last higher low instead, that's a CHoCH: the trend may be ending, so the play is off."

[[steps]]
heading = "Stop, target and size"
text = "Put the **stop** beyond the confirmation candle's wick and the zone. Aim for the **next zone** in the trend's direction, usually the last high. The reward should be at least **twice** the risk; if not, skip. Then size it so a loss is about 1%."
why = "The position-size calculator turns the stop distance into a lot size (Lessons 8 and 9). Stage 6 adds Fibonacci to refine where pullbacks tend to pause."

[[steps]]
heading = "When to skip"
text = "Skip it when the H4 trend isn't clear, the zone is in the middle of nowhere, there's no confirmation, the reward is under twice the risk, or a No-Trade Window is near (Lesson 15). A skipped setup is a good decision; log it as skipped."
why = "Skipped setups go in your journal too. They never count towards Live Readiness, but they show your plan working."

[[videos]]
id = "LpWz8hswCQo"
title = "Pullback Trading Explained (For Beginners)"
channel = "Rayner Teo"
duration = "6:43"
watch_for = "Why trading the pullback beats chasing the move. His exact entry rules may differ from this course's."
fallback_search = "pullback trading explained beginners"
checked = "2026-11-13"

[[videos]]
id = "4MPR-7bickE"
title = "Do You Make This Pullback Trading Mistake?"
channel = "Rayner Teo"
duration = "20:05"
watch_for = "The mistake of buying a pullback before it shows signs of stopping."
fallback_search = "pullback trading mistake"
checked = "2026-11-13"

[[sources]]
claim = "Markets tend to pause at previous highs and lows, and broken resistance tends to become support when price moves back down to it."
name = "CME Group Education: Support and Resistance"
url = "https://www.cmegroup.com/education/courses/technical-analysis/support-and-resistance.html"
checked = "2026-11-13"

[[sources]]
claim = "The 1-hour chart is useful for setting up trades that align with the broader market trend."
name = "CME Group Education: Trading Simulator How-To Guide: Chart Time Frames"
url = "https://www.cmegroup.com/education/courses/using-the-trading-simulator/trading-sim-in-30-seconds-chart-time-frames.html"
checked = "2026-11-13"

[[quiz]]
id = "q1"
q = "What does the pullback play need before anything else?"
options = ["A clear higher-timeframe trend", "A round number", "A big news release", "Three indicators agreeing"]
answer = 0
why = "No trend, no pullback."

[[quiz]]
id = "q2"
q = "H4 is in an uptrend. Price pulls back into a support zone on H1. What do you wait for?"
options = ["A bullish confirmation candle to close at the zone", "Nothing: buy on arrival", "A bearish engulfing", "The weekend"]
answer = 0
why = "Confirmation in the trend's direction, after the close."

[[quiz]]
id = "q3"
q = "Entry 4,096, stop 4,085. The next zone is at 4,118. Is the reward at least twice the risk?"
options = ["Yes: $22 of reward for $11 of risk", "No: it's less than 1:1", "Exactly 1:1", "You can't tell without indicators"]
answer = 0
why = "4,118 − 4,096 = $22; 4,096 − 4,085 = $11; 22 ÷ 11 = 2."

[[quiz]]
id = "q4"
q = "Entry 4,096, stop 4,085, next zone 4,104. What do you do?"
options = ["Skip: $8 of reward for $11 of risk isn't enough", "Take it anyway", "Move the stop to 4,095", "Double the size"]
answer = 0
why = "Under twice the risk, skip. Never shrink the stop to make the numbers work."

[[quiz]]
id = "q5"
q = "During the pullback, price closes below the last higher low. What now?"
options = ["The play is off: that's a CHoCH, the trend may be ending", "Buy more at the lower price", "It's a stronger setup", "Move the stop lower"]
answer = 0
why = "A broken higher low means the trend's structure has changed."

[[quiz]]
id = "q6"
q = "Where does the stop go in the pullback play?"
options = ["Beyond the confirmation candle's wick and the zone", "Exactly at the entry", "Inside the zone", "Wherever keeps the loss smallest"]
answer = 0
why = "Beyond the wick and zone, so normal noise doesn't hit it."

[[recall]]
id = "r1"
q = "The pullback play trades…"
options = ["With the trend, after a pause", "Against the trend", "Only in ranges"]
answer = 0
why = "Trend first, then the pullback."

[[recall]]
id = "r2"
q = "The minimum reward for the risk is…"
options = ["Twice the risk", "Equal to the risk", "Half the risk"]
answer = 0
why = "Below 2:1, skip it."

[[recall]]
id = "r3"
q = "A skipped setup is…"
options = ["A good decision, logged as skipped", "A failure"]
answer = 0
why = "Following the plan includes skipping."

[explain_back]
prompt = "Walk through the pullback play from start to finish. When would you skip it?"
must_include = ["it needs a clear higher-timeframe trend; you wait for price to pull back into a zone instead of chasing", "you enter only after a confirmation candle in the trend's direction closes at the zone", "the stop goes beyond the candle's wick and the zone and the target at the next zone, with at least twice the risk; otherwise, or with no trend, zone or confirmation, you skip"]
misconceptions = ["you can buy as soon as price pulls back", "a pullback always continues the trend"]

[[explain_back.fixtures]]
answer = "First I need a clear trend on H4. Then I wait for price to pull back into a zone instead of chasing it, and only enter once a confirmation candle in the trend's direction closes there. My stop goes beyond that candle's wick and the zone, and my target is the next zone; if that's less than twice my risk, or there's no clear trend, zone or confirmation, I skip it and log it as skipped."
expect = "got_it"

[[explain_back.fixtures]]
answer = "In an H4 trend I wait for a pullback to a zone, then a confirmation candle, with my stop beyond the wick and my target at the next zone."
expect = "almost"

[[explain_back.fixtures]]
answer = "When price drops in an uptrend I buy right away, because pullbacks always continue the trend."
expect = "almost_or_not_yet"
+++
