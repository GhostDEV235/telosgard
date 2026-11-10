+++
id = 2
stage = 1
title = "The honest maths of trading"
objective = "explain why most beginners lose, and why a trader can be wrong often and still make money."
hook = "If you win more trades than you lose, are you guaranteed to make money?"
analogy = "Think of a market stall that sells most of its items at a small loss to bring people in, but a few at a big profit. Count the sales one by one and it looks like it's failing; add up the week and it's doing fine. A trader is judged the same way: on the totals, not on each trade."
analogy_tag = "market stall"
example = "Ten trades, each risking $100. Five hit their stop: −$500. Five reach a target twice as far away: +$1,000. Right only half the time, and still up $500. Now flip it: win 6 of 10 but take $50 on each win and $150 on each loss, and you're down $300."
mistake = "Judging yourself by how often you're right. A high win rate with big losses still loses money, and chasing “being right” leads to holding losers and cutting winners short."
takeaways = [
  "Most retail CFD accounts lose money; regulators make brokers say so.",
  "Results depend on the size of wins vs losses, not just how often you win.",
  "Being right half the time can still make money if wins are bigger than losses.",
  "Small, planned losses and a repeatable process beat being confident.",
]

[[steps]]
heading = "Most beginners lose money"
text = "In the UK and EU, a CFD broker must warn you what share of its retail accounts lose money. The EU regulator found it's typically **74–89%**. That isn't bad luck; it's what happens when people trade without a plan for their losses."
why = "Each broker's standard risk warning has to state its own percentage. The 74–89% range comes from national regulators' analyses across EU countries. It's why this course makes you prove your discipline on a demo account before it teaches you anything about real money."

[[steps]]
heading = "Being right isn't the goal"
text = "What matters is how much you make when you're right versus how much you lose when you're wrong. Win 6 trades out of 10 but make $50 on each win and lose $150 on each loss, and you're **down $300**."
why = "6 wins × $50 = $300 made. 4 losses × $150 = $600 lost. Net: −$300, despite a 60% win rate."

[[steps]]
heading = "Small losses, bigger wins"
text = "Risk $100 on each trade. Lose 5 of 10 at $100 each (−$500) and win 5 at $200 each (+$1,000): **+$500**, despite being right only half the time. Keeping losses small and letting wins be bigger is the whole game."
widget = "win_rate"
why = "Traders call the planned loss **1R** (one unit of risk). A win twice that size is +2R. In R, the result above is 5 × (+2R) + 5 × (−1R) = +5R over ten trades. You'll meet R properly in the risk lessons."

[[steps]]
heading = "Process beats conviction"
text = "A trader isn't someone who feels sure. It's someone who follows the same rules every time: a written plan, a stop-loss on every trade, a size chosen before entry. You'll build that process in this course, one step at a time."
why = "Confidence feels like skill, but the market doesn't care how sure you are. A repeatable process can be measured, reviewed and improved; a feeling can't."

[[videos]]
id = "aKZsireNBIM"
title = "The Risk to Reward Ratio Explained in One Minute: From Definition and \"Formula\" to Examples"
channel = "One Minute Economics"
duration = "1:36"
watch_for = "How comparing what you could lose with what you could gain decides whether a trade is worth taking."
fallback_search = "risk to reward ratio explained"
checked = "2026-11-10"

[[videos]]
id = "tNNqFSLXAOk"
title = "Win Rate, Risk To Reward Ratio, Expected Value (Trading Math)"
channel = "Mind Math Money"
duration = "11:14"
watch_for = "How win rate and the size of wins vs losses combine into one expected result per trade."
fallback_search = "win rate risk reward expected value trading"
checked = "2026-11-10"

[[sources]]
claim = "Across EU jurisdictions, 74–89% of retail CFD accounts typically lose money; providers must show a standardised risk warning with their own percentage of losing retail accounts, and give negative balance protection per account."
name = "ESMA: ESMA agrees to prohibit binary options and restrict CFDs to protect retail investors"
url = "https://www.esma.europa.eu/press-news/esma-news/esma-agrees-prohibit-binary-options-and-restrict-cfds-protect-retail-investors"
checked = "2026-11-10"

[[sources]]
claim = "UK CFD firms must provide a standardised risk warning including the percentage of their retail client accounts that make losses, and guarantee a client can't lose more than the funds in their CFD account."
name = "FCA: FCA confirms permanent restrictions on the sale of CFDs to retail consumers"
url = "https://www.fca.org.uk/news/press-releases/fca-confirms-permanent-restrictions-sale-cfds-and-cfd-options-retail-consumers"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "According to the EU regulator, what share of retail CFD accounts typically lose money?"
options = ["About 10–20%", "About 40–50%", "About 74–89%", "Almost none"]
answer = 2
why = "Regulators found 74–89% of retail accounts typically lose money, which is why brokers must warn you."

[[quiz]]
id = "q2"
q = "You win 5 of 10 trades. Each win is +$200 and each loss is −$100. What's the total?"
options = ["−$500", "$0", "+$500", "+$1,500"]
answer = 2
why = "5 × $200 = $1,000 won; 5 × $100 = $500 lost; net +$500."

[[quiz]]
id = "q3"
q = "You win 7 of 10 trades, making $30 on each win, but lose $100 on each loss. What's the total?"
options = ["+$210", "−$90", "+$90", "−$300"]
answer = 1
why = "7 × $30 = $210 won; 3 × $100 = $300 lost; net −$90. A 70% win rate still lost money."

[[quiz]]
id = "q4"
q = "What decides whether a trader makes money over many trades?"
options = ["How often they're right, nothing else", "How confident they feel", "Win rate together with the size of wins vs losses", "Trading as often as possible"]
answer = 2
why = "Both how often you win and how big wins are compared with losses matter."

[[quiz]]
id = "q5"
q = "Which habit best describes a trader rather than a gambler?"
options = ["Following the same written rules every time", "Betting bigger after a loss to win it back", "Trusting a strong feeling", "Skipping the stop-loss when sure"]
answer = 0
why = "A repeatable process, not conviction, is what separates trading from gambling."

[[recall]]
id = "r1"
q = "Being right on most trades guarantees you make money."
options = ["False", "True"]
answer = 0
why = "If losses are much bigger than wins, a high win rate can still lose money."

[[recall]]
id = "r2"
q = "5 wins of $200 and 5 losses of $100. Total?"
options = ["+$500", "$0", "−$500"]
answer = 0
why = "$1,000 won minus $500 lost."

[[recall]]
id = "r3"
q = "Why must CFD brokers show a risk warning?"
options = ["Most retail accounts lose money", "It's a marketing tradition", "To advertise their bonuses"]
answer = 0
why = "Regulators require it because most retail CFD accounts lose money."

[explain_back]
prompt = "Can you lose more trades than you win and still make money? How?"
must_include = ["most retail CFD accounts lose money", "results depend on the size of wins compared with losses, not just the win rate", "keeping losses small and planned (with a stop) while letting wins be bigger"]
misconceptions = ["being right most of the time is what makes money", "trading is a quick way to get rich"]

[[explain_back.fixtures]]
answer = "Yes. Most retail CFD accounts actually lose money, so being careful matters. What counts is how big your wins are compared with your losses: if I keep every loss small with a stop and let my wins be about twice as big, I can be wrong half the time and still come out ahead."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Yes, if your wins are bigger than your losses you can lose more often and still make money overall."
expect = "almost"

[[explain_back.fixtures]]
answer = "No. To make money you have to be right most of the time, so the trick is to find a strategy that wins 80% or 90% of trades and then trade a lot to get rich quickly."
expect = "almost_or_not_yet"
+++
