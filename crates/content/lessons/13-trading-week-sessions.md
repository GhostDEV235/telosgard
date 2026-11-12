+++
id = 13
stage = 2
title = "The trading week & sessions"
objective = "describe gold's trading week and its three sessions, in New York time and in your own time."
hook = "If gold trades almost around the clock, does it matter what time you trade it?"
analogy = "Think of a relay race around the world. Tokyo carries the baton through the Asian day, hands it to London, and London hands it to New York. The race runs every weekday, but some legs have far more runners than others."
analogy_tag = "relay race"
example = "It's Tuesday, 3:00 am in New York. In winter that's 8:00 in London and 17:00 in Tokyo: Tokyo's session has just finished and London's is starting. Gold was open the whole time; what changed is who is trading it. (Winter times; in summer London is an hour different.)"
mistake = "Thinking gold is equally busy all day. It's open nearly 24 hours on weekdays, but some hours are quiet. Most of the action comes when London and New York are trading."
takeaways = [
  "Gold trades from Sunday 6 pm to Friday 5 pm New York time, with a one-hour break each weekday at 5 pm.",
  "Three sessions hand over around the world: Asian (Tokyo), then London, then New York.",
  "London and New York are gold's two biggest trading centres.",
  "The app's session clock shows every session in your own time zone, daylight saving included.",
]
drills = ["session"]

[[steps]]
heading = "Gold's trading week"
text = "Gold trades almost around the clock on weekdays. The week opens **Sunday at 6 pm New York time** and closes **Friday at 5 pm**. Each weekday there's a **one-hour break at 5 pm New York**, when the trading day rolls over. On weekends gold doesn't trade."
why = "These are the hours of gold futures on CME's Globex platform, and CFD brokers broadly follow them; your own broker's hours can differ slightly, so check its contract details. New York time is the reference because that's where the trading day is counted from."

[[steps]]
heading = "Three sessions around the world"
text = "Each day passes through three main sessions: **Asian** (Tokyo, 9 am–4 pm Tokyo time), **London** (8 am–5 pm London time) and **New York** (8 am–5 pm New York time). A session is simply when that city's banks and traders are at their desks."
widget = "session_clock"
why = "Session hours aren't official opening bells; they're conventions, and sources differ by an hour here or there. This course uses these definitions everywhere, so the session clock, your journal and your plan's Hard Rules always agree."

[[steps]]
heading = "London and New York lead"
text = "London is the centre of the world's over-the-counter gold trading, and New York is home to the biggest gold futures market (COMEX). Twice a day, at **10:30 and 15:00 London time**, the **LBMA Gold Price** auctions set a benchmark price used around the world."
why = "The LBMA Gold Price is set in electronic auctions run by ICE Benchmark Administration. You won't trade the auctions themselves, but they're a reminder of where gold's main trading happens."

[[steps]]
heading = "Clocks change, so sessions move"
text = "London and New York both use daylight saving, but they change clocks on **different Sundays**. For a few weeks each spring and autumn the gap between them is 4 hours instead of 5, and sessions shift in your local time. Don't memorise hours: let the app's clock convert them."
why = "The US changes on the second Sunday in March and the first Sunday in November; the UK on the last Sunday in March and in October. Tokyo doesn't change at all. The session clock does this arithmetic for your own time zone, using your device's settings."

[[videos]]
id = "znpD7T_9fMQ"
title = "Forex Trading Sessions Explained!"
channel = "SimulationFX"
duration = "6:39"
watch_for = "How the Asian, London and New York sessions hand over to each other, and why some hours are busier than others."
fallback_search = "trading sessions explained Asian London New York"
checked = "2026-11-12"

[[videos]]
id = "tW3F-Yv0ZwI"
title = "What's the Best Time to Trade Forex? | 3 Major Market Sessions"
channel = "TraderNick"
duration = "10:10"
watch_for = "Where the three sessions sit in the day and where they overlap. It's about currencies, but gold follows the same clock."
fallback_search = "best time to trade three major market sessions"
checked = "2026-11-12"

[[sources]]
claim = "Gold futures trade Sunday–Friday 6:00 pm–5:00 pm ET with a 60-minute break each day beginning at 5:00 pm ET."
name = "CME Group: E-mini Gold contract specs"
url = "https://cmegroup.com/markets/metals/precious/e-mini-gold.contractSpecs.html"
checked = "2026-11-12"

[[sources]]
claim = "The LBMA Gold Price is set twice daily in auctions run by ICE Benchmark Administration, starting at 10:30 and 15:00 London time."
name = "LBMA: LBMA Gold Price"
url = "https://www.lbma.org.uk/prices-and-data/lbma-gold-price/lbma-gold-price"
checked = "2026-11-12"

