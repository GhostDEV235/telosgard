+++
id = 35
stage = 7
title = "Daily Loss Limit & drawdown"
objective = "set a Daily Loss Limit of 2 to 3R and stop when it's reached, and explain why losses are harder to recover than they look."
hook = "Two stops hit before lunch. The third setup looks perfect. Do you take it?"
analogy = "Think of a casino visit with a fixed amount in your pocket and the card left at home. The limit isn't a prediction that you'll lose; it's a decision made while calm, so a bad hour can't turn into a bad month."
analogy_tag = "leaving the card at home"
example = "Your plan risks 1% a trade, with a Daily Loss Limit of 2R. On Tuesday the first trade is stopped out (−1R), then the second (−2R in total). The limit is reached: you close the platform, write the journal and stop. The next day starts fresh. Without a limit, a 'make it back' streak of five more losses would have cost about 7% in one day."
mistake = "Treating the limit as a suggestion when the next setup looks good. The setups after two losses aren't the problem; your judgement after two losses is. The limit exists for exactly the day you want to break it."
takeaways = [
  "A Daily Loss Limit is a Hard Rule in your Trading Plan: once the day's losses reach it, no more entries that day.",
  "This course sets it at 2R, and never more than 3R.",
  "Losses are harder to recover than they look: −10% needs +11%, −20% needs +25%, −50% needs +100%.",
  "Small risk per trade plus a daily limit keeps drawdowns shallow, where recovery is realistic.",
]

[[steps]]
heading = "A limit decided while calm"
text = "A **Daily Loss Limit** is the most you may lose in one day, counted in R. Once the day's closed losses reach it, any further entry **breaks your Trading Plan**. You set it before the session, in the Pre-Session Checklist, not in the middle of a bad morning."
why = "In this app it's a Hard Rule: the journal checks it by calculation, so an entry after the limit is marked as not following the plan, whatever it made."

[[steps]]
heading = "2R, never more than 3R"
text = "This course's limit is **2R**: two full losses and you're done for the day. You may raise it to **3R** at most. At 1% per trade that caps a bad day at 2–3% of the account, a dent you can recover from."
why = "CME Group points out that an active trader can see five losses in a row in a single day. At 1%, a 2R limit stops that day at 2% instead of 5%."

[[steps]]
heading = "The recovery maths"
text = "A loss is measured against what you had; a recovery against what's left. Lose **10%** and you need **+11%** to get back. Lose **20%**: **+25%**. Lose **50%**: **+100%**, doubling what's left. The deeper the hole, the steeper the climb."
why = "$10,000 down 50% is $5,000; getting back to $10,000 means making $5,000 on $5,000: +100%. CME Group's table continues: −70% needs +233% and −90% needs +900%."

[[steps]]
heading = "Drawdown and your Readiness Run"
text = "**Drawdown** is how far the account sits below its last high. Your Trading Plan also sets a **maximum total loss** for each Readiness Run on demo. Breach it and the run ends; a new one starts with a fresh balance. Small risk and a daily limit keep you well away from it."
why = "Profit isn't required for Live Readiness, but staying inside the maximum total loss is. The habit you're proving is that your losses stay small."

[[steps]]
heading = "What to do at the limit"
text = "When you hit it: **close the platform**, log the trades and write one line on how you felt. Don't look for 'one more'. Tomorrow starts at zero, with a fresh checklist. Lesson 36 looks at the urge to win it back."
why = "Stopping is a skill you practise, like placing a stop. Each day you respect the limit is a day of Plan Adherence, even if it lost money."

[[videos]]
id = "mPzxtXtwFzM"
title = "How To Overcome Trading Losses And Losing Streak"
channel = "Rayner Teo"
duration = "6:01"
watch_for = "Why losing streaks are normal for a working plan, and how small risk lets you ride them out."
fallback_search = "how to handle a losing streak trading"
checked = "2026-11-16"

[[videos]]
id = "wjOcqjeO7S4"
title = "What Your Daily Loss Should Be DAY TRADING!"
channel = "Patrick Wieland"
duration = "8:58"
watch_for = "Setting the daily stop before the session and walking away when it's hit. His dollar figures are his own; use your limit in R."
fallback_search = "daily loss limit trading"
checked = "2026-11-16"

