+++
id = 9
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
q = "Gold is at 4,100. You want to buy only if it dips to 4,080. Which order?"
options = ["Market buy", "Buy limit at 4,080", "Buy stop at 4,080", "Sell limit at 4,080"]
answer = 1
why = "A buy limit waits below the current price for a better entry."

[[quiz]]
q = "Gold is at 4,100. You want to buy only if it breaks above 4,120. Which order?"
options = ["Buy limit at 4,120", "Buy stop at 4,120", "Sell stop at 4,120", "Market buy now"]
answer = 1
why = "A buy stop sits above the current price and triggers when price rises to it."

[[quiz]]
q = "Which order gets you into the market immediately?"
options = ["Market order", "Buy limit", "Sell stop", "Take-profit"]
answer = 0
why = "A market order fills now at the current price."

[[quiz]]
q = "You buy at 4,100 with a stop-loss at 4,090 and a take-profit at 4,130. What is the most you plan to lose per ounce?"
options = ["$1", "$10", "$30", "$40"]
answer = 1
why = "4,100 − 4,090 = $10 per ounce, if the stop fills at its price."

[[quiz]]
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
+++

So far a “trade” has meant clicking buy or sell. Real platforms give you more control than that: you can tell the platform **when** to get in and **when** to get out, then walk away. That’s what order types are for.

### Getting in now: the market order

A **market order** says “buy (or sell) right now at the current price”. A market buy fills near the ask; a market sell fills near the bid. It’s simple and certain to fill. The catch is that you take whatever price is available at that moment, and in a fast market that can be a little different from the price you saw when you clicked.

### Getting in later: pending orders

A **pending order** waits for price to reach a level you choose. There are two families, and the trick is where they sit compared with the current price.

**Limit orders** wait for a **better** price:

- **Buy limit**: placed **below** the current price. “Buy only if gold gets cheaper, at this level.”
- **Sell limit**: placed **above** the current price. “Sell only if gold gets more expensive, at this level.”

**Stop orders** wait for price to **travel through** a level:

- **Buy stop**: placed **above** the current price. “Buy only if gold rises to here.”
- **Sell stop**: placed **below** the current price. “Sell only if gold falls to here.”

A limit order may **never fill**: if gold never dips to your buy limit, you never get in. That’s the price of asking for a better entry.

### Getting out: stop-loss and take-profit

Once you’re in a trade, two more orders protect you:

- A **stop-loss** closes the trade if price moves against you to a level you set. It limits your loss.
- A **take-profit** closes the trade if price reaches your target. It locks in the gain.

Buy at 4,100.00 with a stop-loss at 4,090.00 and a take-profit at 4,130.00, and you’ve decided in advance: risk $10 per ounce to try to make $30 per ounce. The platform handles the rest, even while you sleep.

### A stop isn’t a guarantee

When price reaches your stop-loss, it becomes an instruction to close at the next available price. Usually that’s at or very near your level. But if price jumps, for example on big news, the next available price can be worse. This is called **slippage**, and you’ll meet it again in the next lesson. A stop-loss still matters enormously: a slightly worse exit is far better than no exit at all.

### The habit to build

From your very first demo trade, **every trade gets a stop-loss**, set the moment you open it, never “later”. It’s the one rule in this course that has no exceptions.
