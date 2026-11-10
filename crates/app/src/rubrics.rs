//! Explain-Back rubrics for Stage 1 (from CURRICULUM.md) plus the three fixture answers per lesson
//! used to tune them (IMPLEMENTATION-PLAN Phase 5). Moves into `crates/content` in Phase 6.

use gold_core::course::LessonId;
use gold_core::gate::Grade;
use gold_core::grading::GradingInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expect {
    GotIt,
    Almost,
    /// A misconception answer may land on either; it must never be Got it.
    AlmostOrNotYet,
}

impl Expect {
    pub fn accepts(self, g: Grade) -> bool {
        match self {
            Expect::GotIt => g == Grade::GotIt,
            Expect::Almost => g == Grade::Almost,
            Expect::AlmostOrNotYet => g != Grade::GotIt,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Expect::GotIt => "Got it",
            Expect::Almost => "Almost",
            Expect::AlmostOrNotYet => "Almost / Not yet",
        }
    }
}

pub struct Rubric {
    pub lesson: u8,
    pub title: &'static str,
    pub objective: &'static str,
    pub analogy: &'static str,
    pub prompt: &'static str,
    pub must_include: &'static [&'static str],
    pub misconceptions: &'static [&'static str],
    /// (answer, expected): correct, gap, misconception.
    pub fixtures: [(&'static str, Expect); 3],
}

impl Rubric {
    pub fn input<'a>(&'a self, answer: &'a str) -> GradingInput<'a> {
        GradingInput {
            title: self.title,
            objective: self.objective,
            prompt: self.prompt,
            analogy: self.analogy,
            must_include: self.must_include,
            misconceptions: self.misconceptions,
            answer,
        }
    }
}

pub fn rubric(id: LessonId) -> Option<&'static Rubric> {
    match id {
        LessonId::Numbered(n) => STAGE_1.iter().find(|r| r.lesson == n),
        _ => None,
    }
}

use Expect::*;

