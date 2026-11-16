+++
id = 39
stage = 8
title = "Writing your Trading Plan"
objective = "write your Trading Plan in the builder: Hard Rules checked by calculation, Soft Rules judged from your journal, and new versions only between trades."
hook = "You know the setup, the stops, the sizing and the limits. Where do they live so that every trade can be checked against them?"
analogy = "Think of the rules of a board game written on the inside of the lid. Nobody argues about them mid-game, because they were agreed before the first move. If the rules need changing, you change them between games, not during a turn."
analogy_tag = "rules on the box lid"
example = "Plan v1. Hard Rules: max risk 1% per trade; max total loss 10% of the Readiness Run's starting balance; minimum reward-to-risk 2:1; Daily Loss Limit 2R; at most 2 trades a day; London and New York sessions. Always on: a stop on every trade, No-Trade Windows, a saved Pre-Session Checklist. Soft Rules: the Classic Price Action setup, and each trade is one of the day's Scenarios."
mistake = "Editing the plan to fit a trade you've already taken, or writing rules so loose they can't be broken. A plan you can always satisfy measures nothing."
takeaways = [
  "The Trading Plan lives in the Tools tab; every Demo Trade is judged against the version in force when you took it.",
  "Hard Rules are numbers checked by calculation: max risk per trade, max total loss, minimum reward-to-risk, Daily Loss Limit, trades per day, sessions.",
  "A stop-loss on every trade, No-Trade Windows and a Pre-Session Checklist are always on.",
  "Soft Rules need judgement: your setup, and that each trade was one of the day's Scenarios. Professor Aurel judges them from your journal entry.",
]
drills = ["broken_rule"]

[[steps]]
heading = "One plan, many versions"
text = "Your **Trading Plan** is in the **Tools** tab. Saving it creates **version 1**; every later change saves a new version, and old ones stay readable. Each Demo Trade is judged against the version **in force when you took it**, so changing the rules never rewrites your history."
why = "CME Group's course on building a trade plan starts from the same idea: define your approach and your risk rules up front, honestly, before you trade."

[[steps]]
heading = "Hard Rules: the numbers"
text = "Set the course's values: **1%** max risk per trade, **2:1** minimum reward-to-risk, a **2R** Daily Loss Limit, **2** trades a day at most, **London and New York** sessions, and a **max total loss** of about 10% of the run's starting balance. Each is checked by calculation."
why = "CME Group lists maximum trade loss and maximum day loss among the risk parameters a plan should quantify. The max total loss ends a Readiness Run if it's breached (Lesson 35)."

[[steps]]
heading = "Always on"
text = "Three rules can't be switched off: a **stop-loss on every trade**, the **No-Trade Windows** (rollover, the Sunday open, 5 minutes around tier-1 news), and a saved **Pre-Session Checklist** on any day you trade."
why = "They're the habits this course treats as non-negotiable for a beginner; Lessons 10, 15 and 38 explain why."

[[steps]]
heading = "Soft Rules: the judgement"
text = "Some rules need judgement: **your setup**, pre-filled with the Classic Price Action wording, and that **each trade is one of the day's Scenarios**. Add your own, one per line. Professor Aurel judges them from what you write in each journal entry; you can dispute a verdict."
why = "That's why the journal asks you to describe the trend, the level and the confirmation candle in a sentence: a vague reason can't be judged as following the setup."

[[steps]]
heading = "Change it between trades"
text = "If a rule is wrong, change it **after a review**, not in the middle of a trade or to excuse one. Write why in your Weekly Review, save a new version, and judge it over the next batch of trades, not the next one."
why = "A plan that changes whenever it's inconvenient stops being a plan. Versions keep each change visible."

[[videos]]
id = "G13CVaAFOGI"
title = "The Importance of a Trading Plan + A Free Trading Plan Template!"
channel = "TraderNick"
duration = "5:48"
watch_for = "What a written plan covers and why writing it down beats keeping it in your head. Your plan lives in the app instead of his template."
fallback_search = "why you need a trading plan template"
checked = "2026-11-16"

[[videos]]
id = "WSBsg7V8XXA"
title = "How to Create a Winning Trading Plan: A Step-by-Step Guide"
channel = "Chart Champions"
duration = "5:26"
watch_for = "Turning entry, exit and risk ideas into written rules. \"Winning\" is the video's word; a plan guarantees discipline, not profit."
fallback_search = "how to create a trading plan step by step"
checked = "2026-11-16"

