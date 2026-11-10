+++
id = 3
stage = 1
title = "What is gold, and why does the world want it?"
objective = "name three reasons people buy gold and connect them to its price."
hook = "Why does a shiny yellow metal still matter in a world of apps and digital money?"
analogy = "Think of gold as the world’s emergency savings jar. Most of the time it sits on the shelf. When people get nervous about jobs, banks or wars, they reach for the jar, and when everyone reaches for the same jar at once, what’s inside it becomes more valuable."
analogy_tag = "emergency savings jar"
example = "Imagine gold is at 4,100.00 on a calm Monday. On Tuesday, alarming news breaks and investors around the world move money into gold. More buyers than sellers push the price to 4,150.00: a $50 move in a day. Nothing about the metal changed; only how many people wanted it. (Illustrative prices.)"
mistake = "Believing gold only ever goes up, or that it pays you while you hold it. Gold can fall for long stretches, and it pays no interest or dividends: you only gain if someone later pays more than you did."
takeaways = [
  "People want gold for jewellery, investment, central bank reserves and technology.",
  "Gold is seen as holding its value, so demand often rises in times of fear.",
  "More demand than supply pushes the price up; less demand lets it fall.",
  "Gold pays no interest, and it does not only go up.",
]

[[videos]]
id = "ezolUaQ37xE"
title = "CNA Explains: Why is gold a safe haven asset?"
channel = "CNA"
duration = "3:55"
watch_for = "Why investors move money into gold when they are worried, and what that does to its price."
fallback_search = "why is gold a safe haven asset explained"
checked = "2026-11-10"

[[videos]]
id = "qOpzs7IP51U"
title = "The Volatility of the Gold Market, Explained"
channel = "WSJ | The Wall Street Journal"
duration = "5:41"
watch_for = "The idea that gold’s price can swing sharply, in both directions."
fallback_search = "gold market volatility explained"
checked = "2026-11-10"

[[sources]]
claim = "Gold demand comes from jewellery, bars and coins, ETFs, central banks and technology."
name = "World Gold Council: Gold Demand Trends"
url = "https://www.gold.org/goldhub/research/gold-demand-trends"
checked = "2026-11-10"

[[quiz]]
id = "q1"
q = "Which of these is NOT one of the main sources of demand for gold?"
options = ["Jewellery", "Central banks", "Monthly interest payments", "Technology"]
answer = 2
why = "Gold pays no interest. Demand comes from jewellery, investment, central banks and technology."

[[quiz]]
id = "q2"
q = "Why do people often buy gold when they’re scared?"
options = ["It is guaranteed to rise", "It is seen as holding its value in a crisis", "Brokers give it away free", "It pays a bonus in bad times"]
answer = 1
why = "Gold is seen as a safe haven: something that keeps its value when other things feel risky."

[[quiz]]
id = "q3"
q = "Many more people want to buy gold than sell it today. What usually happens to the price?"
options = ["It rises", "It falls", "It stays exactly the same", "Trading stops"]
answer = 0
why = "When buyers outnumber sellers, they compete on price and push it up."

[[quiz]]
id = "q4"
q = "Gold moves from 4,100.00 to 4,150.00 after scary news. How big was the move?"
options = ["$5", "$50", "$500", "$0.50"]
answer = 1
why = "4,150 − 4,100 = $50."

[[quiz]]
id = "q5"
q = "Which statement is true?"
options = ["Gold only ever goes up", "Gold pays dividends every year", "Gold can fall for long periods", "Gold’s price never changes on calm days"]
answer = 2
why = "Gold has had long falling periods. It is not a one-way bet."

[explain_back]
prompt = "Why do people rush to buy gold when they’re scared?"
must_include = ["gold is seen as holding its value over time", "it is a safe haven in fear or crisis", "more demand pushes the price up"]
misconceptions = ["gold only ever goes up", "gold pays interest or dividends"]

[[explain_back.fixtures]]
answer = "Because gold has kept its value for a very long time, people treat it as a safe place for money when things feel risky. When lots of people want it at once, the extra demand pushes the price up."
expect = "got_it"

[[explain_back.fixtures]]
answer = "When people are scared they want somewhere safe to put their money, and gold is seen as safe."
expect = "almost"

[[explain_back.fixtures]]
answer = "They buy gold because it always goes up in value and it pays them interest while they hold it."
expect = "almost_or_not_yet"

[[steps]]
heading = "Who wants gold?"
text = "The World Gold Council groups the world's demand into a few buckets: **jewellery**, **investment** (bars, coins and funds), **central banks** holding reserves, and a little for **technology**. Each buys more or less depending on prices, incomes and mood."
why = "Central banks keep gold in their reserves as savings that don't depend on any one government. Add every group together and you get the world's demand for gold."
[[steps]]
heading = "Why fear matters"
text = "Gold doesn't depend on a company's profits or a government's promises, so many people see it as **holding its value over time**. When the world feels uncertain, money moves into gold. That's why gold is called a **safe haven**."
[[steps]]
heading = "How demand becomes price"
text = "A price is where buyers and sellers agree to meet. More buyers than sellers, and buyers must offer more: **the price rises**. When demand fades, **the price falls**. You don't need to predict the news, only understand that news changes demand."
[[steps]]
heading = "What gold doesn't do"
text = "Gold pays you nothing while you hold it: no interest, no dividend. And it doesn't only go up; it has had long falling stretches. Anyone calling gold a sure thing is selling something."
why = "Later you'll see why paying nothing matters: when interest rates are high, holding something that earns nothing becomes less attractive."
[[recall]]
id = "r1"
q = "Which of these is a real source of demand for gold?"
options = ["Central banks", "Monthly gold dividends", "Broker bonuses"]
answer = 0
why = "Central banks hold gold in their reserves."
[[recall]]
id = "r2"
q = "Buyers suddenly outnumber sellers. The price usually…"
options = ["Rises", "Falls", "Stays the same"]
answer = 0
why = "Buyers have to offer more to get it."
[[recall]]
id = "r3"
q = "Does gold pay interest while you hold it?"
options = ["No", "Yes, monthly", "Only to big holders"]
answer = 0
why = "Gold earns nothing on its own; you gain only if the price rises."
+++
