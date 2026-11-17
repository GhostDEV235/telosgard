+++
id = 102
stage = 10
title = "Asian-range breakout"
objective = "mark the Asian session's high and low, trade a break of it only inside a fixed window after London opens, and know exactly where the idea is proven wrong."
hook = "Overnight, while Tokyo traded, gold drifted inside a $13 box. London is about to open. Which way will it leave the box, and how will you know you were wrong?"
analogy = "Think of a kettle on a low heat. For a while nothing much happens; the pressure just builds. When the heat goes up, it boils over one side or the other. You don't guess the side in advance: you watch which way it goes, and you decide beforehand what would mean you read it wrong."
analogy_tag = "the kettle"
example = "Asian range: 4,118 to 4,131, $13 wide, with H1 ATR at $7. The 3:00 to 4:00 New York candle closes at 4,136, above the box and with the H4 uptrend. Entry 4,136, stop below the box's middle (4,124.50) at 4,124: a $12 risk. Target 4,160, twice the risk, just before the next H4 zone at 4,165. On a $5,000 account at 1%, that's 0.04 lots. (Illustrative prices.)"
mistake = "Trading the break whenever it happens. A break at 11 am New York, hours after London opened, isn't this setup any more: the energy of the open has passed and the day's direction may already be set. Outside the window, there is no Asian-range trade."
takeaways = [
  "Mark the Asian range: the highest high and lowest low from the Asian session's start until London opens.",
  "Trade only an H1 close beyond the range inside a fixed window, the first three hours after London opens, ideally with the H4 trend.",
  "The idea is wrong if price falls back past the middle of the range: the stop goes there, and the target is at least twice the risk.",
  "Skip it when the range is too wide for a 2:1 target, when the break goes against the H4 trend, or when the window has closed.",
]

[[steps]]
heading = "Mark the box"
text = "On H1, mark the **highest high** and **lowest low** from the start of the Asian session until **London opens** (3 am New York time; the app's session clock shows it in your time). That's the **Asian range**. Its middle, halfway between the two, matters later."
why = "Asian hours are usually calmer for gold than London and New York, so price often drifts sideways and builds a range. CME Group's course calls this consolidation: neither side can push price beyond it."

[[steps]]
heading = "A fixed window"
text = "Only trade a break during the **first three hours** after London opens: 3 to 6 am New York time. You need an **H1 candle that closes** beyond the range. A break **with the H4 trend** is the one to take. When the window closes with no break, there's no trade today."
why = "London is the centre of over-the-counter gold trading. Its open often brings the activity that breaks the overnight range; later in the day, a break of the old box means much less."

[[steps]]
heading = "Clear invalidation"
text = "If price falls back past the **middle of the range**, the break has failed. Put the **stop** just beyond that middle. The target is at least **twice the risk**, and must be reachable before the next H4 zone. Size it to your plan's risk."
why = "A clean break shouldn't come back halfway into the box. Using the middle keeps the stop at a level that actually means something, not an arbitrary number."

[[steps]]
heading = "When to skip"
text = "Skip when the range is **too wide** for a 2:1 target before the next zone, when the break goes **against the H4 trend**, when a candle closes back inside the range before you enter, or on a **London bank holiday**. And like every Advanced strategy, add it to a **new plan version** first."
why = "A wide range means a far stop, a small size and a target that may never come. Fewer, cleaner trades beat forcing one every morning."

[[videos]]
id = "h2Q7SGbd4o4"
title = "London Breakout Strategy the Right Way"
channel = "The Moving Average"
duration = "2:45"
watch_for = "How a range built before the London open is marked and traded once London starts. His instruments and settings may differ; this course's window and stop rules come first."
fallback_search = "london breakout strategy asian range"
checked = "2026-11-17"

[[videos]]
id = "8P9BHSVD_vI"
title = "How To Trade The Opening Range Breakout (ORB)"
channel = "BrandonTrades"
duration = "8:22"
watch_for = "The same idea applied to a stock market's opening range: mark the range, wait for the break, know where you're wrong. A different market; the logic carries over."
fallback_search = "opening range breakout explained"
checked = "2026-11-17"

