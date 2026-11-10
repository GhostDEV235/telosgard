//! Plain hash routes (`#today`, `#lesson-5`, …): the only hash form the artifact viewer passes through.

use gold_core::course::LessonId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrillKind {
    Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Today,
    Lesson(LessonId),
    Practice,
    Drill(DrillKind),
    Tools,
    Plan,
    Journal,
    Readiness,
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
            "drill-direction" => Route::Drill(DrillKind::Direction),
            "size" => Route::Tools,
            "plan" => Route::Plan,
            "journal" => Route::Journal,
            "readiness" => Route::Readiness,
            _ => h.strip_prefix("lesson-").and_then(parse_lesson).map(Route::Lesson).unwrap_or(Route::Today),
        }
    }

    pub fn tab(self) -> Tab {
        match self {
            Route::Today | Route::Lesson(_) => Tab::Today,
            Route::Practice | Route::Drill(_) => Tab::Practice,
            Route::Tools | Route::Plan => Tab::Tools,
            Route::Journal | Route::Readiness => Tab::Journal,
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
        assert_eq!(Route::parse("#lesson-30"), Route::Today);
        for id in [LessonId::Numbered(5), LessonId::Advanced(2), LessonId::Indicators] {
            assert_eq!(Route::parse(&lesson_href(id)), Route::Lesson(id));
        }
    }
}
