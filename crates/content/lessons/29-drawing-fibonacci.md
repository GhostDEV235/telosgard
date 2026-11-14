+++
id = 29
stage = 6
title = "Drawing Fibonacci correctly"
objective = "draw a Fibonacci grid on the right leg: a clean impulse that broke structure, from the swing low to the swing high in an uptrend (high to low in a downtrend), wick to wick."
hook = "Two traders put Fibonacci on the same gold chart and get different levels. Can they both be right?"
analogy = "Think of a tape measure. Hook it on the exact start of the plank and pull it to the exact end, and every mark in between is right. Hook it a few inches in, or run it backwards, and every mark after that is wrong, however carefully you read it."
analogy_tag = "tape measure"
example = "H4 uptrend. Gold rallies from a swing low at 4,060 to a swing high at 4,100, closing above the old high at 4,088 on the way: a BOS, so this is the impulse leg. You draw from the low wick at 4,060 to the high wick at 4,100, a $40 leg. The tool then marks 38.2% at 4,084.72, 50% at 4,080.00 and 61.8% at 4,075.28: how far a pullback has given back. (Illustrative prices.)"
mistake = "Drawing it backwards. In an uptrend, dragging from the high down to the low flips the grid: the line labelled 38.2% lands where 61.8% belongs, and every level you watch is in the wrong place."
takeaways = [
  "Fibonacci measures how far a pullback has gone back into the last move: 38.2%, 50% and 61.8% are the main levels.",
  "Draw it on the last clean impulse leg that broke structure, on your trend timeframe.",
  "Uptrend: from the swing low to the swing high. Downtrend: from the swing high to the swing low. Wick to wick.",
  "A new high before the pullback means a new leg: redraw. Price below the leg's low means the leg is broken: delete it.",
]
drills = ["pick_leg"]

[[steps]]
heading = "What Fibonacci measures"
text = "The **Fibonacci** tool marks how far a pullback has gone back into the last move. Its main levels are **38.2%**, **50%** and **61.8%** of that move. CME Group's course notes that 50% isn't a Fibonacci ratio at all, but traders watch it anyway."
why = "The ratios come from the Fibonacci sequence (0, 1, 1, 2, 3, 5, 8, 13…, each number the sum of the two before it): divide a number by the one two places later and you get about 0.382, and 1 − 0.618 gives the same."

[[steps]]
heading = "Pick the right leg"
text = "Draw it on the **impulse leg**: the last strong move in the trend's direction that **broke structure** (a BOS, Lesson 25), on your trend timeframe, H4 in this course. Not on the pullback that's happening now, and not on an older leg."
why = "A leg that didn't break structure is just noise inside a range. Most of the disagreement between two traders' grids comes from picking different legs, not from the tool."

[[steps]]
heading = "Low to high in an uptrend"
text = "In an **uptrend**, draw from the **swing low** to the **swing high**. In a **downtrend**, from the swing high to the swing low. The tool then measures pullbacks back against the move. Drawn the wrong way round, the grid flips and its labels point at the wrong prices."
why = "A quick check: in an uptrend, the 0% line should sit at the top (the high) and 100% at the bottom (the low), with 38.2% nearest the high."

[[steps]]
heading = "Wick to wick"
text = "Anchor on the leg's **extreme points**: the lowest wick and the highest wick, not the candle bodies. Fidelity's guide describes the tool as drawn between two extreme points, and that's what everyone else watching the chart will use."
why = "Levels only matter because many traders watch the same ones. Anchoring on bodies moves your levels a few dollars away from theirs."

[[steps]]
heading = "Redraw, or delete"
text = "A grid belongs to one leg. If price makes a **new high** before pulling back, the leg ends there: **redraw** from the same low. If price falls **below the leg's low**, the leg is broken: **delete** the grid; its levels no longer mean anything."
why = "Lesson 30 adds a stricter line this course uses: a close beyond 78.6% already ends the pullback play."

[[videos]]
id = "mnEe3uM4pyQ"
title = "How to Draw FIBONACCI Retracement For Trading"
channel = "Mind Math Money"
duration = "2:59"
watch_for = "Which swing he anchors on, and the direction he drags in an uptrend and a downtrend."
fallback_search = "how to draw fibonacci retracement correctly"
checked = "2026-11-13"

[[videos]]
id = "nYKpik-o8zI"
title = "Fibonacci Retracement explained in under 5 minutes"
channel = "The Moving Average"
duration = "3:49"
watch_for = "A short overview of the levels and why the leg you pick matters. Treat any win rates mentioned as examples, not promises."
fallback_search = "fibonacci retracement explained"
checked = "2026-11-13"

