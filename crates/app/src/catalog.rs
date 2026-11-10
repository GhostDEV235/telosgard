//! Course map shown in the app. Lesson *content* moves to `crates/content` in Phase 6;
//! the Lesson 1 draft here exists so the reader, quiz and gate can be checked visually.

use gold_core::course::LessonId::{self, Advanced, Indicators, Numbered};

pub struct Section {
    pub eyebrow: &'static str,
    pub title: &'static str,
    pub blurb: &'static str,
    pub lessons: &'static [(LessonId, &'static str)],
    pub locked_hint: &'static str,
}

pub const SECTIONS: &[Section] = &[
    Section {
        eyebrow: "Stage 1",
        title: "Foundations",
        blurb: "What trading is, what gold is, and how a gold price is quoted, bought and measured.",
        lessons: &[
            (Numbered(1), "What is trading?"),
            (Numbered(2), "What is gold, and why does the world want it?"),
            (Numbered(3), "Reading \u{201c}XAUUSD\u{201d}"),
            (Numbered(4), "Buying vs selling"),
            (Numbered(5), "Bid, ask & spread"),
            (Numbered(6), "Dollar moves, points and pips"),
            (Numbered(7), "Lots: how big is your trade?"),
            (Numbered(8), "Leverage & margin"),
            (Numbered(9), "Market hours & sessions"),
        ],
        locked_hint: "",
    },
    Section {
        eyebrow: "Stage 2",
        title: "What Moves Gold",
        blurb: "The dollar, interest rates, news days and fear: the forces behind gold's big moves.",
        lessons: &[
            (Numbered(10), "The US dollar"),
            (Numbered(11), "Interest rates and the Fed"),
            (Numbered(12), "News events"),
            (Numbered(13), "Fear, war and central banks"),
        ],
        locked_hint: "Pass Stage 1 to unlock.",
    },
    Section {
        eyebrow: "Stage 3",
        title: "Reading Price Action",
        blurb: "Candles, timeframes, levels and structure: reading a gold chart the way your friend does.",
        lessons: &[
            (Numbered(14), "Candlesticks"),
            (Numbered(15), "Timeframes"),
            (Numbered(16), "Support & resistance"),
            (Numbered(17), "Market structure"),
            (Numbered(18), "Confirmation candles"),
            (Indicators, "Indicators, lightly"),
        ],
        locked_hint: "Pass Stage 2 to unlock.",
    },
    Section {
        eyebrow: "Stage 4",
        title: "Risk",
        blurb: "Stops, position size, reward-to-risk and drawdown: how to stay in the game.",
        lessons: &[
            (Numbered(19), "Stop-loss & take-profit"),
            (Numbered(20), "Position sizing"),
            (Numbered(21), "Risk-to-reward & win rate"),
            (Numbered(22), "Drawdown"),
        ],
        locked_hint: "Pass Stage 3 to unlock.",
    },
    Section {
        eyebrow: "Stage 5",
        title: "Plan & Practise",
        blurb: "The Classic Price Action setup, your written Trading Plan, and your first Demo Trade.",
        lessons: &[
            (Numbered(23), "The Classic Price Action setup"),
            (Numbered(24), "Writing your Trading Plan"),
            (Numbered(25), "Your first Demo Trade"),
        ],
        locked_hint: "Pass Stage 4 to unlock.",
    },
    Section {
        eyebrow: "Live Track",
        title: "Going Live",
        blurb: "Psychology, reviewing your journal, avoiding scams, and starting small with real money.",
        lessons: &[
            (Numbered(26), "Trading psychology"),
            (Numbered(27), "Reviewing your journal"),
            (Numbered(28), "Brokers & scams"),
            (Numbered(29), "Going live, small"),
        ],
        locked_hint: "Unlocks with Live Readiness: 20 disciplined Demo Trades over 3 weeks.",
    },
    Section {
        eyebrow: "Advanced Track",
        title: "Supply & Demand, then SMC",
        blurb: "Zones, liquidity, order blocks and fair value gaps, layered on top of Classic Price Action.",
        lessons: &[
            (Advanced(1), "Supply & demand zones"),
            (Advanced(2), "Fresh vs tested zones"),
            (Advanced(3), "Zones with Classic structure"),
            (Advanced(4), "Liquidity"),
            (Advanced(5), "Order blocks"),
            (Advanced(6), "Fair value gaps"),
            (Advanced(7), "Your SMC rulebook"),
        ],
        locked_hint: "Unlocks with Live Readiness.",
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
    example: "Gold is at 2,600.00 and you buy. Later it's at 2,620.00 and you sell. Gold moved $20 in your favour, and that $20 move is where your profit comes from. Had it fallen to 2,580.00 instead, the same $20 move would have been a loss. (Illustrative prices.)",
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
            q: "You buy gold at 2,600.00 and sell at 2,615.00. How big was the move?",
            options: &["$1.50", "$15", "$150", "$1,500"],
            answer: 1,
            why: "2,615 − 2,600 = $15. That's the Dollar Move, and it was in your favour.",
        },
        QuizQ {
            q: "You buy at 2,640.00 and sell at 2,630.00. What happened?",
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
