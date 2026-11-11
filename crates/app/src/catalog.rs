//! Course map shown in the app: stage cards and lesson titles, including lessons not yet written.
//! Written lessons live in `crates/content`.

use telosgard_core::course::LessonId::{self, Advanced, Indicators, Numbered};

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
            (Numbered(2), "The honest maths of trading"),
            (Numbered(3), "What is gold, and why does the world want it?"),
            (Numbered(4), "Reading \u{201c}XAUUSD\u{201d}"),
            (Numbered(5), "Buying vs selling"),
            (Numbered(6), "Bid, ask & spread"),
            (Numbered(7), "Dollar moves, points and pips"),
            (Numbered(8), "Lots: how big is your trade?"),
            (Numbered(9), "Leverage & margin"),
            (Numbered(10), "Order types, stop-loss & take-profit"),
            (Numbered(11), "The real costs: spread, commission, slippage, swap"),
            (Numbered(12), "Your demo account & platforms"),
        ],
        locked_hint: "",
        pace: "Suggested: weeks 1–2",
    },
    Section {
        eyebrow: "Stage 2",
        title: "Sessions & Timing",
        blurb: "When gold moves, when it sleeps, and the hours a beginner should avoid, in your own time zone.",
        lessons: &[
            (Numbered(13), "The trading week & sessions"),
            (Numbered(14), "The overlap, in your local time"),
            (Numbered(15), "Windows to avoid"),
        ],
        locked_hint: "Pass Stage 1 to unlock.",
        pace: "Suggested: weeks 2–3",
    },
    Section {
        eyebrow: "Stage 3",
        title: "What Moves Gold",
        blurb: "The dollar, real yields, the big US news days and fear: the forces behind gold's biggest moves.",
        lessons: &[
            (Numbered(16), "The US dollar"),
            (Numbered(17), "Real yields & the Fed"),
            (Numbered(18), "Tier-1 news: CPI, NFP, FOMC"),
            (Numbered(19), "Surprise vs forecast"),
            (Numbered(20), "Fear, war & central banks"),
        ],
        locked_hint: "Pass Stage 2 to unlock.",
        pace: "Suggested: weeks 3–5",
    },
    Section {
        eyebrow: "Stage 4",
        title: "Reading the Chart",
        blurb: "Candles, timeframes, and a five-step way to read any gold chart before you look for a trade.",
        lessons: &[
            (Numbered(21), "Candlesticks"),
            (Numbered(22), "Timeframes & top-down analysis"),
            (Numbered(23), "The five-step chart read"),
        ],
        locked_hint: "Pass Stage 3 to unlock.",
        pace: "Suggested: weeks 5–7",
    },
    Section {
        eyebrow: "Stage 5",
        title: "Price Action",
        blurb: "Zones, structure and confirmation candles: the core skill, and the way your friend trades.",
        lessons: &[
            (Numbered(24), "Support & resistance zones"),
            (Numbered(25), "Market structure: BOS & CHoCH"),
            (Numbered(26), "Confirmation candles"),
            (Numbered(27), "The pullback play"),
            (Numbered(28), "The bounce play"),
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
            (Numbered(29), "Drawing Fibonacci correctly"),
            (Numbered(30), "What the levels mean"),
            (Numbered(31), "Confluence"),
        ],
        locked_hint: "Pass Stage 5 to unlock.",
        pace: "Suggested: weeks 10–12",
    },
    Section {
        eyebrow: "Stage 7",
        title: "Risk & Psychology",
        blurb: "Stops, sizing, reward-to-risk, a daily loss limit, and the difference between a bad trade and a losing one.",
        lessons: &[
            (Numbered(32), "Stops beyond structure, sized by ATR"),
            (Numbered(33), "Position sizing"),
            (Numbered(34), "Risk-to-reward & expectancy"),
            (Numbered(35), "Daily loss limit & drawdown"),
            (Numbered(36), "Bad trade vs losing trade"),
        ],
        locked_hint: "Pass Stage 6 to unlock.",
        pace: "Suggested: weeks 12–16",
    },
    Section {
        eyebrow: "Stage 8",
        title: "Plan & Practise",
        blurb: "The Classic Price Action setup end to end, your daily routine, your written plan, and your first Demo Trade.",
        lessons: &[
            (Numbered(37), "The Classic Price Action setup"),
            (Numbered(38), "The pre-session routine"),
            (Numbered(39), "Writing your Trading Plan"),
            (Numbered(40), "Your first Demo Trade & weekly review"),
        ],
        locked_hint: "Pass Stage 7 to unlock.",
        pace: "Suggested: week 16 on",
    },
    Section {
        eyebrow: "Live Track",
        title: "Going Live",
        blurb: "Reviewing your journal, avoiding scams, starting small with real money, and when to size up.",
        lessons: &[
            (Numbered(41), "Reviewing your journal"),
            (Numbered(42), "Brokers & scams"),
            (Numbered(43), "Going live, small"),
            (Numbered(44), "Scaling up"),
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