[[sources]]
claim = "The Fibonacci series adds the two preceding numbers; dividing a number by the one two places to the right gives 38.2%, also 1 − 0.618; 50% is not a Fibonacci ratio but is included by traders; 61.8, 38.2 and 50 form the basis of Fibonacci studies."
name = "CME Group Education: Fibonacci Retracements and Extensions"
url = "https://www.cmegroup.com/education/courses/technical-analysis/fibonacci-retracements-and-extensions"
checked = "2026-11-13"

[[sources]]
claim = "Fibonacci retracements are displayed by first drawing a trend line between two extreme points; the percentages are applied to the difference between the high and the low."
name = "Fidelity Learning Center: Fibonacci Retracement"
url = "https://www.fidelity.com/learning-center/trading-investing/technical-analysis/technical-indicator-guide/fibonacci-retracement"
checked = "2026-11-13"

[[quiz]]
id = "q1"
q = "What does the Fibonacci tool measure?"
options = ["How far a pullback has gone back into the last move", "Where price will turn next, for certain", "The size of your position", "The spread"]
answer = 0
why = "It marks levels to watch, not a forecast."

[[quiz]]
id = "q2"
q = "H4 uptrend. Which leg do you draw Fibonacci on?"
options = ["The last strong move up that broke structure", "The pullback happening now", "Any leg you like, as long as it's long", "The first move of the week"]
answer = 0
why = "The last clean impulse that broke structure (a BOS)."

[[quiz]]
id = "q3"
q = "In an uptrend, which way do you draw it?"
options = ["From the swing low to the swing high", "From the swing high to the swing low", "From the current price to the high", "It doesn't matter"]
answer = 0
why = "Low to high in an uptrend; high to low in a downtrend. Backwards flips the levels."

[[quiz]]
id = "q4"
q = "The leg runs from 4,060 to 4,100. Where is the 50% level?"
options = ["4,080.00", "4,075.28", "4,084.72", "4,100.00"]
answer = 0
why = "Half of $40 is $20: 4,100 − 20 = 4,080. 4,075.28 is 61.8% and 4,084.72 is 38.2%."

[[quiz]]
id = "q5"
q = "Before pulling back, price makes a new high at 4,112. What happens to your grid drawn to 4,100?"
options = ["Redraw it from the same low to the new high", "Keep it: the old levels still count", "Delete Fibonacci for good", "Move the low up"]
answer = 0
why = "The leg ended at the new high, so the grid moves with it."

[[quiz]]
id = "q6"
q = "Where do you anchor the grid on each end?"
options = ["On the extreme wicks of the leg", "On the candle bodies", "On round numbers near the swing", "Wherever looks tidiest"]
answer = 0
why = "Wick to wick: the leg's real high and low, the points other traders use."

[[recall]]
id = "r1"
q = "The three main Fibonacci levels are…"
options = ["38.2%, 50% and 61.8%", "10%, 20% and 30%", "100%, 200% and 300%"]
answer = 0
why = "50% isn't a Fibonacci ratio, but traders watch it."

[[recall]]
id = "r2"
q = "In a downtrend, you draw from…"
options = ["The swing high to the swing low", "The swing low to the swing high"]
answer = 0
why = "Always from the start of the move to its end."

[[recall]]
id = "r3"
q = "Price falls below the leg's low. Your grid…"
options = ["Is broken: delete it", "Is still valid"]
answer = 0
why = "The move it measured has been fully undone."

[explain_back]
prompt = "How do you draw Fibonacci correctly, and when do you redraw or delete it?"
must_include = ["you draw it on the last clean impulse leg that broke structure, on the trend timeframe", "in an uptrend from the swing low to the swing high (in a downtrend high to low), anchored on the wicks; drawn backwards the levels are wrong", "a new high before the pullback means redrawing, and price below the leg's low breaks the leg so the grid is deleted"]
misconceptions = ["the direction you draw it doesn't matter", "Fibonacci levels predict exactly where price will turn"]

[[explain_back.fixtures]]
answer = "I pick the last strong leg in the trend's direction that broke structure on H4. In an uptrend I drag from the swing low to the swing high, wick to wick; doing it backwards would flip the levels. If price makes a new high before pulling back I redraw to that high, and if it falls below the leg's low, the leg is broken and I delete the grid."
expect = "got_it"

[[explain_back.fixtures]]
answer = "I draw it on the last impulse leg that broke structure, from the swing low to the swing high in an uptrend, using the wicks."
expect = "almost"

[[explain_back.fixtures]]
answer = "You can drag it either way, and price always turns at 61.8%, so I just buy there."
expect = "almost_or_not_yet"
+++
