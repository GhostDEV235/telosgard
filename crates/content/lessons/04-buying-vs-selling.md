+++
id = 4
stage = 1
title = "Buying vs selling"
objective = "explain going long and going short, and when each one makes or loses money."
hook = "Can you make money when the price of gold is falling?"
analogy = "Going long is like buying an umbrella before the rainy season: you expect it to be worth more later. Going short is like borrowing a friend’s umbrella, selling it today for $20, then buying an identical one for $12 when the rain stops and giving it back. You keep the $8 difference, but if umbrellas got more expensive instead, you’d have to pay more to replace it."
analogy_tag = "umbrellas"
example = "Long: you buy at 4,150.00 and close at 4,160.00, a $10 move in your favour. Short: you sell at 4,150.00 and close by buying back at 4,140.00, also a $10 move in your favour. If the short had been closed at 4,160.00 instead, that same $10 move would have been a loss. (Illustrative prices.)"
mistake = "Thinking you must own gold before you can sell it, or that “selling” only means closing a buy. With a broker you can open a trade by selling, and close it later by buying."
takeaways = [
  "Going long = buy first, sell later; you profit if the price rises.",
  "Going short = sell first, buy back later; you profit if the price falls.",
  "Each one loses money if price moves the other way.",
  "With a broker you can open a trade in either direction.",
]
drills = ["candle_direction"]

[[videos]]
id = "fXnCtGcvqdk"
title = "Long Trade vs  Short Trade (Explained In Less Than 4 Minutes)"
channel = "Whyze"
duration = "3:33"
watch_for = "How the same price move is a win for one direction and a loss for the other."
fallback_search = "long vs short trade explained"
checked = "2026-11-10"

[[sources]]
claim = "In MetaTrader 5, a buy is executed at the Ask price and a sell at the Bid price, and trades can be opened in either direction."
name = "MetaTrader 5 Help: Types of Orders"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade/general_concept/order_types"
checked = "2026-11-10"

[[sources]]
claim = "Products such as CFDs can be used to speculate on both the rise and fall in an asset’s price."
name = "FCA: Understanding high-risk investments"
url = "https://www.fca.org.uk/investsmart/understanding-high-risk-investments"
checked = "2026-11-10"

[[quiz]]
q = "You think gold will rise. Which trade fits that view?"
options = ["Go long (buy)", "Go short (sell)", "Either; it makes no difference", "Neither; you can only profit from falls"]
answer = 0
why = "Going long profits when the price rises."

[[quiz]]
q = "You go short at 4,150.00 and close at 4,138.00. What’s the result?"
options = ["A $12 profit", "A $12 loss", "No change", "A $1.20 profit"]
answer = 0
why = "You sold at 4,150 and bought back at 4,138, $12 lower. Price fell, so the short made money."

[[quiz]]
q = "You go long at 4,150.00 and close at 4,143.00. What’s the result?"
options = ["A $7 profit", "A $7 loss", "No change", "A $70 loss"]
answer = 1
why = "You bought at 4,150 and sold at 4,143: price fell $7 against your long."

[[quiz]]
q = "Do you need to own gold before you can go short with a broker?"
options = ["Yes, always", "No, you can open a trade by selling", "Only on weekends", "Only if you hold a gold bar"]
answer = 1
why = "With a broker you trade the price, so you can open a position by selling."

[[quiz]]
q = "Your short trade’s price is rising. What is happening to your trade?"
options = ["It is making money", "It is losing money", "Nothing until you close it", "It becomes a long automatically"]
answer = 1
why = "A short loses money when price rises, because you’ll have to buy back higher than you sold."

[explain_back]
prompt = "How can you make money when gold’s price falls?"
must_include = ["you sell first (go short) and buy back later", "you profit if the price falls", "you lose if the price rises instead"]
misconceptions = ["you must own gold before you can sell it", "selling only means closing a buy"]

[[explain_back.fixtures]]
answer = "You go short: you sell first at the current price and buy back later. If gold falls you buy back cheaper and keep the difference, but if it rises you have to buy back higher and you lose."
expect = "got_it"

[[explain_back.fixtures]]
answer = "You sell first and then buy it back later at a lower price, keeping the difference."
expect = "almost"

[[explain_back.fixtures]]
answer = "You can't really. You first have to buy gold and own it, and selling just means getting out of that buy."
expect = "almost_or_not_yet"
+++

In Lesson 1 you learned that trading is about the change in price. Most people picture only one way to profit from a change: buy low, sell high. Traders have a second way, and it’s just as normal.

### Going long

**Going long** means you **buy first** and **sell later**. You profit if the price **rises** between the two, and you lose if it falls. This is the version everyone already understands.

- Buy at 4,150.00, sell at 4,160.00 → **+$10 move** in your favour.
- Buy at 4,150.00, sell at 4,142.00 → **−$8 move** against you.

### Going short

**Going short** means you **sell first** and **buy back later**. You profit if the price **falls** between the two, and you lose if it rises.

- Sell at 4,150.00, buy back at 4,140.00 → **+$10 move** in your favour.
- Sell at 4,150.00, buy back at 4,158.00 → **−$8 move** against you.

It feels strange to sell something you don’t own. Remember that with a broker you’re trading the **price**, not delivering metal. Opening a trade by selling is simply the broker’s way of letting you take the view that price will fall. When you close a short, the platform buys back for you automatically.

### Same move, opposite results

Here is the key idea: **the same price move is a win for one side and a loss for the other.** If gold rises $10, every long trader gains on that move and every short trader loses on it. The market doesn’t care which side you chose. Your job is to choose a side for a good reason, and to have a plan for when you’re wrong.

### Words you’ll hear

- **Buy**, **long** and **bullish** all describe the view that price will go **up**.
- **Sell**, **short** and **bearish** all describe the view that price will go **down**.
- **Closing** a trade means doing the opposite of how you opened it: a long is closed by selling, a short by buying back.

### Why this matters for gold

Gold doesn’t only rise. It has long falling stretches, sharp drops after news, and choppy weeks that go nowhere. Being able to trade in both directions means a falling market isn’t automatically bad news for you. But it also means you can be wrong in both directions, so later lessons will make sure every trade, long or short, has a plan for limiting the loss.
