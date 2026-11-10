//! Course map and unlock order (PRD §4.1, FR-2.2).
//! Core Track L1–39 unlock in sequence. Live Track L40–43 and Advanced Track A1–A10 need Live Readiness,
//! then also unlock in sequence. The Indicators side lesson unlocks once Stage 5 (L27) is passed.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LessonId {
    /// L1–L43 (Core 1–39, Live 40–43).
    Numbered(u8),
    /// A1–A10.
    Advanced(u8),
    Indicators,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Track {
    Core,
    Live,
    Advanced,
    Side,
}

pub const LAST_CORE: u8 = 39;
pub const LAST_LIVE: u8 = 43;
pub const LAST_ADVANCED: u8 = 10;
/// Inclusive lesson ranges of the Core Track's stages.
pub const STAGES: [(u8, u8); 8] = [(1, 11), (12, 14), (15, 19), (20, 22), (23, 27), (28, 30), (31, 35), (36, 39)];

impl LessonId {
    pub fn is_valid(self) -> bool {
        match self {
            LessonId::Numbered(n) => (1..=LAST_LIVE).contains(&n),
            LessonId::Advanced(n) => (1..=LAST_ADVANCED).contains(&n),
            LessonId::Indicators => true,
        }
    }
    pub fn track(self) -> Track {
        match self {
            LessonId::Numbered(n) if n <= LAST_CORE => Track::Core,
            LessonId::Numbered(_) => Track::Live,
            LessonId::Advanced(_) => Track::Advanced,
            LessonId::Indicators => Track::Side,
        }
    }
    /// The Core Track stage (1–8), if any.
    pub fn stage(self) -> Option<u8> {
        let LessonId::Numbered(n) = self else { return None };
        STAGES.iter().position(|(a, b)| (*a..=*b).contains(&n)).map(|i| i as u8 + 1)
    }
    /// The database doc id: `lesson-05`, `lesson-a1`, `lesson-ind`.
    pub fn doc_id(self) -> String {
        match self {
            LessonId::Numbered(n) => format!("lesson-{n:02}"),
            LessonId::Advanced(n) => format!("lesson-a{n}"),
            LessonId::Indicators => "lesson-ind".into(),
        }
    }
    pub fn from_doc_id(s: &str) -> Option<LessonId> {
        let rest = s.strip_prefix("lesson-")?;
        let id = match rest {
            "ind" => LessonId::Indicators,
            _ => match rest.strip_prefix('a') {
                Some(n) => LessonId::Advanced(n.parse().ok()?),
                None => LessonId::Numbered(rest.parse().ok()?),
            },
        };
        (id.is_valid() && id.doc_id() == s).then_some(id)
    }
    /// The lesson that must be passed before this one, if any.
    pub fn predecessor(self) -> Option<LessonId> {
        match self {
            LessonId::Numbered(1) | LessonId::Advanced(1) => None,
            LessonId::Numbered(n) => Some(LessonId::Numbered(n - 1)),
            LessonId::Advanced(n) => Some(LessonId::Advanced(n - 1)),
            LessonId::Indicators => Some(LessonId::Numbered(STAGES[4].1)),
        }
    }
    pub fn is_unlocked(self, passed: impl Fn(LessonId) -> bool, readiness_reached: bool) -> bool {
        if !self.is_valid() {
            return false;
        }
        let needs_readiness = matches!(self.track(), Track::Live | Track::Advanced);
        if needs_readiness && !readiness_reached {
            return false;
        }
        // The first Live lesson follows readiness, not L25 directly.
        if self == LessonId::Numbered(LAST_CORE + 1) {
            return true;
        }
        self.predecessor().is_none_or(passed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use LessonId::*;

    #[test]
    fn tracks_and_stages() {
        assert_eq!(Numbered(1).track(), Track::Core);
        assert_eq!(Numbered(39).track(), Track::Core);
        assert_eq!(Numbered(40).track(), Track::Live);
        assert_eq!(Advanced(1).track(), Track::Advanced);
        assert_eq!(Indicators.track(), Track::Side);
        let cases = [(1, 1), (11, 1), (12, 2), (14, 2), (15, 3), (19, 3), (20, 4), (22, 4), (23, 5), (27, 5), (28, 6), (30, 6), (31, 7), (35, 7), (36, 8), (39, 8)];
        for (lesson, stage) in cases {
            assert_eq!(Numbered(lesson).stage(), Some(stage), "lesson {lesson}");
        }
        assert_eq!(Numbered(40).stage(), None);
        assert_eq!(STAGES.len(), 8);
    }

    #[test]
    fn validity() {
        assert!(Numbered(1).is_valid() && Numbered(43).is_valid() && Advanced(10).is_valid());
        assert!(!Numbered(0).is_valid() && !Numbered(44).is_valid() && !Advanced(0).is_valid() && !Advanced(11).is_valid());
    }

    #[test]
    fn doc_ids_round_trip() {
        for id in [Numbered(5), Numbered(40), Advanced(10), Indicators] {
            assert_eq!(LessonId::from_doc_id(&id.doc_id()), Some(id));
        }
        assert_eq!(Numbered(5).doc_id(), "lesson-05");
        assert_eq!(Advanced(3).doc_id(), "lesson-a3");
        assert_eq!(Indicators.doc_id(), "lesson-ind");
        assert_eq!(LessonId::from_doc_id("lesson-44"), None);
        assert_eq!(LessonId::from_doc_id("plan-v1"), None);
    }

    #[test]
    fn core_lessons_unlock_in_order() {
        let passed = |id: LessonId| matches!(id, Numbered(n) if n <= 4);
        assert!(Numbered(1).is_unlocked(|_| false, false));
        assert!(Numbered(5).is_unlocked(passed, false));
        assert!(!Numbered(6).is_unlocked(passed, false), "lessons cannot be skipped");
    }

    #[test]
    fn readiness_alone_does_not_skip_core() {
        assert!(!Numbered(12).is_unlocked(|_| false, true));
    }

    #[test]
    fn live_and_advanced_need_readiness() {
        let all_core = |id: LessonId| matches!(id, Numbered(n) if n <= 39);
        assert!(!Numbered(40).is_unlocked(all_core, false));
        assert!(Numbered(40).is_unlocked(all_core, true));
        assert!(!Numbered(41).is_unlocked(all_core, true));
        assert!(!Advanced(1).is_unlocked(all_core, false));
        assert!(Advanced(1).is_unlocked(all_core, true));
        assert!(!Advanced(2).is_unlocked(all_core, true));
        assert!(Advanced(2).is_unlocked(|id| all_core(id) || id == Advanced(1), true));
    }

    #[test]
    fn indicators_after_stage_five() {
        assert!(!Indicators.is_unlocked(|id| matches!(id, Numbered(n) if n <= 26), false));
        assert!(Indicators.is_unlocked(|id| matches!(id, Numbered(n) if n <= 27), false));
    }
}
