//! The Gate (PRD FR-2): a Lesson is passed when Quiz ≥ 80% AND the latest Explain-Back is "Got it".
//! Passing is permanent: a later attempt never re-locks a lesson.

use crate::Millis;
use serde::{Deserialize, Serialize};

pub const QUIZ_PASS_PERCENT: u8 = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Grade {
    GotIt,
    Almost,
    NotYet,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonProgress {
    pub quiz_best: Option<u8>,
    pub grade: Option<Grade>,
    pub passed_at: Option<Millis>,
}

/// Percentage of correct answers, rounded down. 0 questions scores 0.
pub fn quiz_score(correct: u32, total: u32) -> u8 {
    if total == 0 { 0 } else { (correct.min(total) * 100 / total) as u8 }
}

impl LessonProgress {
    pub fn record_quiz(&mut self, score: u8, at: Millis) {
        self.quiz_best = Some(self.quiz_best.map_or(score, |b| b.max(score)));
        self.mark_if_passed(at);
    }
    pub fn record_grade(&mut self, grade: Grade, at: Millis) {
        self.grade = Some(grade);
        self.mark_if_passed(at);
    }
    pub fn quiz_met(&self) -> bool {
        self.quiz_best.is_some_and(|s| s >= QUIZ_PASS_PERCENT)
    }
    pub fn explain_back_met(&self) -> bool {
        self.grade == Some(Grade::GotIt)
    }
    pub fn is_passed(&self) -> bool {
        self.passed_at.is_some()
    }
    fn mark_if_passed(&mut self, at: Millis) {
        if self.passed_at.is_none() && self.quiz_met() && self.explain_back_met() {
            self.passed_at = Some(at);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quiz_scores_round_down() {
        assert_eq!(quiz_score(4, 5), 80);
        assert_eq!(quiz_score(7, 9), 77);
        assert_eq!(quiz_score(5, 5), 100);
        assert_eq!(quiz_score(0, 0), 0);
    }

    #[test]
    fn needs_both_quiz_and_got_it() {
        let mut p = LessonProgress::default();
        p.record_quiz(90, 1);
        assert!(p.quiz_met() && !p.is_passed());
        p.record_grade(Grade::Almost, 2);
        assert!(!p.is_passed());
        p.record_grade(Grade::GotIt, 3);
        assert!(p.is_passed());
        assert_eq!(p.passed_at, Some(3));
    }

    #[test]
    fn either_order_works() {
        let mut p = LessonProgress::default();
        p.record_grade(Grade::GotIt, 1);
        assert!(p.explain_back_met() && !p.is_passed());
        p.record_quiz(80, 2);
        assert!(p.is_passed());
        assert_eq!(p.passed_at, Some(2));
    }

    #[test]
    fn seventy_nine_is_not_enough() {
        let mut p = LessonProgress::default();
        p.record_grade(Grade::GotIt, 1);
        p.record_quiz(79, 2);
        assert!(!p.is_passed());
    }

    #[test]
    fn quiz_keeps_best_score() {
        let mut p = LessonProgress::default();
        p.record_quiz(90, 1);
        p.record_quiz(40, 2);
        assert_eq!(p.quiz_best, Some(90));
    }

    #[test]
    fn latest_grade_counts_before_passing() {
        let mut p = LessonProgress::default();
        p.record_grade(Grade::GotIt, 1);
        p.record_grade(Grade::NotYet, 2);
        p.record_quiz(100, 3);
        assert!(!p.is_passed(), "a later Not yet replaces an earlier Got it until the gate is met");
    }

    #[test]
    fn passing_is_permanent() {
        let mut p = LessonProgress::default();
        p.record_quiz(100, 1);
        p.record_grade(Grade::GotIt, 2);
        p.record_grade(Grade::NotYet, 3);
        p.record_quiz(10, 4);
        assert!(p.is_passed());
        assert_eq!(p.passed_at, Some(2));
    }
}
