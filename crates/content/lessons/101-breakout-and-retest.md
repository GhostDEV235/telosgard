+++
id = 101
stage = 10
title = "Breakout & retest"
objective = "trade a breakout only after price comes back to test the broken zone and a confirmation candle closes, and recognise the quiet-day false breaks to skip."
hook = "Gold has been stopped at the same ceiling three times. This morning a candle finally closed above it. Do you buy now, or wait?"
analogy = "Think of a crowd pushing through a stadium gate. Once it's open, the gate becomes the way in, and the barrier is behind them. But a few people slipping under the barrier while nobody's watching doesn't mean the gate is open. You wait to see whether the crowd holds on the far side."
analogy_tag = "the stadium gate"
example = "Gold ranged under a 4,150 zone for two days, turned back three times. Tuesday at 9:00 New York time an H1 candle closes at 4,162. No chase. Wednesday at 4:00 price dips back to 4,151 and a bullish pin bar closes at 4,156, its wick at 4,148. H1 ATR is $6: stop at 4,142, a $14 risk. The next zone is 4,190: a $34 reward, about 2.4:1. Sized for 1%, it's a trade. (Illustrative prices.)"
mistake = "Buying the breakout candle itself. Breaks often snap back into the range, especially on quiet days, and an entry at the top of the breakout candle leaves the stop far away and the reward small. If price never returns to test the zone, you simply miss that one."
takeaways = [
  "A breakout is a candle that closes clearly beyond a zone price has respected several times; a wick poking through is not a break.",
  "Don't chase the breakout candle: wait for price to come back to the broken zone and for a confirmation candle to close in the breakout's direction.",
  "Stop about 1–1.5 × ATR beyond the retest candle's wick, target the next zone, at least 2:1, sized to your plan's risk.",
  "Be wary of breaks on quiet days or against the H4 trend: if price closes back inside the old range, the setup is dead.",
]

[[steps]]
heading = "What counts as a breakout"
text = "Start with a **zone** price has respected at least twice (Lesson 24). A **breakout** is an H1 candle that **closes** clearly beyond it. A wick that pokes through and closes back inside is not a break. On a range, look for the break in the direction of the **H4 trend**."
why = "CME Group's course describes consolidation as a range neither buyers nor sellers can push beyond, and notes that price tends to leave a consolidation in the direction it entered it."

[[steps]]
heading = "Wait for the retest"
text = "Don't buy the breakout candle. Wait for price to come **back to the broken zone**: old resistance, now acting as support (or the reverse for a sell). Then wait for a **confirmation candle** (pin bar or engulfing, Lesson 26) to **close** in the breakout's direction. No retest, no trade."
why = "CME Group notes that when price breaks resistance, that level tends to become support when price comes back to it. The retest is where you find out whether it does."

[[steps]]
heading = "Stop, target, size"
text = "Put the **stop** about 1–1.5 × ATR beyond the retest candle's wick, which also puts it back inside the old range (Lesson 32). Aim for the **next zone**. The reward must be at least **twice** the risk. Then size it to your plan's risk per trade (Lesson 33)."
why = "If price falls back inside the old range, the break has failed and the idea is wrong. That is exactly where the stop belongs."

[[steps]]
heading = "False breaks on quiet days"
text = "Breaks fail most often when few people are trading: the **Asian session**, holidays, the Friday afternoon fade, or any day of tiny candles. If price **closes back inside** the range before your confirmation, the setup is **dead**; log it as skipped and move on."
why = "Without volume behind it, a break is often a few orders pushing price through thin liquidity. Real breaks tend to come when London or New York is trading."

[[steps]]
heading = "A new plan version"
text = "Breakout & retest is a new **Scenario** type. Before trading it on real money, add it to a **new version of your Trading Plan** with these rules, and practise it in demo first. Tag it in your journal so your review (Lesson 41) can compare it with your Classic setups."
why = "Every Advanced strategy needs its own written rules and its own record. Otherwise you can't tell whether it's helping or leaking."

[[videos]]
id = "CIsq8mHftAo"
title = "Why 90% of Traders FAIL the Break and Retest"
channel = "Tom Crown"
duration = "8:46"
watch_for = "The common ways a break and retest goes wrong, such as entering before the retest has shown its hand. His markets and figures are his own."
fallback_search = "break and retest mistakes"
checked = "2026-11-17"

[[videos]]
id = "WX0nV7mHxis"
title = "The ULTIMATE Break & Retest Strategy (Most Traders Fail Without This)"
channel = "Pro Trading School"
duration = "16:57"
watch_for = "How a broken level flips role on the retest, and why waiting for confirmation there matters. Ignore any promises; your plan's rules come first."
fallback_search = "break and retest strategy explained"
checked = "2026-11-17"

