//! Course map shown in the app. Lesson *content* moves to `crates/content` in Phase 6;
//! the Lesson 1 draft here exists so the reader, quiz and gate can be checked visually.

use gold_core::course::LessonId::{self, Advanced, Indicators, Numbered};

pub struct Section {
    pub eyebrow: &'static str,
    pub title: &'static str,
    pub blurb: &'static str,
    pub lessons: &'static [(LessonId, &'static str)],
    pub locked_hint: &'static str,
    /// Suggested pace (CURRICULUM), shown as guidance, never a deadline.
    pub pace: &'static str,
}

const READINESS_HINT: &str = "Unlocks with Live Readiness: 50 disciplined Demo Trades over at least 8 weeks.";

pub const SECTIONS: &[Section] = &[
    Section {
        eyebrow: "Stage 1",
        title: "Foundations",
        blurb: "What trading is, how gold is quoted, what a trade really costs, and your demo account.",
        lessons: &[
            (Numbered(1), "What is trading?"),
            (Numbered(2), "What is gold, and why does the world want it?"),
            (Numbered(3), "Reading \u{201c}XAUUSD\u{201d}"),
            (Numbered(4), "Buying vs selling"),
            (Numbered(5), "Bid, ask & spread"),
            (Numbered(6), "Dollar moves, points and pips"),
            (Numbered(7), "Lots: how big is your trade?"),
            (Numbered(8), "Leverage & margin"),
            (Numbered(9), "Order types, stop-loss & take-profit"),
            (Numbered(10), "The real costs: spread, commission, slippage, swap"),
            (Numbered(11), "Your demo account & platforms"),
        ],
        locked_hint: "",
        pace: "Suggested: weeks 1–2",
    },
    Section {
        eyebrow: "Stage 2",
        title: "Sessions & Timing",
        blurb: "When gold moves, when it sleeps, and the hours a beginner should avoid, in your own time zone.",
        lessons: &[
            (Numbered(12), "The trading week & sessions"),
            (Numbered(13), "The overlap, in your local time"),
            (Numbered(14), "Windows to avoid"),
        ],
        locked_hint: "Pass Stage 1 to unlock.",
        pace: "Suggested: weeks 2–3",
    },
    Section {
        eyebrow: "Stage 3",
        title: "What Moves Gold",
        blurb: "The dollar, real yields, the big US news days and fear: the forces behind gold's biggest moves.",
        lessons: &[
            (Numbered(15), "The US dollar"),
            (Numbered(16), "Real yields & the Fed"),
            (Numbered(17), "Tier-1 news: CPI, NFP, FOMC"),
            (Numbered(18), "Surprise vs forecast"),
            (Numbered(19), "Fear, war & central banks"),
        ],
        locked_hint: "Pass Stage 2 to unlock.",
        pace: "Suggested: weeks 3–5",
    },
    Section {
        eyebrow: "Stage 4",
        title: "Reading the Chart",
        blurb: "Candles, timeframes, and a five-step way to read any gold chart before you look for a trade.",
        lessons: &[
            (Numbered(20), "Candlesticks"),
            (Numbered(21), "Timeframes & top-down analysis"),
            (Numbered(22), "The five-step chart read"),
        ],
        locked_hint: "Pass Stage 3 to unlock.",
        pace: "Suggested: weeks 5–7",
    },
    Section {
        eyebrow: "Stage 5",
        title: "Price Action",
        blurb: "Zones, structure and confirmation candles: the core skill, and the way your friend trades.",
        lessons: &[
            (Numbered(23), "Support & resistance zones"),
            (Numbered(24), "Market structure: BOS & CHoCH"),
            (Numbered(25), "Confirmation candles"),
            (Numbered(26), "The pullback play"),
            (Numbered(27), "The bounce play"),
            (Indicators, "Indicators, lightly"),
        ],
        locked_hint: "Pass Stage 4 to unlock.",
        pace: "Suggested: weeks 7–10",
    },
    Section {
        eyebrow: "Stage 6",
        title: "Fibonacci & Confluence",
        blurb: "Where pullbacks tend to pause, and why a level counts more when several reasons line up.",
        lessons: &[
            (Numbered(28), "Drawing Fibonacci correctly"),
            (Numbered(29), "What the levels mean"),
            (Numbered(30), "Confluence"),
        ],
        locked_hint: "Pass Stage 5 to unlock.",
        pace: "Suggested: weeks 10–12",
    },
    Section {
        eyebrow: "Stage 7",
        title: "Risk & Psychology",
        blurb: "Stops, sizing, reward-to-risk, a daily loss limit, and the difference between a bad trade and a losing one.",
        lessons: &[
            (Numbered(31), "Stops beyond structure, sized by ATR"),
            (Numbered(32), "Position sizing"),
            (Numbered(33), "Risk-to-reward & expectancy"),
            (Numbered(34), "Daily loss limit & drawdown"),
            (Numbered(35), "Bad trade vs losing trade"),
        ],
        locked_hint: "Pass Stage 6 to unlock.",
        pace: "Suggested: weeks 12–16",
    },
    Section {
        eyebrow: "Stage 8",
        title: "Plan & Practise",
        blurb: "The Classic Price Action setup end to end, your daily routine, your written plan, and your first Demo Trade.",
        lessons: &[
            (Numbered(36), "The Classic Price Action setup"),
            (Numbered(37), "The pre-session routine"),
            (Numbered(38), "Writing your Trading Plan"),
            (Numbered(39), "Your first Demo Trade & weekly review"),
        ],
        locked_hint: "Pass Stage 7 to unlock.",
        pace: "Suggested: week 16 on",
    },
    Section {
        eyebrow: "Live Track",
        title: "Going Live",
        blurb: "Reviewing your journal, avoiding scams, starting small with real money, and when to size up.",
        lessons: &[
            (Numbered(40), "Reviewing your journal"),
            (Numbered(41), "Brokers & scams"),
            (Numbered(42), "Going live, small"),
            (Numbered(43), "Scaling up"),
        ],
        locked_hint: READINESS_HINT,
        pace: "",
    },
    Section {
        eyebrow: "Advanced Track",
        title: "More Strategies",
        blurb: "Breakouts, the Asian range, news follow-through, Supply & Demand and SMC, all built on Classic Price Action.",
        lessons: &[
            (Advanced(1), "Breakout & retest"),
            (Advanced(2), "Asian-range breakout"),
            (Advanced(3), "News follow-through"),
            (Advanced(4), "Supply & demand zones"),
            (Advanced(5), "Fresh vs tested zones"),
            (Advanced(6), "Zones with Classic structure"),
            (Advanced(7), "Liquidity & sweeps"),
            (Advanced(8), "Order blocks"),
            (Advanced(9), "Fair value gaps"),
            (Advanced(10), "Your SMC rulebook"),
        ],
        locked_hint: "Unlocks with Live Readiness.",
        pace: "",
    },
];

