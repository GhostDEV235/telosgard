+++
id = 12
stage = 1
title = "Your demo account & platforms"
objective = "open an MT5 demo, read XAUUSD’s contract details, and place and close a 0.01-lot trade with a stop-loss and take-profit."
hook = "How do you practise trading without risking a single dollar?"
analogy = "A demo account is a flight simulator. Pilots train on real instruments, in realistic conditions, where a crash costs nothing. You get the same: real gold prices, a real platform, and pretend money, so mistakes become lessons instead of losses."
analogy_tag = "flight simulator"
example = "In your MT5 demo, open the XAUUSD specification and note the contract size, minimum volume, volume step, swap rates and trading hours. Then place a 0.01-lot buy with a stop-loss $5 below your entry and a take-profit $10 above it, watch it for a few minutes, and close it by hand. With 0.01 lots, every $1 move is worth $1."
mistake = "Believing demo results guarantee live results, or that every broker’s gold contract is the same. Demo has no real money at stake, so emotions are different, and brokers set their own contract details."
takeaways = [
  "A demo account uses real prices with simulated money.",
  "Check the contract size, minimum lot, swap and trading hours in your broker’s XAUUSD spec.",
  "Attach a stop-loss to every trade, even on demo.",
  "Only use a broker you have checked on its regulator’s official register.",
]

[[videos]]
id = "QnnDgyJrPl0"
title = "How to Place Orders on MetaTrader 5"
channel = "Pepperstone Australia"
duration = "6:34"
watch_for = "Where the order window is and how volume, stop-loss and take-profit are entered. Ignore any broker-specific offers; the platform steps are the same everywhere."
fallback_search = "how to place orders metatrader 5"
checked = "2026-11-10"

[[videos]]
id = "CSF6bSFc24A"
title = "How to Set a Stop Loss and Take Profit on MetaTrader 5 (Android, iOS, Desktop, Web)"
channel = "Trading Heroes"
duration = "10:27"
watch_for = "Setting and editing a stop-loss and take-profit on the device you’ll actually use."
fallback_search = "set stop loss take profit metatrader 5"
checked = "2026-11-10"

[[sources]]
claim = "MetaTrader 5 lets you open a demo account."
name = "MetaTrader 5: Trading Platform"
url = "https://www.metatrader5.com/en/trading-platform"
checked = "2026-11-10"

[[sources]]
claim = "MetaTrader 5 supports Stop Loss and Take Profit orders on positions."
name = "MetaTrader 5 Help: Types of Orders"
url = "https://www.metatrader5.com/en/mobile-trading/iphone/help/trade/general_concept/order_types"
checked = "2026-11-10"

[[sources]]
claim = "NFA BASIC is a free tool to research US derivatives firms and professionals."
name = "NFA: BASIC"
url = "https://www.nfa.futures.org/basicnet/"
checked = "2026-11-10"

[[sources]]
claim = "ASIC’s registers show organisations licensed with ASIC to provide financial services in Australia."
name = "ASIC: Search ASIC registers"
url = "https://asic.gov.au/online-services/search-asic-s-registers/"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "What is a demo account?"
options = ["A live account with a small balance", "Real prices with simulated money", "A recording of past prices", "A paid trading course"]
answer = 1
why = "Demo uses live market prices but pretend money, so mistakes cost nothing."

[[quiz]]
id = "q2"
q = "Which of these should you check in your broker’s XAUUSD specification?"
options = ["The logo colour", "Contract size, minimum lot, swap and trading hours", "How many other people are trading", "Nothing; all brokers are the same"]
answer = 1
why = "Brokers set their own contract details, and every calculation depends on them."

[[quiz]]
id = "q3"
q = "On demo you place a 0.01-lot buy with a stop-loss $5 below entry. If the stop is hit, roughly how much do you lose?"
options = ["$0.05", "$0.50", "$5", "$50"]
answer = 2
why = "0.01 × 100 × $5 = $5 (simulated, on demo)."

