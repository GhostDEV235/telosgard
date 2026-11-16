+++
id = 34
stage = 7
title = "Risk-to-reward & expectancy"
objective = "work out the win rate a reward-to-risk ratio needs to break even, measure a plan by its expectancy in R, and see why small fixed risk survives losing streaks."
hook = "A strategy wins only 4 trades in 10. Is it a bad strategy?"
analogy = "Think of a goalkeeper in a penalty shoot-out. Most penalties go in, yet a keeper who saves one in three can still win the shoot-out for the team, because each save counts for more than each goal conceded. The score, not the save rate, decides."
analogy_tag = "penalty shoot-out"
example = "You risk 1R on every trade and aim for 2R. Over 100 trades you win 40 and lose 60: 40 × 2R = +80R, 60 × 1R = −60R. Net +20R, or +0.2R per trade. At $100 per R that's +$2,000, while being wrong 60% of the time. (Illustrative numbers; costs come off the top.)"
mistake = "Judging a strategy by its win rate alone, or chasing a high win rate with tiny targets. 70% wins at 1:0.3 loses money: 70 × 0.3R = 21R won, 30 × 1R = 30R lost."
takeaways = [
  "Reward-to-risk compares the target distance with the stop distance; this course needs at least 2:1.",
  "Break-even win rate = 1 ÷ (1 + reward-to-risk): 50% at 1:1, about 33% at 2:1, 25% at 3:1.",
  "Expectancy is the average result per trade in R: win rate × average win − loss rate × average loss.",
  "At 1% per trade, ten losses in a row leave about 90% of the account; at 5% they leave about 60%.",
]

[[steps]]
heading = "Reward to risk"
text = "**Reward-to-risk** compares how far the target is with how far the stop is. Stop $10 away, target $20 away: **2:1**, or 2R for 1R. This course's minimum is 2:1 (Lesson 27). The ratio is fixed before you enter, from the chart."
why = "R is the money you lose if the stop is hit (Lesson 2). Talking in R lets you compare trades of different sizes: a $96 loss and a $50 loss are both −1R if each was the planned risk."

[[steps]]
heading = "The break-even win rate"
text = "Every ratio has a **break-even win rate**: 1 ÷ (1 + reward-to-risk). At **1:1** you must win more than **50%**; at **2:1**, more than about **33%**; at **3:1**, more than **25%**. Above it you make money over many trades; below it you lose."
why = "At 2:1, one win (+2R) pays for two losses (−2R). So winning one trade in three is exactly break-even. In practice spread and commission (Lesson 11) push that line up a little."
widget = "win_rate"

[[steps]]
heading = "Expectancy: the plan's average"
text = "**Expectancy** is what one trade is worth on average, in R: **win rate × average win − loss rate × average loss**. 40% wins at 2R: 0.4 × 2 − 0.6 × 1 = **+0.2R** a trade. Positive expectancy is the goal; it only shows over dozens of trades."
why = "Ten trades prove almost nothing: a positive plan can lose six in a row. That's why Live Readiness asks for 50 Demo Trades before any real money."

[[steps]]
heading = "Small risk survives streaks"
text = "Losing streaks happen even to good plans. At **1%** per trade, ten losses in a row leave about **90%** of the account. At **5%**, the same streak leaves about **60%**. Small, fixed risk is what lets a positive expectancy have time to work."
why = "0.99 multiplied by itself ten times is about 0.904; 0.95 ten times is about 0.599. CME Group's tables show the same effect: a fixed percentage shrinks each loss as the account shrinks."

[[videos]]
id = "fuFlkAnvSXI"
title = "The Ugly Truth About Risk To Reward Ratio (95% Of Traders Get It Wrong)"
channel = "Rayner Teo"
duration = "6:23"
watch_for = "Why a ratio means nothing without the win rate that goes with it. A big ratio with a tiny win rate can still lose."
fallback_search = "risk to reward ratio win rate explained"
checked = "2026-11-16"

[[videos]]
id = "cbhXErBHuNM"
title = "Why Win Rate Alone Tells You Little: Risk-to-Reward Explained"
channel = "QuantCrawler"
duration = "9:37"
watch_for = "The break-even win rate for each ratio, and how expectancy combines the two."
fallback_search = "break even win rate risk reward"
checked = "2026-11-16"

