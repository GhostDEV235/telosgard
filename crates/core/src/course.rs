//! Course map and unlock order (PRD §4.1, FR-2.2).
//! Core Track L1–40 unlock in sequence. Live Track L41–44 and Advanced Track A1–A10 need Live Readiness,
//! then also unlock in sequence. The Indicators side lesson unlocks once Stage 5 (L28) is passed.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LessonId {
    /// L1–L44 (Core 1–40, Live 41–44).
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

pub const LAST_CORE: u8 = 40;
pub const LAST_LIVE: u8 = 44;
pub const LAST_ADVANCED: u8 = 10;
/// Inclusive lesson ranges of the Core Track's stages.
pub const STAGES: [(u8, u8); 8] = [(1, 12), (13, 15), (16, 20), (21, 23), (24, 28), (29, 31), (32, 36), (37, 40)];

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
        assert_eq!(Numbered(40).track(), Track::Core);
        assert_eq!(Numbered(41).track(), Track::Live);
        assert_eq!(Advanced(1).track(), Track::Advanced);
        assert_eq!(Indicators.track(), Track::Side);
        let cases = [(1, 1), (12, 1), (13, 2), (15, 2), (16, 3), (20, 3), (21, 4), (23, 4), (24, 5), (28, 5), (29, 6), (31, 6), (32, 7), (36, 7), (37, 8), (40, 8)];
        for (lesson, stage) in cases {
            assert_eq!(Numbered(lesson).stage(), Some(stage), "lesson {lesson}");
        }
        assert_eq!(Numbered(41).stage(), None);
        assert_eq!(STAGES.len(), 8);
    }

    #[test]
    fn validity() {
        assert!(Numbered(1).is_valid() && Numbered(44).is_valid() && Advanced(10).is_valid());
        assert!(!Numbered(0).is_valid() && !Numbered(45).is_valid() && !Advanced(0).is_valid() && !Advanced(11).is_valid());
    }

    #[test]
    fn doc_ids_round_trip() {
        for id in [Numbered(5), Numbered(41), Advanced(10), Indicators] {
            assert_eq!(LessonId::from_doc_id(&id.doc_id()), Some(id));
        }
        assert_eq!(Numbered(5).doc_id(), "lesson-05");
        assert_eq!(Advanced(3).doc_id(), "lesson-a3");
        assert_eq!(Indicators.doc_id(), "lesson-ind");
        assert_eq!(LessonId::from_doc_id("lesson-45"), None);
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
        assert!(!Numbered(13).is_unlocked(|_| false, true));
    }

    #[test]
    fn live_and_advanced_need_readiness() {
        let all_core = |id: LessonId| matches!(id, Numbered(n) if n <= 40);
        assert!(!Numbered(41).is_unlocked(all_core, false));
        assert!(Numbered(41).is_unlocked(all_core, true));
        assert!(!Numbered(42).is_unlocked(all_core, true));
        assert!(!Advanced(1).is_unlocked(all_core, false));
        assert!(Advanced(1).is_unlocked(all_core, true));
        assert!(!Advanced(2).is_unlocked(all_core, true));
        assert!(Advanced(2).is_unlocked(|id| all_core(id) || id == Advanced(1), true));
    }

    #[test]
    fn indicators_after_stage_five() {
        assert!(!Indicators.is_unlocked(|id| matches!(id, Numbered(n) if n <= 27), false));
        assert!(Indicators.is_unlocked(|id| matches!(id, Numbered(n) if n <= 28), false));
    }
}