pub static STAGE_1: [Rubric; 11] = [
    Rubric {
        lesson: 1,
        title: "What is trading?",
        objective: "explain trading as profiting from a change in price, not from owning the thing",
        analogy: "Reselling concert tickets: buy before demand rises, sell after",
        prompt: "What is a trader actually trying to profit from?",
        must_include: &["profit comes from the change in price", "you buy and sell at different prices", "a trade can lose as well as gain"],
        misconceptions: &["trading is the same as investing for the long term", "you need to own physical gold to trade it"],
        fixtures: [
            ("A trader tries to make money from the price changing. You buy at one price and sell at another, and the difference is your profit, but if the price goes the wrong way you lose money instead.", GotIt),
            ("They want to profit from the price changing, buying at one price and selling at a higher one.", Almost),
            ("A trader buys gold bars and keeps them safe for many years so they are worth more when they retire.", AlmostOrNotYet),
        ],
    },
    Rubric {
        lesson: 2,
        title: "What is gold, and why does the world want it?",
        objective: "name three reasons people buy gold and connect them to price",
        analogy: "Gold as the world's emergency savings jar",
        prompt: "Why do people rush to buy gold when they're scared?",
        must_include: &["gold is seen as holding its value over time", "it is a safe haven in fear or crisis", "more demand pushes the price up"],
        misconceptions: &["gold only ever goes up", "gold pays interest or dividends"],
        fixtures: [
            ("Because gold has kept its value for a very long time, people treat it as a safe place for money when things feel risky. When lots of people want it at once, the extra demand pushes the price up.", GotIt),
            ("When people are scared they want somewhere safe to put their money, and gold is seen as safe.", Almost),
            ("They buy gold because it always goes up in value and it pays them interest while they hold it.", AlmostOrNotYet),
        ],
    },
    Rubric {
        lesson: 3,
        title: "Reading \"XAUUSD\"",
        objective: "read the symbol and the quote as US dollars per troy ounce",
        analogy: "A price tag in a shop: one ounce of gold costs $X",
        prompt: "What does XAUUSD = 4,150 mean, and what does it mean when it rises?",
        must_include: &["XAU is gold and USD is the US dollar", "the number is the price of one troy ounce in dollars", "a rise means gold costs more dollars"],
        misconceptions: &["the number is the price of a gram or a kilo", "XAU is a company or a stock"],
        fixtures: [
            ("XAU means gold and USD means US dollars, so 4,150 means one troy ounce of gold costs $4,150. If it rises to 4,155, gold has become $5 more expensive in dollars.", GotIt),
            ("It means gold costs 4,150 US dollars, and if the number goes up gold is more expensive.", Almost),
            ("XAUUSD is the share price of a big gold mining company, and 4,150 is what one share costs.", AlmostOrNotYet),
        ],
    },
    Rubric {
        lesson: 4,
        title: "Buying vs selling",
        objective: "explain going long and going short, and when each one profits",
        analogy: "Long = buying an umbrella before the rain; short = selling a borrowed umbrella now and buying it back cheaper later",
        prompt: "How can you make money when gold's price falls?",
        must_include: &["you sell first (go short) and buy back later", "you profit if the price falls", "you lose if the price rises instead"],
        misconceptions: &["you must own gold before you can sell it", "selling only means closing a buy"],
        fixtures: [
            ("You go short: you sell first at the current price and buy back later. If gold falls you buy back cheaper and keep the difference, but if it rises you have to buy back higher and you lose.", GotIt),
            ("You sell first and then buy it back later at a lower price, keeping the difference.", Almost),
            ("You can't really. You first have to buy gold and own it, and selling just means getting out of that buy.", AlmostOrNotYet),
        ],
    },
    Rubric {
        lesson: 5,
        title: "Bid, ask & spread",
        objective: "explain why every trade starts slightly negative and what the spread costs",
        analogy: "An airport currency booth: \"We buy at\" and \"We sell at\"",
        prompt: "Why do you start every trade slightly \"in the red\"?",
        must_include: &["you buy at the ask and sell at the bid", "the ask is higher than the bid", "the spread is a cost you pay when you enter"],
        misconceptions: &["the spread is only charged when you close", "the spread is always the same size"],
        fixtures: [
            ("Because you buy at the higher ask price but your position is valued at the lower bid. The gap between them is the spread, which is a cost you pay as soon as you open the trade.", GotIt),
            ("Because of the spread: there are two prices and the gap between them is a cost.", Almost),
            ("Because the broker charges you the spread at the end when you close the trade, and it's always exactly the same amount.", AlmostOrNotYet),
        ],
    },
    Rubric {
        lesson: 6,
        title: "Dollar moves, points and pips",
        objective: "measure any move in dollars and convert other people's \"pips\"",
        analogy: "A ruler with centimetres, millimetres and metres: the same distance, different numbers",
        prompt: "A YouTuber says \"50-pip stop on gold.\" What do you do before trusting that?",
        must_include: &["convert it into a dollar move", "pip size differs between sources", "in this course a point is $0.01 and a pip is $0.10"],
        misconceptions: &["a pip means the same everywhere", "pips are an amount of money"],
        fixtures: [
            ("I'd turn it into a dollar move first, because people use different pip sizes for gold. In this course a pip is $0.10 and a point is $0.01, so if they mean that, 50 pips is a $5 stop, but they might mean something else.", GotIt),
            ("I'd convert it into dollars, because 50 pips could mean different things depending on who's talking.", Almost),
            ("Nothing, 50 pips is 50 pips on every chart. It means you'd lose $50.", AlmostOrNotYet),
        ],
    },
    Rubric {
        lesson: 7,
        title: "Lots: how big is your trade?",
        objective: "turn a lot size and a dollar move into money",
        analogy: "Buying in boxes: 1 lot is a box of 100 ounces",
        prompt: "You buy 0.20 lots and gold rises $4. How much did you make, and why?",
        must_include: &["1 lot is usually 100 ounces (check the broker)", "money = lots × 100 × dollar move", "so 0.20 lots × $4 = $80"],
        misconceptions: &["a lot is a fixed dollar amount", "profit depends only on the price move, not the size"],
        fixtures: [
            ("A standard lot is 100 ounces, though you should check your broker, so 0.20 lots is 20 ounces. Each ounce made $4, so that's 0.20 × 100 × $4 = $80.", GotIt),
            ("$80, because 0.20 lots times $4 times 100 is 80.", Almost),
            ("$4, because the price went up $4 and that's what you make no matter how many lots.", AlmostOrNotYet),
        ],
    },
    Rubric {
        lesson: 8,
        title: "Leverage & margin",
        objective: "explain margin as a deposit and why leverage magnifies losses as much as gains",
        analogy: "Buying a house with a mortgage: a small deposit controls a large asset, and losses hit your deposit",
        prompt: "Why is leverage dangerous for a beginner even though it \"lets you trade bigger\"?",
        must_include: &["margin is a deposit, not the cost of the trade", "leverage magnifies losses as much as gains", "it can wipe out an account fast", "position size, not leverage, sets your risk"],
        misconceptions: &["leverage is free money", "more leverage means more profit"],
        fixtures: [
            ("Margin is just a deposit that lets you control a much bigger position. Because the position is big, a small move against you is a big loss compared with your deposit, just as wins are bigger, so an account can be wiped out quickly. What really sets your risk is your position size, not the leverage number.", GotIt),
            ("Because leverage makes your losses as big as your wins, and a small move against you can wipe out your account.", Almost),
            ("It isn't dangerous really. More leverage means more profit, it's like the broker giving you extra money to trade with.", AlmostOrNotYet),
        ],
    },
    Rubric {
        lesson: 9,
        title: "Order types, stop-loss & take-profit",
        objective: "choose a market, limit or stop order for a given intent, with a stop-loss and take-profit attached",
        analogy: "A shop: pay at the till now (market); leave a written offer for a lower price (limit); \"buy only if it breaks above this\" (stop)",
        prompt: "You only want to buy if gold dips to 4,080. Which order do you use, and why not a market order?",
        must_include: &["use a buy limit order at 4,080", "a market order fills immediately at the current price", "a limit order waits and may never fill", "a stop-loss and take-profit close the trade automatically"],
        misconceptions: &["a limit order always fills", "a stop-loss guarantees the exact exit price"],
        fixtures: [
            ("A buy limit at 4,080. A market order would buy right now at today's higher price, while a limit waits for the dip and might never fill if gold doesn't get there. I'd attach a stop-loss and take-profit so it closes itself either way.", GotIt),
            ("A buy limit at 4,080, because a market order would buy immediately at the current price instead of waiting.", Almost),
            ("A buy stop at 4,080, and it's guaranteed to fill at exactly 4,080 so there's no risk of a worse price.", AlmostOrNotYet),
        ],
    },
    Rubric {
        lesson: 10,
        title: "The real costs: spread, commission, slippage, swap",
        objective: "name all four trading costs and estimate them on a trade",
        analogy: "A taxi ride: flag fall (spread), booking fee (commission), a traffic surprise (slippage), overnight parking (swap)",
        prompt: "Your trade moved $5 your way, but your account grew less than you expected. What could explain it?",
        must_include: &["the spread paid on entry", "commission per lot, depending on the broker", "slippage when price jumps past your order", "swap charged for each night the trade is held"],
        misconceptions: &["costs only matter for big accounts", "swap is a one-off fee", "stops always fill exactly at their price"],
        fixtures: [
            ("Costs ate into it: the spread you pay when you open, any commission per lot, slippage if the price jumped past my order, and swap if I held it overnight, which is charged every night.", GotIt),
            ("The spread and the broker's commission would take some of the profit.", Almost),
            ("Probably a one-off swap fee the broker takes, but costs like that only really matter for big accounts anyway.", AlmostOrNotYet),
        ],
    },
    Rubric {
        lesson: 11,
        title: "Your demo account & platforms",
        objective: "open an MT5 demo, read XAUUSD's contract spec, and place and close a 0.01-lot order with a stop-loss and take-profit",
        analogy: "A flight simulator: real instruments, no real crash",
        prompt: "Why do you practise on a demo first, and what must you check in your broker's XAUUSD spec?",
        must_include: &["a demo uses real prices with simulated money", "check contract size, minimum lot, swap and trading hours", "always attach a stop", "use a regulated broker"],
        misconceptions: &["demo results guarantee live results", "every broker's gold contract is the same"],
        fixtures: [
            ("A demo has real market prices but fake money, so mistakes are free while I learn. In the XAUUSD spec I'd check the contract size, the minimum lot, the swap rates and the trading hours, always put a stop on every order, and make sure the broker is regulated.", GotIt),
            ("Because it's real prices with pretend money, so I can practise safely. I'd check how many ounces are in a lot.", Almost),
            ("So I can prove I'm profitable, because if it works on demo it will definitely work live. The specs don't matter since gold is the same at every broker.", AlmostOrNotYet),
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_stage_one_lesson_has_a_rubric() {
        for n in 1..=11 {
            let r = rubric(LessonId::Numbered(n)).expect("rubric");
            assert!((2..=4).contains(&r.must_include.len()), "lesson {n}: 2–4 key ideas (CURRICULUM rule 4)");
            assert!((1..=3).contains(&r.misconceptions.len()), "lesson {n}: 1–3 misconceptions");
            assert_eq!(r.fixtures.map(|f| f.1), [GotIt, Almost, AlmostOrNotYet], "lesson {n}: fixture order");
        }
        assert!(rubric(LessonId::Numbered(12)).is_none());
    }

    #[test]
    fn expectations() {
        assert!(AlmostOrNotYet.accepts(Grade::NotYet) && AlmostOrNotYet.accepts(Grade::Almost) && !AlmostOrNotYet.accepts(Grade::GotIt));
        assert!(GotIt.accepts(Grade::GotIt) && !Almost.accepts(Grade::GotIt));
    }
}