[[sources]]
claim = "Trading results depend on how much you make when right and how little you lose when wrong, not on being right; fear of losses leads traders to sell winners too early and hold losers too long."
name = "CME Group Education: Misconceptions of Taking Losses"
url = "https://www.cmegroup.com/education/courses/trade-and-risk-management/misconceptions-of-taking-losses.html"
checked = "2026-11-16"

[[sources]]
claim = "Over a five-trade losing streak a fixed 1% rule takes a $50,000 account to $47,550 and a 2% rule to $45,198: a fixed percentage reduces each succeeding trade's size and slows the account's decay."
name = "CME Group Education: Controlling Risk"
url = "https://www.cmegroup.com/education/courses/trade-and-risk-management/controlling-risk.html"
checked = "2026-11-16"

[[quiz]]
id = "q1"
q = "Stop $8 away, target $24 away. What is the reward-to-risk?"
options = ["3:1", "1:3", "2:1", "1:1"]
answer = 0
why = "$24 ÷ $8 = 3: three times the risk."

[[quiz]]
id = "q2"
q = "At 2:1, roughly what win rate breaks even (before costs)?"
options = ["About 33%", "50%", "66%", "10%"]
answer = 0
why = "1 ÷ (1 + 2) = 1/3: one +2R win pays for two −1R losses."

[[quiz]]
id = "q3"
q = "You win 40% of trades at 2R and lose 60% at 1R. What's the expectancy?"
options = ["+0.2R per trade", "−0.2R per trade", "+0.8R per trade", "Zero"]
answer = 0
why = "0.4 × 2 − 0.6 × 1 = 0.8 − 0.6 = +0.2R."

[[quiz]]
id = "q4"
q = "A plan wins 70% but takes 0.3R on wins and loses 1R on losses. Over 100 trades?"
options = ["−9R: it loses money despite the win rate", "+70R", "+21R", "Break-even"]
answer = 0
why = "70 × 0.3R = 21R won; 30 × 1R = 30R lost; net −9R."

[[quiz]]
id = "q5"
q = "Ten losses in a row at 1% each leave roughly…"
options = ["90% of the account", "50%", "99%", "0%"]
answer = 0
why = "0.99¹⁰ ≈ 0.904, about 90%."

[[quiz]]
id = "q6"
q = "Your first 8 trades of a positive plan: 6 losses. What does it tell you?"
options = ["Very little yet: expectancy shows over dozens of trades", "The plan is broken; change it today", "Double the risk to win it back", "Stop using stops"]
answer = 0
why = "Short streaks are normal; judge a plan on a large sample, as long as you followed it."

[[recall]]
id = "r1"
q = "The break-even win rate at 1:1 is…"
options = ["50%", "33%", "25%"]
answer = 0
why = "1 ÷ (1 + 1)."

[[recall]]
id = "r2"
q = "Expectancy is…"
options = ["The average result per trade, in R", "Your win rate", "Your biggest win"]
answer = 0
why = "Win rate × average win − loss rate × average loss."

[[recall]]
id = "r3"
q = "Small fixed risk per trade matters because…"
options = ["It survives losing streaks so the edge has time to work", "It guarantees wins"]
answer = 0
why = "Ten losses at 1% leave about 90%."

[explain_back]
prompt = "Why can a strategy that's wrong most of the time still make money, and what keeps you in the game long enough to find out?"
must_include = ["reward-to-risk sets a break-even win rate of 1 divided by 1 plus the ratio, about 33% at 2:1, so winning less than half can still be profitable", "expectancy, win rate times average win minus loss rate times average loss in R, tells you whether a plan makes money, and it only shows over many trades", "risking a small fixed amount like 1% means a losing streak of ten leaves about 90% of the account"]
misconceptions = ["a high win rate means a profitable strategy", "a few losses in a row prove a plan doesn't work"]

[[explain_back.fixtures]]
answer = "Because what matters is wins times their size against losses times theirs. At 2:1 one win pays for two losses, so the break-even win rate is about 33%, which is 1 ÷ (1 + 2). Expectancy puts it together: win rate times average win minus loss rate times average loss, in R; 40% at 2R is +0.2R a trade, but you only see it over many trades. Risking just 1% a trade keeps me alive through streaks: ten losses in a row still leave about 90%."
expect = "got_it"

[[explain_back.fixtures]]
answer = "At 2:1 a win pays for two losses, so I only need to win about a third of trades to break even, and expectancy in R shows whether the plan makes money over time."
expect = "almost"

[[explain_back.fixtures]]
answer = "A good strategy wins most of its trades. If I lose three in a row, it's broken and I should switch."
expect = "almost_or_not_yet"
+++
