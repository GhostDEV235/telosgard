//! Course map and unlock order (PRD §4.1, FR-2.2).
//! Core Track L1–25 unlock in sequence. Live Track L26–29 and Advanced Track A1–A7 need Live Readiness,
//! then also unlock in sequence. The Indicators side lesson unlocks once Stage 3 (L18) is passed.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LessonId {
    /// L1–L29 (Core 1–25, Live 26–29).
    Numbered(u8),
    /// A1–A7.
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

pub const LAST_CORE: u8 = 25;
pub const LAST_LIVE: u8 = 29;
pub const LAST_ADVANCED: u8 = 7;
/// Inclusive lesson ranges of the Core Track's stages.
pub const STAGES: [(u8, u8); 5] = [(1, 9), (10, 13), (14, 18), (19, 22), (23, 25)];

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
    /// The Core Track stage (1–5), if any.
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
            LessonId::Indicators => Some(LessonId::Numbered(STAGES[2].1)),
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
        assert_eq!(Numbered(25).track(), Track::Core);
        assert_eq!(Numbered(26).track(), Track::Live);
        assert_eq!(Advanced(1).track(), Track::Advanced);
        assert_eq!(Indicators.track(), Track::Side);
        assert_eq!(Numbered(9).stage(), Some(1));
        assert_eq!(Numbered(10).stage(), Some(2));
        assert_eq!(Numbered(18).stage(), Some(3));
        assert_eq!(Numbered(25).stage(), Some(5));
        assert_eq!(Numbered(26).stage(), None);
    }

    #[test]
    fn validity() {
        assert!(Numbered(1).is_valid() && Numbered(29).is_valid() && Advanced(7).is_valid());
        assert!(!Numbered(0).is_valid() && !Numbered(30).is_valid() && !Advanced(0).is_valid() && !Advanced(8).is_valid());
    }

    #[test]
    fn doc_ids_round_trip() {
        for id in [Numbered(5), Numbered(26), Advanced(1), Indicators] {
            assert_eq!(LessonId::from_doc_id(&id.doc_id()), Some(id));
        }
        assert_eq!(Numbered(5).doc_id(), "lesson-05");
        assert_eq!(Advanced(3).doc_id(), "lesson-a3");
        assert_eq!(Indicators.doc_id(), "lesson-ind");
        assert_eq!(LessonId::from_doc_id("lesson-30"), None);
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
        assert!(!Numbered(10).is_unlocked(|_| false, true));
    }

    #[test]
    fn live_and_advanced_need_readiness() {
        let all_core = |id: LessonId| matches!(id, Numbered(n) if n <= 25);
        assert!(!Numbered(26).is_unlocked(all_core, false));
        assert!(Numbered(26).is_unlocked(all_core, true));
        assert!(!Numbered(27).is_unlocked(all_core, true));
        assert!(!Advanced(1).is_unlocked(all_core, false));
        assert!(Advanced(1).is_unlocked(all_core, true));
        assert!(!Advanced(2).is_unlocked(all_core, true));
        assert!(Advanced(2).is_unlocked(|id| all_core(id) || id == Advanced(1), true));
    }

    #[test]
    fn indicators_after_stage_three() {
        assert!(!Indicators.is_unlocked(|id| matches!(id, Numbered(n) if n <= 17), false));
        assert!(Indicators.is_unlocked(|id| matches!(id, Numbered(n) if n <= 18), false));
    }
}