pub fn title(id: LessonId) -> &'static str {
    SECTIONS.iter().flat_map(|s| s.lessons.iter()).find(|(l, _)| *l == id).map_or("Lesson", |(_, t)| t)
}

pub fn label(id: LessonId) -> String {
    match id {
        Numbered(n) => n.to_string(),
        Advanced(n) => format!("A{n}"),
        Indicators => "+".into(),
    }
}

pub fn eyebrow(id: LessonId) -> String {
    match (id, id.stage()) {
        (Numbered(n), Some(s)) => format!("Stage {s} · Lesson {n}"),
        (Numbered(n), None) => format!("Live Track · Lesson {n}"),
        (Advanced(n), _) => format!("Advanced Track · A{n}"),
        (Indicators, _) => "Side lesson · Optional".into(),
    }
}

pub struct VideoRef {
    pub title: &'static str,
    pub channel: &'static str,
    pub url: &'static str,
    pub watch_for: &'static str,
}

pub struct QuizQ {
    pub q: &'static str,
    pub options: &'static [&'static str],
    pub answer: usize,
    pub why: &'static str,
}

pub struct LessonDraft {
    pub objective: &'static str,
    pub hook: &'static str,
    pub body: &'static [&'static str],
    pub analogy: &'static str,
    pub example: &'static str,
    pub mistake: &'static str,
    pub takeaways: &'static [&'static str],
    pub videos: &'static [VideoRef],
    pub quiz: &'static [QuizQ],
    pub explain_prompt: &'static str,
}

