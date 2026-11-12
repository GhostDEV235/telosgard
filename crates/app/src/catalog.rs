//! Course map shown in the app: stage cards and lesson titles, including lessons not yet written.
//! Written lessons live in `crates/content`. Every text is English and French (docs/FRENCH.md).

use crate::i18n::t;
use telosgard_core::course::LessonId::{self, Advanced, Indicators, Numbered};

/// A text in English and French; `.get()` picks the current language.
#[derive(Clone, Copy)]
pub struct Tx(pub &'static str, pub &'static str);

impl Tx {
    pub fn get(self) -> &'static str {
        t(self.0, self.1)
    }
}

pub struct Section {
    pub eyebrow: Tx,
    pub title: Tx,
    pub blurb: Tx,
    pub lessons: &'static [(LessonId, Tx)],
    pub locked_hint: Tx,
    /// Suggested pace (CURRICULUM), shown as guidance, never a deadline.
    pub pace: Tx,
}

const READINESS_HINT: Tx = Tx(
    "Unlocks with Live Readiness: 50 disciplined Demo Trades over at least 8 weeks.",
    "Se débloque avec « Prêt pour le réel » : 50 Trades démo disciplinés sur au moins 8 semaines.",
);
const NONE: Tx = Tx("", "");

pub const SECTIONS: &[Section] = &[
    Section {
        eyebrow: Tx("Stage 1", "Étape 1"),
        title: Tx("Foundations", "Les bases"),
        blurb: Tx("What trading is, how gold is quoted, what a trade really costs, and your demo account.", "Ce qu’est le trading, comment l’or est coté, ce que coûte vraiment un trade, et ton compte démo."),
        lessons: &[
            (Numbered(1), Tx("What is trading?", "Qu’est-ce que le trading ?")),
            (Numbered(2), Tx("The honest maths of trading", "Les vrais calculs du trading")),
            (Numbered(3), Tx("What is gold, and why does the world want it?", "Qu’est-ce que l’or, et pourquoi le monde en veut-il ?")),
            (Numbered(4), Tx("Reading \u{201c}XAUUSD\u{201d}", "Lire « XAUUSD »")),
            (Numbered(5), Tx("Buying vs selling", "Acheter ou vendre")),
            (Numbered(6), Tx("Bid, ask & spread", "Bid, ask et spread")),
            (Numbered(7), Tx("Dollar moves, points and pips", "Mouvements en dollars, points et pips")),
            (Numbered(8), Tx("Lots: how big is your trade?", "Les lots : quelle est la taille de ton trade ?")),
            (Numbered(9), Tx("Leverage & margin", "Effet de levier et marge")),
            (Numbered(10), Tx("Order types, stop-loss & take-profit", "Types d’ordres, stop-loss et take-profit")),
            (Numbered(11), Tx("The real costs: spread, commission, slippage, swap", "Les vrais coûts : spread, commission, slippage, swap")),
            (Numbered(12), Tx("Your demo account & platforms", "Ton compte démo et tes plateformes")),
        ],
        locked_hint: NONE,
        pace: Tx("Suggested: weeks 1–2", "Conseillé : semaines 1–2"),
    },
    Section {
        eyebrow: Tx("Stage 2", "Étape 2"),
        title: Tx("Sessions & Timing", "Séances et horaires"),
        blurb: Tx("When gold moves, when it sleeps, and the hours a beginner should avoid, in your own time zone.", "Quand l’or bouge, quand il dort, et les heures qu’un débutant doit éviter, dans ton propre fuseau horaire."),
        lessons: &[
            (Numbered(13), Tx("The trading week & sessions", "La semaine de trading et les séances")),
            (Numbered(14), Tx("The overlap, in your local time", "Le chevauchement, dans ton heure locale")),
            (Numbered(15), Tx("Windows to avoid", "Les fenêtres à éviter")),
        ],
        locked_hint: Tx("Pass Stage 1 to unlock.", "Réussis l’étape 1 pour débloquer."),
        pace: Tx("Suggested: weeks 2–3", "Conseillé : semaines 2–3"),
    },
    Section {
        eyebrow: Tx("Stage 3", "Étape 3"),
        title: Tx("What Moves Gold", "Ce qui fait bouger l’or"),
        blurb: Tx("The dollar, real yields, the big US news days and fear: the forces behind gold's biggest moves.", "Le dollar, les taux réels, les grandes annonces américaines et la peur : les forces derrière les plus gros mouvements de l’or."),
        lessons: &[
            (Numbered(16), Tx("The US dollar", "Le dollar américain")),
            (Numbered(17), Tx("Real yields & the Fed", "Les taux réels et la Fed")),
            (Numbered(18), Tx("Tier-1 news: CPI, NFP, FOMC", "Les grandes annonces : CPI, NFP, FOMC")),
            (Numbered(19), Tx("Surprise vs forecast", "Surprise ou prévision")),
            (Numbered(20), Tx("Fear, war & central banks", "La peur, la guerre et les banques centrales")),
        ],
        locked_hint: Tx("Pass Stage 2 to unlock.", "Réussis l’étape 2 pour débloquer."),
        pace: Tx("Suggested: weeks 3–5", "Conseillé : semaines 3–5"),
    },
    Section {
        eyebrow: Tx("Stage 4", "Étape 4"),
        title: Tx("Reading the Chart", "Lire le graphique"),
        blurb: Tx("Candles, timeframes, and a five-step way to read any gold chart before you look for a trade.", "Les bougies, les unités de temps, et une lecture en cinq étapes de n’importe quel graphique de l’or avant de chercher un trade."),
        lessons: &[
            (Numbered(21), Tx("Candlesticks", "Les chandeliers")),
            (Numbered(22), Tx("Timeframes & top-down analysis", "Unités de temps et analyse du haut vers le bas")),
            (Numbered(23), Tx("The five-step chart read", "La lecture du graphique en cinq étapes")),
        ],
        locked_hint: Tx("Pass Stage 3 to unlock.", "Réussis l’étape 3 pour débloquer."),
        pace: Tx("Suggested: weeks 5–7", "Conseillé : semaines 5–7"),
    },
    Section {
        eyebrow: Tx("Stage 5", "Étape 5"),
        title: Tx("Price Action", "Price action"),
        blurb: Tx("Zones, structure and confirmation candles: the core skill, and the way your friend trades.", "Zones, structure et bougies de confirmation : la compétence centrale, et la façon dont trade ton ami."),
        lessons: &[
            (Numbered(24), Tx("Support & resistance zones", "Zones de support et de résistance")),
            (Numbered(25), Tx("Market structure: BOS & CHoCH", "Structure du marché : BOS et CHoCH")),
            (Numbered(26), Tx("Confirmation candles", "Les bougies de confirmation")),
            (Numbered(27), Tx("The pullback play", "Le jeu du repli")),
            (Numbered(28), Tx("The bounce play", "Le jeu du rebond")),
            (Indicators, Tx("Indicators, lightly", "Les indicateurs, avec légèreté")),
        ],
        locked_hint: Tx("Pass Stage 4 to unlock.", "Réussis l’étape 4 pour débloquer."),
        pace: Tx("Suggested: weeks 7–10", "Conseillé : semaines 7–10"),
    },
    Section {
        eyebrow: Tx("Stage 6", "Étape 6"),
        title: Tx("Fibonacci & Confluence", "Fibonacci et confluence"),
        blurb: Tx("Where pullbacks tend to pause, and why a level counts more when several reasons line up.", "Où les replis ont tendance à marquer une pause, et pourquoi un niveau compte plus quand plusieurs raisons s’alignent."),
        lessons: &[
            (Numbered(29), Tx("Drawing Fibonacci correctly", "Bien tracer un Fibonacci")),
            (Numbered(30), Tx("What the levels mean", "Ce que veulent dire les niveaux")),
            (Numbered(31), Tx("Confluence", "La confluence")),
        ],
        locked_hint: Tx("Pass Stage 5 to unlock.", "Réussis l’étape 5 pour débloquer."),
        pace: Tx("Suggested: weeks 10–12", "Conseillé : semaines 10–12"),
    },
    Section {
        eyebrow: Tx("Stage 7", "Étape 7"),
        title: Tx("Risk & Psychology", "Risque et psychologie"),
        blurb: Tx("Stops, sizing, reward-to-risk, a daily loss limit, and the difference between a bad trade and a losing one.", "Les stops, la taille de position, le rapport gain/risque, une perte maximale du jour, et la différence entre un mauvais trade et un trade perdant."),
        lessons: &[
            (Numbered(32), Tx("Stops beyond structure, sized by ATR", "Des stops au-delà de la structure, dimensionnés par l’ATR")),
            (Numbered(33), Tx("Position sizing", "La taille de position")),
            (Numbered(34), Tx("Risk-to-reward & expectancy", "Rapport risque/gain et espérance")),
            (Numbered(35), Tx("Daily loss limit & drawdown", "Perte maximale du jour et drawdown")),
            (Numbered(36), Tx("Bad trade vs losing trade", "Mauvais trade ou trade perdant")),
        ],
        locked_hint: Tx("Pass Stage 6 to unlock.", "Réussis l’étape 6 pour débloquer."),
        pace: Tx("Suggested: weeks 12–16", "Conseillé : semaines 12–16"),
    },
    Section {
        eyebrow: Tx("Stage 8", "Étape 8"),
        title: Tx("Plan & Practise", "Planifier et s’entraîner"),
        blurb: Tx("The Classic Price Action setup end to end, your daily routine, your written plan, and your first Demo Trade.", "La configuration Price Action classique de bout en bout, ta routine quotidienne, ton plan écrit et ton premier Trade démo."),
        lessons: &[
            (Numbered(37), Tx("The Classic Price Action setup", "La configuration Price Action classique")),
            (Numbered(38), Tx("The pre-session routine", "La routine avant la séance")),
            (Numbered(39), Tx("Writing your Trading Plan", "Écrire ton Plan de trading")),
            (Numbered(40), Tx("Your first Demo Trade & weekly review", "Ton premier Trade démo et le bilan de la semaine")),
        ],
        locked_hint: Tx("Pass Stage 7 to unlock.", "Réussis l’étape 7 pour débloquer."),
        pace: Tx("Suggested: week 16 on", "Conseillé : à partir de la semaine 16"),
    },
    Section {
        eyebrow: Tx("Live Track", "Parcours réel"),
        title: Tx("Going Live", "Passer au réel"),
        blurb: Tx("Reviewing your journal, avoiding scams, starting small with real money, and when to size up.", "Relire ton journal, éviter les arnaques, commencer petit avec de l’argent réel, et savoir quand grossir."),
        lessons: &[
            (Numbered(41), Tx("Reviewing your journal", "Relire ton journal")),
            (Numbered(42), Tx("Brokers & scams", "Courtiers et arnaques")),
            (Numbered(43), Tx("Going live, small", "Passer au réel, petit")),
            (Numbered(44), Tx("Scaling up", "Grossir progressivement")),
        ],
        locked_hint: READINESS_HINT,
        pace: NONE,
    },
    Section {
        eyebrow: Tx("Advanced Track", "Parcours avancé"),
        title: Tx("More Strategies", "D’autres stratégies"),
        blurb: Tx("Breakouts, the Asian range, news follow-through, Supply & Demand and SMC, all built on Classic Price Action.", "Cassures, fourchette asiatique, suite d’annonce, offre et demande et SMC, tout construit sur la Price Action classique."),
        lessons: &[
            (Advanced(1), Tx("Breakout & retest", "Cassure et retest")),
            (Advanced(2), Tx("Asian-range breakout", "Cassure de la fourchette asiatique")),
            (Advanced(3), Tx("News follow-through", "La suite d’une annonce")),
            (Advanced(4), Tx("Supply & demand zones", "Zones d’offre et de demande")),
            (Advanced(5), Tx("Fresh vs tested zones", "Zones neuves ou déjà testées")),
            (Advanced(6), Tx("Zones with Classic structure", "Les zones avec la structure classique")),
            (Advanced(7), Tx("Liquidity & sweeps", "Liquidité et balayages")),
            (Advanced(8), Tx("Order blocks", "Les order blocks")),
            (Advanced(9), Tx("Fair value gaps", "Les fair value gaps")),
            (Advanced(10), Tx("Your SMC rulebook", "Ton règlement SMC")),
        ],
        locked_hint: Tx("Unlocks with Live Readiness.", "Se débloque avec « Prêt pour le réel »."),
        pace: NONE,
    },
];

pub fn title(id: LessonId) -> &'static str {
    if let Numbered(n) = id {
        if let Some(l) = crate::i18n::lesson(n) {
            return l.title;
        }
    }
    SECTIONS.iter().flat_map(|s| s.lessons.iter()).find(|(l, _)| *l == id).map_or(t("Lesson", "Leçon"), |(_, x)| x.get())
}

pub fn label(id: LessonId) -> String {
    match id {
        Numbered(n) => n.to_string(),
        Advanced(n) => format!("A{n}"),
        Indicators => "+".into(),
    }
}

pub fn eyebrow(id: LessonId) -> String {
    let fr = crate::i18n::is_fr();
    match (id, id.stage()) {
        (Numbered(n), Some(s)) => if fr { format!("Étape {s} · Leçon {n}") } else { format!("Stage {s} · Lesson {n}") },
        (Numbered(n), None) => if fr { format!("Parcours réel · Leçon {n}") } else { format!("Live Track · Lesson {n}") },
        (Advanced(n), _) => if fr { format!("Parcours avancé · A{n}") } else { format!("Advanced Track · A{n}") },
        (Indicators, _) => t("Side lesson · Optional", "Leçon annexe · Facultative").into(),
    }
}