[[quiz]]
id = "q4"
q = "How do you check that a broker is properly regulated?"
options = ["Trust the logo on its website", "Search the regulator’s official register yourself", "Ask in a social media group", "Check how many followers it has"]
answer = 1
why = "Licences can be faked. Look the firm up on the regulator’s own register."

[[quiz]]
id = "q5"
q = "You’re profitable on demo for two weeks. What does that prove about live trading?"
options = ["You will definitely profit live", "Nothing is guaranteed; real money changes how you feel and act", "Live trading is easier", "You can skip the stop-loss"]
answer = 1
why = "Demo builds skill and habits, but real money brings emotions that demo can’t."

[explain_back]
prompt = "Why do you practise on a demo first, and what must you check in your broker’s XAUUSD spec?"
must_include = ["a demo uses real prices with simulated money", "check contract size, minimum lot, swap and trading hours", "always attach a stop", "use a regulated broker"]
misconceptions = ["demo results guarantee live results", "every broker's gold contract is the same"]

[[explain_back.fixtures]]
answer = "A demo has real market prices but fake money, so mistakes are free while I learn. In the XAUUSD spec I'd check the contract size, the minimum lot, the swap rates and the trading hours, always put a stop on every order, and make sure the broker is regulated."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Because it's real prices with pretend money, so I can practise safely. I'd check how many ounces are in a lot."
expect = "almost"

[[explain_back.fixtures]]
answer = "So I can prove I'm profitable, because if it works on demo it will definitely work live. The specs don't matter since gold is the same at every broker."
expect = "almost_or_not_yet"

[[steps]]
heading = "Two tools"
text = "You'll use **MetaTrader 5 (MT5)**, a platform many brokers offer, to place practice trades, and **TradingView**, a charting website, to study gold's price. You'll use TradingView properly from Stage 4."
[[steps]]
heading = "Choose a broker carefully"
text = "Pick a broker **licensed by a financial regulator** where you live, and look it up yourself on the regulator's register: the FCA's in the UK, ASIC's in Australia, NFA BASIC in the US. If you can't find it, walk away."
why = "Even for a demo, choose one you could one day trust with real money. A licence number on a website can be faked; the regulator's own register can't. If the name doesn't match exactly, walk away."
[[steps]]
heading = "Open a demo and read the spec"
text = "Open an MT5 **demo account**: live prices, pretend money, with a balance close to what you'd really trade. In the XAUUSD specification, note the **contract size**, **minimum lot and step**, **swap** and **trading hours**."
why = "These numbers feed every calculation you'll make, so keep them somewhere handy. A huge pretend balance teaches bad habits."
[[steps]]
heading = "Your first practice trade"
text = "Open a **0.01-lot** order, where every $1 move is worth $1. Before confirming, set a stop-loss about $5 away and a take-profit about $10 away. Watch it start slightly negative (the spread), then close it."
why = "Then find it in your history and check the result matches lots × 100 × Dollar Move. You're not trying to make money yet; you're learning where the buttons are, so they never distract you later."
widget = "stop_first"
[[steps]]
heading = "Demo is training, not proof"
text = "Demo trading builds skill and habits. It isn't proof you'll make money live: with no real money at stake, it's easy to hold losers or take risky trades. That's why this course judges your demo trading on **discipline**, not profit."
[[recall]]
id = "r1"
q = "A demo account uses…"
options = ["Real prices and pretend money", "Old prices and real money", "Pretend prices"]
answer = 0
why = "Real market prices, simulated money."
[[recall]]
id = "r2"
q = "Where do you check a broker's licence?"
options = ["On the regulator's own register", "On the broker's website", "In a social media group"]
answer = 0
why = "Licences can be faked; the register can't."
[[recall]]
id = "r3"
q = "0.01 lots with a $5 stop: if the stop is hit you lose about…"
options = ["$5", "$50", "$0.50"]
answer = 0
why = "0.01 × 100 × $5 = $5."
+++
