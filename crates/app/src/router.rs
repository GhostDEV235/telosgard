//! Plain hash routes (`#today`, `#lesson-5`, …): the only hash form the artifact viewer passes through.

use telosgard_core::course::LessonId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrillKind {
    Calibration,
    Direction,
    BidAsk,
    Distance,
    Pips,
    Orders,
    Session,
    NoTrade,
    Surprise,
}

impl DrillKind {
    pub const ALL: [DrillKind; 9] = [DrillKind::Surprise, DrillKind::Session, DrillKind::NoTrade, DrillKind::Calibration, DrillKind::Direction, DrillKind::BidAsk, DrillKind::Distance, DrillKind::Pips, DrillKind::Orders];
    pub fn slug(self) -> &'static str {
        match self {
            DrillKind::Calibration => "calibration",
            DrillKind::Direction => "direction",
            DrillKind::BidAsk => "bid-ask",
            DrillKind::Distance => "distance",
            DrillKind::Pips => "pips",
            DrillKind::Orders => "orders",
            DrillKind::Session => "session",
            DrillKind::NoTrade => "no-trade",
            DrillKind::Surprise => "surprise",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Today,
    Lesson(LessonId),
    Practice,
    Drill(DrillKind),
    Tools,
    Plan,
    Journal,
    Readiness,
    GraderLab,
    WidgetLab,
    QuestionStats,
    Checklist,
    Review,
    TradeNew,
    TradeSkip,
    Trade(String),
    /// `#join-<invite token>`: create an account from an invite link.
    Join(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Today,
    Practice,
    Tools,
    Journal,
}

impl Route {
    pub fn parse(hash: &str) -> Route {
        let h = hash.trim_start_matches('#');
        match h {
            "" | "today" => Route::Today,
            "practice" => Route::Practice,
            "checklist" => Route::Checklist,
            "review" => Route::Review,
            "trade-new" => Route::TradeNew,
            "trade-skip" => Route::TradeSkip,
            "size" => Route::Tools,
            "plan" => Route::Plan,
            "journal" => Route::Journal,
            "readiness" => Route::Readiness,
            "grader-lab" => Route::GraderLab,
            "widget-lab" => Route::WidgetLab,
            "question-stats" => Route::QuestionStats,
            // The owner's own first account: no invite needed for an OWNER_EMAILS address.
            "join" => Route::Join(String::new()),
            _ => {
                if let Some(t) = h.strip_prefix("join-").filter(|t| (20..=100).contains(&t.len()) && t.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')) {
                    Route::Join(t.to_string())
                } else if let Some(d) = h.strip_prefix("drill-").and_then(|s| DrillKind::ALL.into_iter().find(|k| k.slug() == s)) {
                    Route::Drill(d)
                } else if let Some(id) = h.strip_prefix("trade-").filter(|s| s.len() >= 8 && s.chars().all(|c| c.is_ascii_alphanumeric())) {
                    Route::Trade(format!("trade-{id}"))
                } else {
                    h.strip_prefix("lesson-").and_then(parse_lesson).map(Route::Lesson).unwrap_or(Route::Today)
                }
            }
        }
    }

    pub fn tab(&self) -> Tab {
        match self {
            Route::Today | Route::Lesson(_) | Route::GraderLab | Route::WidgetLab | Route::QuestionStats | Route::Join(_) => Tab::Today,
            Route::Practice | Route::Drill(_) => Tab::Practice,
            Route::Tools | Route::Plan => Tab::Tools,
            Route::Journal | Route::Readiness | Route::Checklist | Route::Review | Route::TradeNew | Route::TradeSkip | Route::Trade(_) => Tab::Journal,
        }
    }
}

fn parse_lesson(s: &str) -> Option<LessonId> {
    let id = match s {
        "ind" => LessonId::Indicators,
        _ => match s.strip_prefix('a') {
            Some(n) => LessonId::Advanced(n.parse().ok()?),
            None => LessonId::Numbered(s.parse().ok()?),
        },
    };
    id.is_valid().then_some(id)
}

pub fn lesson_href(id: LessonId) -> String {
    match id {
        LessonId::Numbered(n) => format!("#lesson-{n}"),
        LessonId::Advanced(n) => format!("#lesson-a{n}"),
        LessonId::Indicators => "#lesson-ind".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_routes_and_round_trips_lessons() {
        assert_eq!(Route::parse(""), Route::Today);
        assert_eq!(Route::parse("#size"), Route::Tools);
        assert_eq!(Route::parse("#nonsense"), Route::Today);
        assert_eq!(Route::parse("#lesson-45"), Route::Today);
        assert_eq!(Route::parse("#lesson-44"), Route::Lesson(LessonId::Numbered(44)));
        assert_eq!(Route::parse("#lesson-39"), Route::Lesson(LessonId::Numbered(39)));
        assert_eq!(Route::parse("#drill-pips"), Route::Drill(DrillKind::Pips));
        assert_eq!(Route::parse("#trade-0198abc0123def45"), Route::Trade("trade-0198abc0123def45".into()));
        assert_eq!(Route::parse("#trade-x"), Route::Today, "too short to be an id");
        assert_eq!(Route::parse("#trade-new"), Route::TradeNew);
        for id in [LessonId::Numbered(5), LessonId::Advanced(2), LessonId::Indicators] {
            assert_eq!(Route::parse(&lesson_href(id)), Route::Lesson(id));
        }
    }
}