[[sources]]
claim = "London's over-the-counter market and New York's futures market are among gold's major trading hubs, and London's time zone bridges Asian and US trading hours."
name = "World Gold Council: Global gold market"
url = "https://gold.org/gold-market-structure/global-gold-market"
checked = "2026-11-12"

[[sources]]
claim = "UK clocks go forward on the last Sunday in March and back on the last Sunday in October."
name = "GOV.UK: When do the clocks change?"
url = "https://gov.uk/when-do-the-clocks-change"
checked = "2026-11-12"

[[sources]]
claim = "US daylight saving time begins on the second Sunday in March and ends on the first Sunday in November."
name = "US Naval Observatory: Daylight time"
url = "https://aa.usno.navy.mil/faq/daylight_time"
checked = "2026-11-12"

[[quiz]]
id = "q1"
q = "Gold's daily trading break happens at…"
options = ["5 pm New York time", "Midnight in London", "9 am Tokyo time", "There is no daily break"]
answer = 0
why = "Each weekday gold pauses for an hour from 5 pm New York time, when the trading day rolls over."

[[quiz]]
id = "q2"
q = "In which order do the main sessions run each weekday?"
options = ["New York → London → Asian", "Asian → London → New York", "London → Asian → New York", "All three at the same time"]
answer = 1
why = "The day moves west with the sun: Tokyo first, then London, then New York."

[[quiz]]
id = "q3"
q = "It's 9 am London time on a Wednesday in winter. Which session is open?"
options = ["London and New York", "Asian only", "London only", "None: gold is closed"]
answer = 2
why = "9 am London is 4 am in New York (before its session) and 18:00 in Tokyo (after its session), so only London is open."

[[quiz]]
id = "q4"
q = "When do the LBMA Gold Price auctions start?"
options = ["8:30 am New York time", "10:30 and 15:00 London time", "Every hour on the hour", "Only on Fridays"]
answer = 1
why = "The two daily auctions start at 10:30 and 15:00 London time."

[[quiz]]
id = "q5"
q = "Why might a session start an hour earlier or later in your local time for a few weeks of the year?"
options = ["Brokers move sessions for holidays", "Gold trades fewer hours in summer", "The US and UK change their clocks on different dates", "Tokyo uses daylight saving"]
answer = 2
why = "The US and UK change clocks on different Sundays, so for a few weeks the gap between them is 4 hours, not 5."

[[quiz]]
id = "q6"
q = "When does gold's trading week close?"
options = ["Friday 5 pm New York time", "Saturday noon London time", "Friday midnight UTC", "It never closes"]
answer = 0
why = "The week closes on Friday at 5 pm New York time and reopens on Sunday at 6 pm."

[[recall]]
id = "r1"
q = "When does gold's trading week open?"
options = ["Sunday 6 pm New York time", "Monday 9 am London time", "Saturday midnight UTC"]
answer = 0
why = "The week opens on Sunday evening, New York time."

[[recall]]
id = "r2"
q = "Which session comes right after the Asian session?"
options = ["London", "New York", "None: the market closes"]
answer = 0
why = "Tokyo hands over to London, and London hands over to New York."

[[recall]]
id = "r3"
q = "Why use the app's clock instead of memorising session hours?"
options = ["Daylight saving shifts them in your local time", "Sessions change every week", "Brokers don't publish hours"]
answer = 0
why = "Clock changes in the US and UK move the sessions in your own time zone; the clock handles that for you."

[explain_back]
prompt = "Describe gold's trading week and its three sessions. Why does the time of day matter?"
must_include = ["gold trades from Sunday evening to Friday evening New York time, with a short daily break", "the Asian, London and New York sessions follow one another around the world", "activity is not the same all day: London and New York are the busiest centres"]
misconceptions = ["gold is equally active every hour it is open", "gold trades over the weekend"]

[[explain_back.fixtures]]
answer = "Gold is open from Sunday 6 pm to Friday 5 pm New York time, with a one-hour break every weekday at 5 pm. The day passes from the Asian session in Tokyo to London and then New York. It's not equally busy all day: London and New York are the biggest centres, so most of the action happens while they trade."
expect = "got_it"

[[explain_back.fixtures]]
answer = "Gold trades nearly 24 hours a day through the Asian, London and New York sessions, one after the other."
expect = "almost"

[[explain_back.fixtures]]
answer = "Gold trades 24/7, weekends included, and every hour is the same, so it doesn't matter when you trade."
expect = "almost_or_not_yet"
+++