pub fn draft(id: LessonId) -> Option<&'static LessonDraft> {
    (id == Numbered(1)).then_some(&LESSON_1)
}

static LESSON_1: LessonDraft = LessonDraft {
    objective: "explain what a trader is actually trying to profit from.",
    hook: "If you buy a gold bar and keep it in a drawer for ten years, are you trading?",
    body: &[
        "Not quite. Trading means buying something at one price and selling it at another, aiming for the difference to land in your favour. The thing itself matters less than how its price moves.",
        "You don't need to own a gold bar to trade gold. When you trade XAUUSD with a broker, no metal changes hands: you open a position at one price, close it at another, and the difference is settled in money. What you are really trading is the price.",
        "That difference can go either way. If the price moves against you, the difference is a loss, and it is real money leaving your account.",
        "Trading is also different from investing. An investor buys and holds for years. A trader is in and out over minutes, hours or days, aiming to profit from shorter moves.",
    ],
    analogy: "Think of reselling concert tickets. You buy a ticket for $50 before the band announces it's their final tour. Demand jumps, and you sell it for $80. You never went to the concert: your $30 came from the change in price. If the show had been cancelled, the price could just as easily have fallen.",
    example: "Gold is at 4,100.00 and you buy. Later it's at 4,120.00 and you sell. Gold moved $20 in your favour, and that $20 move is where your profit comes from. Had it fallen to 4,080.00 instead, the same $20 move would have been a loss. (Illustrative prices.)",
    mistake: "Thinking trading means owning gold for the long run. That's investing. Trading is about the change in price, and every trade can lose as well as win.",
    takeaways: &[
        "A trader profits from a change in price.",
        "You buy and sell at different prices; the gap is your profit or loss.",
        "You don't need to own physical gold to trade it.",
        "Price can move against you, so every trade can lose.",
    ],
    videos: &[VideoRef {
        title: "Search: what is trading, for beginners",
        channel: "YouTube search",
        url: "https://www.youtube.com/results?search_query=what+is+trading+for+beginners",
        watch_for: "a placeholder. Verified videos from educational channels are added in Phase 6.",
    }],
    quiz: &[
        QuizQ {
            q: "What is a trader trying to profit from?",
            options: &["Owning gold for many years", "A change in price", "Interest paid by the broker", "Storing gold safely"],
            answer: 1,
            why: "Trading profits from the difference between the price you buy at and the price you sell at.",
        },
        QuizQ {
            q: "You buy gold at 4,100.00 and sell at 4,115.00. How big was the move?",
            options: &["$1.50", "$15", "$150", "$1,500"],
            answer: 1,
            why: "4,115 − 4,100 = $15. That's the Dollar Move, and it was in your favour.",
        },
        QuizQ {
            q: "You buy at 4,140.00 and sell at 4,130.00. What happened?",
            options: &["A $10 profit", "A $10 loss", "Nothing: the price came back", "A $100 profit"],
            answer: 1,
            why: "Price fell $10 after you bought, so the move went against you.",
        },
        QuizQ {
            q: "Do you need to own a physical gold bar to trade XAUUSD?",
            options: &["Yes, the broker posts it to you", "Yes, it's held in a vault in your name", "No, you trade the price", "Only for large trades"],
            answer: 2,
            why: "Online gold trading settles the price difference in money; no bar changes hands.",
        },
        QuizQ {
            q: "Which of these is trading rather than investing?",
            options: &["Buying gold coins for retirement", "Buying in the morning and selling that afternoon after a $12 rise", "Inheriting gold jewellery", "Keeping gold in a safe for 20 years"],
            answer: 1,
            why: "Trading aims to profit from shorter price moves; investing holds for the long term.",
        },
    ],
    explain_prompt: "In your own words: what is a trader actually trying to profit from?",
};
