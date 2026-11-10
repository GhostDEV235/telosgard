+++
id = 1
stage = 1
title = "What is trading?"
objective = "explain what a trader is actually trying to profit from."
hook = "If you buy a gold bar and keep it in a drawer for ten years, are you trading?"
analogy = "Think of reselling concert tickets. You buy a ticket for $50 before the band announces it’s their final tour. Demand jumps, and you sell it for $80. You never went to the concert: your $30 came from the change in price. If the show had been cancelled, the price could just as easily have fallen, and you’d have sold at a loss."
analogy_tag = "concert tickets"
example = "Gold is at 4,100.00 and you buy. Later it’s at 4,120.00 and you sell. Gold moved $20 in your favour, and that $20 move is where your profit comes from. Had it fallen to 4,080.00 instead, the very same $20 move would have been a loss. (Illustrative prices.)"
mistake = "Thinking trading means owning gold for the long run. That’s investing. Trading is about the change in price over a shorter time, and every single trade can lose as well as win."
takeaways = [
  "A trader profits from a change in price, not from owning the thing.",
  "You buy and sell at different prices; the gap is your profit or your loss.",
  "You don’t need to own physical gold to trade its price.",
  "Price can move against you, so every trade can lose real money.",
]

[[videos]]
id = "dmqoqVwFopE"
title = "The Difference Between Trading and Investing"
channel = "The Plain Bagel"
duration = "10:29"
watch_for = "How the time you hold something, and what you’re hoping to earn, separates a trader from an investor."
fallback_search = "difference between trading and investing explained"
checked = "2026-11-10"

[[sources]]
claim = "Products such as CFDs are used to speculate on the rise and fall of an asset’s price, and carry a high chance of losing money."
name = "FCA: Understanding high-risk investments"
url = "https://www.fca.org.uk/investsmart/understanding-high-risk-investments"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "What is a trader trying to profit from?"
options = ["Owning gold for many years", "A change in price", "Interest paid by the broker", "Storing gold safely"]
answer = 1
why = "Trading profits from the difference between the price you buy at and the price you sell at."

[[quiz]]
id = "q2"
q = "You buy gold at 4,100.00 and sell at 4,115.00. How big was the move?"
options = ["$1.50", "$15", "$150", "$1,500"]
answer = 1
why = "4,115 − 4,100 = $15. That’s the size of the move, and it went your way."

[[quiz]]
id = "q3"
q = "You buy at 4,140.00 and sell at 4,130.00. What happened?"
options = ["A $10 profit", "A $10 loss", "Nothing: the price came back", "A $100 profit"]
answer = 1
why = "Price fell $10 after you bought, so the move went against you."

[[quiz]]
id = "q4"
q = "Do you need to own a physical gold bar to trade XAUUSD online?"
options = ["Yes, the broker posts it to you", "Yes, it’s held in a vault in your name", "No, you trade the price", "Only for large trades"]
answer = 2
why = "Online gold trading settles the price difference in money; no bar changes hands."

[[quiz]]
id = "q5"
q = "Which of these is trading rather than investing?"
options = ["Buying gold coins for retirement", "Buying in the morning and selling that afternoon after a $12 rise", "Inheriting gold jewellery", "Keeping gold in a safe for 20 years"]
answer = 1
why = "Trading aims to profit from shorter price moves; investing holds for the long term."

[explain_back]
prompt = "In your own words: what is a trader actually trying to profit from?"
must_include = ["profit comes from the change in price", "you buy and sell at different prices", "a trade can lose as well as gain"]
misconceptions = ["trading is the same as investing for the long term", "you need to own physical gold to trade it"]

[[explain_back.fixtures]]
answer = "A trader tries to make money from the price changing. You buy at one price and sell at another, and the difference is your profit, but if the price goes the wrong way you lose money instead."
expect = "got_it"

[[explain_back.fixtures]]
answer = "They want to profit from the price changing, buying at one price and selling at a higher one."
expect = "almost"

[[explain_back.fixtures]]
answer = "A trader buys gold bars and keeps them safe for many years so they are worth more when they retire."
expect = "almost_or_not_yet"

[[steps]]
heading = "Trading is about the change"
text = "Trading means buying at one price and selling at another, aiming for the difference to land in your favour. A trader cares about one thing: will the price be higher or lower later? **Your profit or loss is the change in price** between getting in and getting out."
widget = "price_drag"
[[steps]]
heading = "You don't need to own the gold"
text = "When you trade gold online, no metal changes hands. You open a position at one price, close it at another, and the difference is settled in money. You're trading **the price**, which is also why you can make money when it falls (Lesson 5)."
why = "That's why you can trade gold from a phone in your kitchen without ever touching a gold bar."
[[steps]]
heading = "The difference can go against you"
text = "If you buy and the price drops, the gap is a loss: real money leaving your account. There is no version of trading where you only win. Good traders lose often; what keeps them in the game is that their losses are small and planned."
why = "That's what most of this course is about: making sure that when you're wrong, it costs a little, and only what you planned."
[[steps]]
heading = "Trading is not investing"
text = "An **investor** buys and holds for years, expecting value to grow. A **trader** is in and out over minutes, hours or days, aiming to profit from shorter moves. Different jobs, different skills. This course teaches the trader's."
why = "Every decision later comes back to one question: *where do I think price is going next, and what happens to me if I'm wrong?* Remember that trading is about the change in price, and that the change can go against you."
[[recall]]
id = "r1"
q = "A trader's profit comes from…"
options = ["The change in price", "Owning the metal", "Interest from the broker"]
answer = 0
why = "Trading profits from the difference between your buy and sell prices."
[[recall]]
id = "r2"
q = "You buy at 4,100 and sell at 4,090. What happened per ounce?"
options = ["A $10 loss", "A $10 profit", "Nothing"]
answer = 0
why = "Price fell $10 after you bought."
[[recall]]
id = "r3"
q = "Holding gold for 20 years is…"
options = ["Investing", "Trading", "Neither"]
answer = 0
why = "Long holding is investing; trading aims at shorter moves."
+++
