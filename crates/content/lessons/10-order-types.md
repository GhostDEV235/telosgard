+++
id = 10
stage = 1
title = "Order types, stop-loss & take-profit"
objective = "choose a market, limit or stop order for what you intend, with a stop-loss and take-profit attached."
hook = "You like gold, but only at a lower price. Do you have to sit and watch the screen until it gets there?"
analogy = "Think of a shop. Paying at the till right now is a market order. Leaving a written offer, “I’ll buy it if the price drops to $80”, is a limit order. Telling the shop “sell me one only if it becomes popular and goes above $120” is a stop order. Each one says when you are willing to act."
analogy_tag = "shop till and written offers"
example = "Gold is at 4,100.00. A market buy fills now, near the ask. A buy limit at 4,080.00 waits for a dip and may never fill. A buy stop at 4,120.00 triggers only if price rises through 4,120. Attach a stop-loss at 4,090.00 and a take-profit at 4,130.00 to a buy at 4,100.00, and the trade closes itself at either level. (Illustrative prices.)"
mistake = "Thinking a limit order always fills, or that a stop-loss guarantees the exact exit price. A limit may never be reached, and in a fast market a stop can fill at a worse price than the one you set."
takeaways = [
  "Market order: buy or sell now at the current price.",
  "Limit order: buy lower or sell higher than now; it may never fill.",
  "Stop order: buy above or sell below the current price once it is reached.",
  "Stop-loss and take-profit close your trade automatically.",
]
drills = ["order_type"]

[[videos]]
id = "yWTBndni56g"
title = "Market Order, Buy Limit, Sell Limit, Buy Stop, Sell Stop"
channel = "The Moving Average"
duration = "3:27"
watch_for = "Where each pending order sits relative to the current price: limits on the “better price” side, stops on the far side."
fallback_search = "buy limit sell limit buy stop sell stop explained"
checked = "2026-11-10"

[[videos]]
id = "Tiyystl8x40"
title = "Understanding Market, Limit, and Stop Orders"
channel = "Charles Schwab"
duration = "5:31"
watch_for = "Why a market order gets you in now while a limit order trades certainty of filling for a better price. The examples are stocks; the order types work the same way."
fallback_search = "market limit stop orders explained"
checked = "2026-11-10"

[[sources]]
claim = "MetaTrader 5 defines Buy Limit, Sell Limit, Buy Stop and Sell Stop pending orders, plus Stop Loss and Take Profit orders."
name = "MetaTrader 5 Help: Types of Orders"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade/general_concept/order_types"
checked = "2026-11-10"

[[sources]]
claim = "With market execution, the broker decides the execution price, so a fill can differ from the price you saw."
name = "MetaTrader 5 Help: Types of Execution"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade/general_concept/execution_types"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "Gold is at 4,100. You want to buy only if it dips to 4,080. Which order?"
options = ["Market buy", "Buy limit at 4,080", "Buy stop at 4,080", "Sell limit at 4,080"]
answer = 1
why = "A buy limit waits below the current price for a better entry."

[[quiz]]
id = "q2"
q = "Gold is at 4,100. You want to buy only if it breaks above 4,120. Which order?"
options = ["Buy limit at 4,120", "Buy stop at 4,120", "Sell stop at 4,120", "Market buy now"]
answer = 1
why = "A buy stop sits above the current price and triggers when price rises to it."

[[quiz]]
id = "q3"
q = "Which order gets you into the market immediately?"
options = ["Market order", "Buy limit", "Sell stop", "Take-profit"]
answer = 0
why = "A market order fills now at the current price."

[[quiz]]
id = "q4"
q = "You buy at 4,100 with a stop-loss at 4,090 and a take-profit at 4,130. What is the most you plan to lose per ounce?"
options = ["$1", "$10", "$30", "$40"]
answer = 1
why = "4,100 − 4,090 = $10 per ounce, if the stop fills at its price."

[[quiz]]
id = "q5"
q = "Big news hits and price jumps straight past your stop-loss. What can happen?"
options = ["It always fills exactly at your stop", "It can fill at a worse price", "The trade stays open", "The broker refunds the difference"]
answer = 1
why = "In a fast move a stop becomes a market order and can fill at the next available price."

[explain_back]
prompt = "You only want to buy if gold dips to 4,080. Which order do you use, and why not a market order?"
must_include = ["use a buy limit order at 4,080", "a market order fills immediately at the current price", "a limit order waits and may never fill", "a stop-loss and take-profit close the trade automatically"]
misconceptions = ["a limit order always fills", "a stop-loss guarantees the exact exit price"]

[[explain_back.fixtures]]
answer = "A buy limit at 4,080. A market order would buy right now at today's higher price, while a limit waits for the dip and might never fill if gold doesn't get there. I'd attach a stop-loss and take-profit so it closes itself either way."
expect = "got_it"

[[explain_back.fixtures]]
answer = "A buy limit at 4,080, because a market order would buy immediately at the current price instead of waiting."
expect = "almost"

[[explain_back.fixtures]]
answer = "A buy stop at 4,080, and it's guaranteed to fill at exactly 4,080 so there's no risk of a worse price."
expect = "almost_or_not_yet"

[[steps]]
heading = "Getting in now: market orders"
text = "A **market order** buys or sells right now at the current price: buys near the ask, sells near the bid. It's simple and certain to fill, but you take whatever price is available, which in a fast market can differ from what you saw."
[[steps]]
heading = "Limit orders: a better price"
text = "A **buy limit** sits **below** the current price: “buy only if gold gets cheaper, at this level.” A **sell limit** sits **above** it. Limits wait for a better price, and **may never fill** if price doesn't get there."
[[steps]]
heading = "Stop orders: through a level"
text = "A **buy stop** sits **above** the current price: “buy only if gold rises to here.” A **sell stop** sits **below** it. Stop orders trigger only once price travels through your level."
[[steps]]
heading = "Getting out: stop-loss and take-profit"
text = "A **stop-loss** closes the trade if price moves against you to your level; a **take-profit** closes it at your target. Set both when you open the trade, and the platform handles the rest, even while you sleep."
why = "Buy at 4,100.00 with a stop-loss at 4,090.00 and a take-profit at 4,130.00, and you've decided in advance: risk $10 per ounce to try to make $30 per ounce."
widget = "stop_first"
[[steps]]
heading = "A stop isn't a guarantee"
text = "When price reaches your stop, the trade closes at the next available price, usually at or near your level. In a jump, such as big news, it can be worse: that's **slippage**. Still, every trade gets a stop, set the moment you open it."
why = "This is the one rule in the course with no exceptions. A slightly worse exit is far better than no exit at all."
[[recall]]
id = "r1"
q = "Gold is at 4,100. You want to buy only if it dips to 4,080. Use a…"
options = ["Buy limit", "Buy stop", "Market order"]
answer = 0
why = "A buy limit waits below the price for a better entry."
[[recall]]
id = "r2"
q = "You want to buy only if gold rises through 4,120. Use a…"
options = ["Buy stop", "Buy limit", "Sell stop"]
answer = 0
why = "A buy stop sits above the price and triggers on the way up."
[[recall]]
id = "r3"
q = "Does a stop-loss guarantee your exact exit price?"
options = ["No, it can slip in a fast move", "Yes, always"]
answer = 0
why = "It closes at the next available price."
+++