[[sources]]
claim = "Consolidation means price is unable to break out of a defined range because neither buyers nor sellers can move it beyond a certain price; price tends to exit a consolidation in the same direction that it entered it."
name = "CME Group Education: Trend and Continuation Patterns"
url = "https://www.cmegroup.com/education/courses/technical-analysis/trend-and-continuation-patterns.html"
checked = "2026-11-17"

[[sources]]
claim = "When price breaks support or resistance, support becomes resistance and resistance becomes support."
name = "CME Group Education: Support and Resistance"
url = "https://www.cmegroup.com/education/courses/technical-analysis/support-and-resistance.html"
checked = "2026-11-17"

[[quiz]]
id = "q1"
q = "Resistance at 4,150. An H1 candle's wick reaches 4,156 but it closes at 4,147. Is that a breakout?"
options = ["No: it closed back below the zone", "Yes: price went above 4,150", "Yes, if it happened in London", "Only on H4"]
answer = 0
why = "A break needs a close beyond the zone."

[[quiz]]
id = "q2"
q = "An H1 candle closes well above resistance. What's your next move?"
options = ["Wait for price to come back to the broken zone and for a confirmation candle", "Buy at the close of the breakout candle", "Sell, because breakouts fail", "Move your target higher"]
answer = 0
why = "No chase: the retest and the confirmation are the setup."

[[quiz]]
id = "q3"
q = "After a break above 4,150, price drops back and closes at 4,141, inside the old range, before any confirmation. What now?"
options = ["The setup is dead: log it as skipped", "Buy: it's cheaper now", "Wait a few more hours for the retest", "Widen the stop"]
answer = 0
why = "A close back inside the range means the break failed."

[[quiz]]
id = "q4"
q = "Retest candle's wick at 4,148, H1 ATR $6, entry 4,156. Where does the stop go?"
options = ["About 4,142, roughly 1 ATR beyond the wick", "4,150, on the zone", "4,147, just under the wick", "4,120, far away to be safe"]
answer = 0
why = "1–1.5 × ATR beyond the wick, back inside the old range."

[[quiz]]
id = "q5"
q = "Which break deserves the most suspicion?"
options = ["One during a quiet Asian session on a holiday", "One during London with the H4 trend", "One that retests and confirms", "One on a busy New York morning"]
answer = 0
why = "Quiet markets produce false breaks."

[[quiz]]
id = "q6"
q = "What do you do before trading breakout & retest with real money?"
options = ["Add it to a new Trading Plan version and practise it in demo", "Nothing: it's like the Classic plays", "Double the risk because it's advanced", "Drop the 2:1 rule for breakouts"]
answer = 0
why = "New Scenario, new written rules, demo first."

[[recall]]
id = "r1"
q = "A breakout needs a candle that…"
options = ["Closes beyond the zone", "Touches the zone", "Has a long wick"]
answer = 0
why = "A wick alone isn't a break."

[[recall]]
id = "r2"
q = "You enter on…"
options = ["The retest, after a confirmation candle", "The breakout candle", "The first touch of the zone"]
answer = 0
why = "No retest, no trade."

[[recall]]
id = "r3"
q = "A close back inside the old range means…"
options = ["The setup is dead", "A better entry"]
answer = 0
why = "The break has failed."

[explain_back]
prompt = "How do you trade a breakout and retest, and when do you skip it?"
must_include = ["a breakout is a candle closing clearly beyond a zone respected at least twice, not a wick poking through", "wait for price to return to the broken zone and for a confirmation candle to close in the breakout direction, then stop about 1 to 1.5 ATR beyond its wick, target the next zone at 2:1 or better, sized to the plan's risk", "skip quiet-day breaks or breaks against the H4 trend, and drop the setup if price closes back inside the old range"]
misconceptions = ["you should buy the breakout candle as soon as price crosses the level", "any wick through a level is a breakout"]

[[explain_back.fixtures]]
answer = "First I need a zone price has respected at least twice, and a candle that closes clearly beyond it; a wick poking through doesn't count. I don't buy the breakout candle. I wait for price to come back to the broken zone and for a pin bar or engulfing candle to close in the breakout's direction, then put the stop about 1 to 1.5 ATR beyond its wick, aim for the next zone at 2:1 or better, and size it to my plan's risk. I'm suspicious of breaks on quiet days or against the H4 trend, and if price closes back inside the old range, the setup is dead."
expect = "got_it"

[[explain_back.fixtures]]
answer = "When a candle closes beyond a zone, I wait for price to come back to it and for a confirmation candle, then I enter with a stop beyond the wick and a 2:1 target."
expect = "almost"

[[explain_back.fixtures]]
answer = "As soon as price pokes above resistance I buy straight away, because waiting means missing the move."
expect = "almost_or_not_yet"
+++