[[sources]]
claim = "Consolidation means price is unable to break out of a defined range because neither buyers nor sellers can move it beyond a certain price; price tends to exit a consolidation in the same direction that it entered it."
name = "CME Group Education: Trend and Continuation Patterns"
url = "https://www.cmegroup.com/education/courses/technical-analysis/trend-and-continuation-patterns.html"
checked = "2026-11-17"

[[sources]]
claim = "London's over-the-counter market and New York's futures market are among gold's major trading hubs."
name = "World Gold Council: Global gold market"
url = "https://gold.org/gold-market-structure/global-gold-market"
checked = "2026-11-17"

[[sources]]
claim = "A study of COMEX gold futures found that intraday patterns have a profound impact on the dynamics of gold's return volatility."
name = "Journal of Futures Markets (2001): What moves the gold market?"
url = "https://ideas.repec.org/a/wly/jfutmk/v21y2001i3p257-278.html"
checked = "2026-11-17"

[[quiz]]
id = "q1"
q = "What is the Asian range?"
options = ["The high and low from the Asian session's start until London opens", "The day's full range", "The range of the last H4 candle", "The distance between two round numbers"]
answer = 0
why = "It's the overnight box before London."

[[quiz]]
id = "q2"
q = "Range 4,118–4,131. At 10:30 am New York, an H1 candle closes at 4,140. Do you trade it as an Asian-range breakout?"
options = ["No: the 3–6 am window has closed", "Yes: it closed above the range", "Yes, with a smaller size", "Only if London is still open"]
answer = 0
why = "Outside the window, there's no Asian-range trade."

[[quiz]]
id = "q3"
q = "Range 4,118–4,131. Where does the idea become wrong after a break upward?"
options = ["When price falls back past the middle, about 4,124.50", "At 4,131, the range high", "At 4,100", "There's no fixed point"]
answer = 0
why = "A clean break shouldn't come back halfway into the box."

[[quiz]]
id = "q4"
q = "The range is $40 wide and the next H4 zone is $25 above the high. What do you do?"
options = ["Skip: the stop is too far for a 2:1 target", "Trade with a tighter stop", "Trade with double size", "Wait for 11 am"]
answer = 0
why = "A wide range makes the reward-to-risk impossible."

[[quiz]]
id = "q5"
q = "Two breaks inside the window: one with the H4 uptrend, one against it. Which fits the rules?"
options = ["The one with the H4 trend", "The one against it", "Both", "Neither"]
answer = 0
why = "A break against the H4 trend is a skip."

[[recall]]
id = "r1"
q = "The trading window is…"
options = ["The first three hours after London opens", "All day", "The Asian session"]
answer = 0
why = "3 to 6 am New York time."

[[recall]]
id = "r2"
q = "The stop goes beyond…"
options = ["The middle of the range", "The day's high", "A round number of your choice"]
answer = 0
why = "That's where the break has failed."

[[recall]]
id = "r3"
q = "A break needs…"
options = ["An H1 candle closing beyond the range", "A wick touching the range"]
answer = 0
why = "A close, not a touch."

[explain_back]
prompt = "How do you trade the Asian-range breakout, and where is it proven wrong?"
must_include = ["mark the high and low from the Asian session's start until London opens", "only trade an H1 close beyond the range in the first three hours after London opens, ideally with the H4 trend", "the stop goes just beyond the middle of the range, the target is at least twice the risk, and you skip ranges too wide for that or breaks outside the window"]
misconceptions = ["you can trade the break of the Asian range at any time of day", "the stop can go anywhere as long as it's small"]

[[explain_back.fixtures]]
answer = "I mark the highest high and lowest low on H1 from the start of the Asian session until London opens. Then I only trade an H1 candle that closes beyond that box in the first three hours after London opens, 3 to 6 am New York, and preferably with the H4 trend. If price comes back past the middle of the range the break has failed, so my stop goes just beyond the middle, and I need at least twice the risk before the next zone. If the range is too wide for that, the break is against the H4 trend, or the window has closed, I skip."
expect = "got_it"

[[explain_back.fixtures]]
answer = "I mark the Asian high and low and buy or sell when an H1 candle closes outside it after London opens, with a 2:1 target."
expect = "almost"

[[explain_back.fixtures]]
answer = "Whenever gold breaks the overnight range, morning or afternoon, I jump in with a small stop just behind the candle."
expect = "almost_or_not_yet"
+++