[[sources]]
claim = "A trade plan should define your risk management approach and rules at the outset; consider your leverage, your maximum trade loss and your maximum day loss, and what share of your account you're prepared to risk on each trade."
name = "CME Group Education: Risk Management and Your Trade Plan"
url = "https://www.cmegroup.com/education/courses/building-a-trade-plan/risk-management-and-your-trade-plan.html"
checked = "2026-11-16"

[[sources]]
claim = "It is extremely important to establish clear entry and exit criteria and stick to those rules, because it is far too easy to base a decision on an emotional response rather than a strategy."
name = "CME Group Education: Trading Strategies in Your Trade Plan"
url = "https://www.cmegroup.com/education/courses/building-a-trade-plan/trading-strategies-in-your-trade-plan.html"
checked = "2026-11-16"

[[quiz]]
id = "q1"
q = "You change your plan on Thursday. What happens to Monday's trades?"
options = ["They're still judged against the version in force on Monday", "They're re-judged with the new rules", "They're deleted", "They no longer count"]
answer = 0
why = "Each trade is judged against the version in force when it was taken."

[[quiz]]
id = "q2"
q = "Which of these is a Hard Rule?"
options = ["Daily Loss Limit of 2R", "Entered on a confirmation candle at a key zone", "Felt calm", "The trade was one of today's Scenarios"]
answer = 0
why = "Hard Rules are numbers checked by calculation."

[[quiz]]
id = "q3"
q = "Which rule can you switch off?"
options = ["None of these: stop-loss, No-Trade Windows and the checklist are always on", "The stop-loss", "The No-Trade Windows", "The checklist"]
answer = 0
why = "Those three are always on."

[[quiz]]
id = "q4"
q = "Who judges the Soft Rules?"
options = ["Professor Aurel, from what you write in the journal entry", "Your broker", "Nobody", "The calculator"]
answer = 0
why = "Soft Rules need judgement; you can dispute a verdict."

[[quiz]]
id = "q5"
q = "A trade broke your 2-trades-a-day rule but made money. Should you raise the limit to 3 tonight so it fits?"
options = ["No: change rules after a review, never to excuse a trade", "Yes, the trade worked", "Yes, and re-date the trade", "Delete the trade"]
answer = 0
why = "A plan bent to fit trades measures nothing."

[[quiz]]
id = "q6"
q = "What reward-to-risk minimum does this course put in the plan?"
options = ["2:1", "1:1", "1:2", "No minimum"]
answer = 0
why = "At least twice the risk, from Lessons 27 and 34."

[[recall]]
id = "r1"
q = "Hard Rules are…"
options = ["Numbers checked by calculation", "Feelings about the market", "Your broker's rules"]
answer = 0
why = "Risk %, total loss, reward-to-risk, daily limit, trades per day, sessions."

[[recall]]
id = "r2"
q = "A new plan version applies to…"
options = ["Trades taken after you save it", "Every past trade too"]
answer = 0
why = "History isn't rewritten."

[[recall]]
id = "r3"
q = "Soft Rules include…"
options = ["Your setup and that each trade is one of the day's Scenarios", "Your lot size"]
answer = 0
why = "They need judgement, so Professor Aurel reads your entry."

[explain_back]
prompt = "What's in your Trading Plan, and how is a trade checked against it?"
must_include = ["Hard Rules are numbers checked by calculation: max risk per trade, max total loss, minimum reward-to-risk, Daily Loss Limit, trades per day and sessions", "a stop-loss on every trade, the No-Trade Windows and a saved Pre-Session Checklist are always on, and Soft Rules like your setup and matching a Scenario are judged by Professor Aurel from the journal entry", "the plan is versioned: each trade is judged against the version in force when it was taken, and you change rules between trades after a review, never to excuse a trade"]
misconceptions = ["you can update the plan to fit a trade you already took", "Soft Rules are optional suggestions"]

[[explain_back.fixtures]]
answer = "The Hard Rules are numbers the app checks by calculation: 1% max risk, a max total loss for the run, at least 2:1, a 2R Daily Loss Limit, 2 trades a day and my sessions. A stop on every trade, the No-Trade Windows and the checklist are always on. The Soft Rules are my setup and that each trade is one of the day's Scenarios, and Professor Aurel judges those from what I write. Each trade is checked against the plan version in force when I took it, and I only change rules between trades after a review."
expect = "got_it"

[[explain_back.fixtures]]
answer = "It has Hard Rules like 1% risk, 2:1 and a 2R daily limit that are checked by calculation, and Soft Rules like my setup that Professor Aurel judges."
expect = "almost"

[[explain_back.fixtures]]
answer = "If a trade breaks a rule but works, I just change the plan afterwards so it fits. The Soft Rules are only suggestions."
expect = "almost_or_not_yet"
+++