[[sources]]
claim = "A five-in-a-row losing streak is something an active trader could experience in a single day; after a loss, the gain needed to recover is larger than the loss: 10% needs 11.11%, 20% needs 25%, 50% needs 100%, 70% needs 233% and 90% needs 900%."
name = "CME Group Education: Controlling Risk"
url = "https://www.cmegroup.com/education/courses/trade-and-risk-management/controlling-risk.html"
checked = "2026-11-16"

[[sources]]
claim = "Having strict limits on position size can help limit the losses most likely to occur when you are beginning to trade; new traders are best off risking a modest 1%–3% of the account on a single trade."
name = "CME Group Education: Proper Position Size"
url = "https://www.cmegroup.com/education/courses/trade-and-risk-management/proper-position-size.html"
checked = "2026-11-16"

[[quiz]]
id = "q1"
q = "Your Daily Loss Limit is 2R and you've lost 2R by 11 am. A great setup appears. What do you do?"
options = ["Stop for the day: the limit is reached", "Take it with half size", "Take it with double size to win it back", "Take it if it looks very good"]
answer = 0
why = "Any entry after the limit breaks the plan, however good the setup looks."

[[quiz]]
id = "q2"
q = "An account falls 50%. What gain does it need to get back to where it started?"
options = ["+100%", "+50%", "+75%", "+150%"]
answer = 0
why = "Half the money has to double: $5,000 → $10,000."

[[quiz]]
id = "q3"
q = "An account falls 20%. What gain gets it back?"
options = ["+25%", "+20%", "+40%", "+10%"]
answer = 0
why = "$8,000 needs $2,000 more: 2,000 ÷ 8,000 = 25%."

[[quiz]]
id = "q4"
q = "At 1% per trade, what does a 2R Daily Loss Limit cap a bad day at?"
options = ["About 2% of the account", "About 20%", "About 0.2%", "Nothing: it's only a guide"]
answer = 0
why = "Two losses of 1R, each about 1%."

[[quiz]]
id = "q5"
q = "When should you set the day's loss limit?"
options = ["Before the session, in the Pre-Session Checklist", "After the first loss", "At the end of the day", "Only on bad days"]
answer = 0
why = "A limit decided calmly is the one you'll keep."

[[quiz]]
id = "q6"
q = "What is drawdown?"
options = ["How far the account sits below its last high", "The spread", "Your biggest single win", "The distance to your target"]
answer = 0
why = "It measures the hole you're in, from the peak."

[[recall]]
id = "r1"
q = "This course's Daily Loss Limit is…"
options = ["2R, never more than 3R", "10R", "There isn't one"]
answer = 0
why = "Two or three losses and the day is over."

[[recall]]
id = "r2"
q = "A 50% loss needs…"
options = ["+100% to recover", "+50% to recover"]
answer = 0
why = "The recovery is measured on what's left."

[[recall]]
id = "r3"
q = "When the limit is reached, you…"
options = ["Close the platform, journal and stop", "Look for one more setup", "Raise the limit"]
answer = 0
why = "Tomorrow starts fresh."

[explain_back]
prompt = "What is a Daily Loss Limit, and why do losses matter more than they seem?"
must_include = ["a Daily Loss Limit is the most you may lose in a day in R, set before the session at 2R and never more than 3R, and once it's reached you stop trading that day", "losses are harder to recover than they look because the recovery is measured on what's left: 20% needs 25% and 50% needs 100%", "small risk per trade and a daily limit keep drawdowns shallow, inside the plan's maximum total loss"]
misconceptions = ["you can trade on after the limit if the setup looks good", "a 50% loss needs a 50% gain to recover"]

[[explain_back.fixtures]]
answer = "It's the most I'm allowed to lose in a day, counted in R and set in my checklist before I start: 2R, 3R at the very most. Once my losses reach it I close the platform and stop, however good the next setup looks. Losses hurt more than they seem because you recover on a smaller balance: down 20% needs +25%, and down 50% needs +100%. Keeping risk at 1% and a daily limit keeps the drawdown shallow, well inside my plan's maximum total loss."
expect = "got_it"

[[explain_back.fixtures]]
answer = "It's a limit of 2R a day set before the session, and when I reach it I stop trading for the day."
expect = "almost"

[[explain_back.fixtures]]
answer = "If I lose 50% I just need to make 50% back, so a bad day isn't a big deal. I keep going until I win it back."
expect = "almost_or_not_yet"
+++
