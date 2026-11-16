+++
id = 41
stage = 9
title = "Reviewing your journal"
objective = "read your own journal for leaks: group your trades, compare their results in R, find the costliest habit, and change one rule at a time."
hook = "Fifty Demo Trades are in your journal. Somewhere in them is the habit costing you the most. How do you find it?"
analogy = "Think of a household budget. Looking at the total spent tells you little; sorting the receipts into groceries, transport and takeaways shows where the money actually goes. One category is usually the leak, and it's rarely the one you guessed."
analogy_tag = "sorting the receipts"
example = "60 Demo Trades, averaging +0.15R. Sorted: London-morning trades, 32 of them, average +0.4R. Trades after noon New York, 14, average −0.3R. The 7 tagged FOMO average −0.8R, and 5 of them broke a rule. The leak isn't the setup; it's late-day trades and chasing. One change: a new plan version that trades only until 12:00 New York. Then 20 more trades before judging it."
mistake = "Changing three things after one bad week. With several changes at once you can't tell which one helped, and a week is too few trades to show anything but noise."
takeaways = [
  "Group your trades: by session, by Scenario type, by Emotion Tag, by rule broken.",
  "Compare each group's average result in R; a leak is a group with a clearly negative average or most of the rule breaks.",
  "Small groups mislead: wait for about 20 trades in a group before concluding.",
  "Change one rule at a time, as a new plan version, and judge it over the next batch of trades.",
]

[[steps]]
heading = "From diary to data"
text = "A journal entry is a diary; fifty of them are **data**. Once you have them, read back through your entries and **group** them: by session, by kind of Scenario (pullback, bounce), by **Emotion Tag**, by which rule was broken. Write each group's count and results in a simple table."
why = "CME Group's trade-plan course calls the trade log a tool for seeing the bigger picture: in one snapshot you can see your history and spot your successes and mistakes."

[[steps]]
heading = "Average R per group"
text = "For each group, add up the results in **R** and divide by the number of trades: its **average R**. Your whole journal might average +0.15R, but one group could be +0.4R and another −0.3R. The groups below zero are your **leaks**."
why = "Measuring in R, not dollars, keeps groups comparable even if your balance or risk changed over time (Lesson 34)."

[[steps]]
heading = "Look at feelings and rule breaks"
text = "Two groups deserve special attention: trades tagged **FOMO, revenge or overconfident**, and trades that **broke the plan**. They're often a small share of trades and a large share of losses. If so, your edge is fine; your discipline leaks."
why = "This is Lesson 36 in your own numbers: the habits that turn good trades into bad ones usually show up clearly once they're counted."

[[steps]]
heading = "Small samples lie"
text = "Five trades in a group can look brilliant or terrible by chance. Treat a group as **evidence** only once it has around **20 trades**; before that, it's a hint to watch. The same goes for a single good or bad week."
why = "With a positive expectancy you can still lose six in a row (Lesson 34). Small groups mostly show luck."

[[steps]]
heading = "One change, then measure"
text = "Pick the **one** biggest leak and turn the fix into a rule: save it as a **new plan version**, and write why in your Weekly Review. Then trade the next 20 or so trades before judging it. Fix one leak, then look for the next."
why = "Changing several rules at once means you can't tell which change helped. Versions keep a record of what changed and when (Lesson 39)."

[[videos]]
id = "WjWC5Qlpqns"
title = "The EXACT thought process, entries, exits, and results of my recent trades..."
channel = "Rayner Teo"
duration = "14:23"
watch_for = "How a trader looks back at a batch of trades: the reasoning, the entry and exit, and the result, side by side. His trades and markets are his own."
fallback_search = "review my recent trades thought process entries exits"
checked = "2026-11-16"

[[sources]]
claim = "A trade log gives an overview of your trading history so you can identify successes and mistakes; record why you made each trade and, after each day, focus on the why and how of your performance more than the profit or loss."
name = "CME Group Education: Keep a Trade Log"
url = "https://www.cmegroup.com/education/courses/building-a-trade-plan/keep-a-trade-log.html"
checked = "2026-11-16"

[[quiz]]
id = "q1"
q = "Your journal averages +0.15R, but trades after noon New York average −0.3R over 14 trades. What is that?"
options = ["A likely leak: a group with a negative average", "Proof the whole plan is broken", "Nothing worth noting", "A reason to trade more after noon"]
answer = 0
why = "A group below zero is a leak to examine."

[[quiz]]
id = "q2"
q = "Why compare groups in R rather than in dollars?"
options = ["R stays comparable even if your balance or risk changed", "Dollars are illegal in journals", "R always looks better", "There's no difference"]
answer = 0
why = "A −1R loss means the same thing at any size."

[[quiz]]
id = "q3"
q = "Four trades on Fridays all lost. What do you conclude?"
options = ["Not much yet: four trades is too few; keep watching", "Never trade Fridays again", "Fridays are cursed", "Double size on Fridays to win it back"]
answer = 0
why = "Wait for about 20 trades in a group before concluding."

[[quiz]]
id = "q4"
q = "You find two leaks. What's the plan?"
options = ["Fix the bigger one first with one new rule, measure, then the next", "Change both rules and the setup at once", "Ignore them", "Stop journaling"]
answer = 0
why = "One change at a time, or you can't tell what worked."

[[quiz]]
id = "q5"
q = "The 7 trades tagged FOMO average −0.8R, and 5 broke a rule. What does it suggest?"
options = ["The leak is discipline, not the setup", "The setup doesn't work", "FOMO trades should be bigger", "Emotion Tags don't matter"]
answer = 0
why = "A small share of trades, a large share of the losses."

[[recall]]
id = "r1"
q = "A leak is…"
options = ["A group of trades with a clearly negative average R", "Any losing trade", "A slow internet connection"]
answer = 0
why = "Losing trades are normal; losing groups are leaks."

[[recall]]
id = "r2"
q = "Before concluding about a group, wait for about…"
options = ["20 trades", "2 trades", "1 week"]
answer = 0
why = "Small samples mostly show luck."

[[recall]]
id = "r3"
q = "When you fix a leak, you…"
options = ["Save one new rule as a new plan version and measure it", "Change everything at once"]
answer = 0
why = "One change, then measure."

[explain_back]
prompt = "How do you find a leak in your journal, and what do you do about it?"
must_include = ["group your trades by session, Scenario type, Emotion Tag and rule broken, and compare each group's average result in R", "a leak is a group with a clearly negative average or most of the rule breaks, but small groups mislead, so wait for about 20 trades before concluding", "change one rule at a time as a new plan version and judge it over the next batch of trades"]
misconceptions = ["one bad week proves a rule is wrong", "changing several things at once fixes leaks faster"]

[[explain_back.fixtures]]
answer = "I sort my trades into groups, like session, kind of setup, Emotion Tag and which rule was broken, and work out each group's average R. A group that's clearly below zero, or holds most of my rule breaks, is a leak, as long as it has around 20 trades; fewer than that could just be luck. Then I fix only the biggest leak with one new rule, save it as a new plan version, and give it the next 20 or so trades before judging it."
expect = "got_it"

[[explain_back.fixtures]]
answer = "I group my trades by session and by Emotion Tag, work out the average R for each, and the ones below zero are my leaks."
expect = "almost"

[[explain_back.fixtures]]
answer = "After a bad week I change my setup, my sessions and my risk all at once so the next week is better."
expect = "almost_or_not_yet"
+++
