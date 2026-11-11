//! Retention (FR-13): Question Records, Confidence and Spaced Review picking.
//! A right answer counts for the Gate whatever its Confidence; here a guessed right answer is
//! treated as not yet learned. Picking is integer-weighted and deterministic for a seed.

use crate::Millis;
use crate::drills::Rng;
use serde::{Deserialize, Serialize};

/// The Learner's own tag on an answer, chosen before it is revealed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    Sure,
    Guessing,
}

/// How the latest answer went.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Outcome {
    Wrong,
    /// Right, but tagged Guessing.
    Guessed,
    /// Right and tagged Sure.
    Sure,
}

/// Right while Sure this many times in a row and the question leaves Spaced Review.
pub const RETIRE_AFTER_SURE: u32 = 2;
pub const WEIGHT_WRONG: u64 = 3;
pub const WEIGHT_GUESSED: u64 = 2;
pub const WEIGHT_OTHER: u64 = 1;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionRecord {
    pub answered: u32,
    pub wrong: u32,
    /// Every answer tagged Guessing, right or wrong.
    #[serde(default)]
    pub guessed: u32,
    pub guessed_right: u32,
    pub last_at: Millis,
    #[serde(default)]
    pub last: Option<Outcome>,
    /// Right-while-Sure answers in a row, ending with the latest.
    #[serde(default)]
    pub sure_streak: u32,
}

impl QuestionRecord {
    pub fn record(&mut self, correct: bool, confidence: Confidence, at: Millis) {
        self.answered += 1;
        self.last_at = self.last_at.max(at);
        let guessing = confidence == Confidence::Guessing;
        self.guessed += guessing as u32;
        let outcome = match (correct, guessing) {
            (false, _) => Outcome::Wrong,
            (true, true) => Outcome::Guessed,
            (true, false) => Outcome::Sure,
        };
        match outcome {
            Outcome::Wrong => self.wrong += 1,
            Outcome::Guessed => self.guessed_right += 1,
            Outcome::Sure => {}
        }
        self.sure_streak = if outcome == Outcome::Sure { self.sure_streak + 1 } else { 0 };
        self.last = Some(outcome);
    }

    /// Learned: right while Sure twice in a row.
    pub fn retired(&self) -> bool {
        self.sure_streak >= RETIRE_AFTER_SURE
    }

    /// Spaced Review weight: 0 when retired, else by the latest outcome. Never-answered counts as 1.
    pub fn weight(&self) -> u64 {
        if self.retired() {
            return 0;
        }
        match self.last {
            Some(Outcome::Wrong) => WEIGHT_WRONG,
            Some(Outcome::Guessed) => WEIGHT_GUESSED,
            _ => WEIGHT_OTHER,
        }
    }

    /// Adds a record kept elsewhere (answers given before storage connected).
    pub fn absorb(&mut self, other: &QuestionRecord) {
        self.answered += other.answered;
        self.wrong += other.wrong;
        self.guessed += other.guessed;
        self.guessed_right += other.guessed_right;
        if other.last_at >= self.last_at && other.last.is_some() {
            self.last = other.last;
            self.sure_streak = if other.answered == other.sure_streak { self.sure_streak + other.sure_streak } else { other.sure_streak };
        }
        self.last_at = self.last_at.max(other.last_at);
    }

    /// (answered, wrong %, guessed %), rounded down. Zero answers gives zeros.
    pub fn stats(&self) -> (u32, u32, u32) {
        if self.answered == 0 {
            return (0, 0, 0);
        }
        (self.answered, self.wrong * 100 / self.answered, self.guessed * 100 / self.answered)
    }
}

/// One question somewhere in the course: lesson number and its stable id ("q1", "r2").
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuestionKey {
    pub lesson: u8,
    pub id: String,
}

impl QuestionKey {
    pub fn new(lesson: u8, id: &str) -> QuestionKey {
        QuestionKey { lesson, id: id.to_string() }
    }
}

/// Picks up to `n` distinct questions for Spaced Review.
/// `pool` holds every question with its record (None = never answered). Only questions from
/// `passed` lessons other than `current` are eligible; retired ones are skipped. Weighted draws
/// without replacement; the same inputs and seed always give the same picks, in draw order.
pub fn pick_review(pool: &[(QuestionKey, Option<&QuestionRecord>)], passed: &[u8], current: u8, n: usize, seed: u64) -> Vec<QuestionKey> {
    let mut cands: Vec<(QuestionKey, u64)> = pool
        .iter()
        .filter(|(k, _)| k.lesson != current && passed.contains(&k.lesson))
        .map(|(k, r)| (k.clone(), r.map_or(WEIGHT_OTHER, QuestionRecord::weight)))
        .filter(|(_, w)| *w > 0)
        .collect();
    cands.sort();
    cands.dedup_by(|a, b| a.0 == b.0);
    let mut rng = Rng::new(seed);
    let mut out = Vec::new();
    while out.len() < n && !cands.is_empty() {
        let total: u64 = cands.iter().map(|c| c.1).sum();
        let mut r = rng.below(total);
        let i = cands
            .iter()
            .position(|c| {
                if r < c.1 {
                    true
                } else {
                    r -= c.1;
                    false
                }
            })
            .expect("r < total");
        out.push(cands.remove(i).0);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use Confidence::*;

    fn rec(answers: &[(bool, Confidence)]) -> QuestionRecord {
        let mut r = QuestionRecord::default();
        for (i, (c, conf)) in answers.iter().enumerate() {
            r.record(*c, *conf, i as Millis + 1);
        }
        r
    }

    #[test]
    fn record_counts_each_outcome() {
        let r = rec(&[(false, Sure), (true, Guessing), (false, Guessing), (true, Sure)]);
        assert_eq!((r.answered, r.wrong, r.guessed, r.guessed_right), (4, 2, 2, 1));
        assert_eq!(r.last, Some(Outcome::Sure));
        assert_eq!(r.last_at, 4);
        assert_eq!(r.stats(), (4, 50, 50));
    }

    #[test]
    fn weights_follow_the_latest_answer() {
        assert_eq!(QuestionRecord::default().weight(), 1);
        assert_eq!(rec(&[(false, Sure)]).weight(), 3);
        assert_eq!(rec(&[(true, Guessing)]).weight(), 2);
        assert_eq!(rec(&[(true, Sure)]).weight(), 1);
        assert_eq!(rec(&[(true, Sure), (false, Guessing)]).weight(), 3);
    }

    #[test]
    fn two_sure_rights_in_a_row_retire_a_question() {
        assert!(!rec(&[(true, Sure)]).retired());
        assert!(rec(&[(true, Sure), (true, Sure)]).retired());
        assert_eq!(rec(&[(true, Sure), (true, Sure)]).weight(), 0);
        // A guess or a miss in between resets the streak.
        assert!(!rec(&[(true, Sure), (true, Guessing), (true, Sure)]).retired());
        assert!(!rec(&[(true, Sure), (false, Sure), (true, Sure)]).retired());
        assert!(rec(&[(false, Sure), (true, Sure), (true, Sure)]).retired());
    }

    #[test]
    fn absorb_adds_counts_and_keeps_the_later_outcome() {
        let mut a = rec(&[(true, Sure)]);
        let mut later = QuestionRecord::default();
        later.record(true, Sure, 10);
        a.absorb(&later);
        assert_eq!(a.answered, 2);
        assert!(a.retired(), "a Sure streak continues across the two halves");
        let mut b = rec(&[(true, Sure)]);
        let mut miss = QuestionRecord::default();
        miss.record(false, Guessing, 10);
        b.absorb(&miss);
        assert_eq!((b.answered, b.wrong, b.guessed, b.last, b.sure_streak), (2, 1, 1, Some(Outcome::Wrong), 0));
    }

    fn keys(lesson: u8, ids: &[&str]) -> Vec<QuestionKey> {
        ids.iter().map(|i| QuestionKey::new(lesson, i)).collect()
    }

    #[test]
    fn only_passed_lessons_other_than_the_current_one_are_eligible() {
        let mut ks = keys(1, &["q1", "q2"]);
        ks.extend(keys(2, &["q1", "q2"]));
        ks.extend(keys(3, &["q1"]));
        let pool: Vec<_> = ks.iter().map(|k| (k.clone(), None)).collect();
        for seed in 0..50 {
            let picked = pick_review(&pool, &[1, 2], 2, 3, seed);
            assert_eq!(picked.len(), 2);
            assert!(picked.iter().all(|k| k.lesson == 1), "{picked:?}");
        }
        assert!(pick_review(&pool, &[], 1, 3, 7).is_empty());
        assert!(pick_review(&pool, &[1], 1, 3, 7).is_empty());
    }

    #[test]
    fn retired_questions_are_skipped_and_picks_are_distinct() {
        let done = rec(&[(true, Sure), (true, Sure)]);
        let ks = keys(1, &["q1", "q2", "q3", "q4"]);
        let pool = vec![(ks[0].clone(), Some(&done)), (ks[1].clone(), None), (ks[2].clone(), None), (ks[3].clone(), None)];
        for seed in 0..50 {
            let mut picked = pick_review(&pool, &[1], 2, 3, seed);
            assert!(!picked.contains(&ks[0]));
            picked.sort();
            picked.dedup();
            assert_eq!(picked.len(), 3);
        }
    }

    #[test]
    fn same_seed_same_picks() {
        let ks = keys(1, &["q1", "q2", "q3", "q4", "q5", "r1", "r2", "r3"]);
        let pool: Vec<_> = ks.iter().map(|k| (k.clone(), None)).collect();
        let mut shuffled = pool.clone();
        shuffled.reverse();
        let a = pick_review(&pool, &[1], 2, 3, 42);
        assert_eq!(a, pick_review(&pool, &[1], 2, 3, 42));
        assert_eq!(a, pick_review(&shuffled, &[1], 2, 3, 42), "pool order doesn't matter");
        let differs = (0..20).any(|s| pick_review(&pool, &[1], 2, 3, s) != a);
        assert!(differs, "different seeds vary the picks");
    }

    #[test]
    fn wrong_and_guessed_answers_are_picked_more_often() {
        let wrong = rec(&[(false, Sure)]);
        let guessed = rec(&[(true, Guessing)]);
        let sure = rec(&[(true, Sure)]);
        let ks = keys(1, &["w", "g", "s"]);
        let pool = vec![(ks[0].clone(), Some(&wrong)), (ks[1].clone(), Some(&guessed)), (ks[2].clone(), Some(&sure))];
        let mut first = [0u32; 3];
        for seed in 0..6000 {
            let p = pick_review(&pool, &[1], 2, 1, seed);
            first[ks.iter().position(|k| *k == p[0]).unwrap()] += 1;
        }
        // Expected 3:2:1 of 6000 = 3000, 2000, 1000.
        assert!((2800..3200).contains(&first[0]), "{first:?}");
        assert!((1800..2200).contains(&first[1]), "{first:?}");
        assert!((850..1150).contains(&first[2]), "{first:?}");
    }

    #[test]
    fn a_guessed_right_answer_comes_back_in_the_next_lessons_review() {
        // Lesson 1 passed with q3 right-but-guessed and everything else learned.
        let learned = rec(&[(true, Sure), (true, Sure)]);
        let guessed = rec(&[(true, Guessing)]);
        let ks = keys(1, &["q1", "q2", "q3"]);
        let pool = vec![(ks[0].clone(), Some(&learned)), (ks[1].clone(), Some(&learned)), (ks[2].clone(), Some(&guessed))];
        assert_eq!(pick_review(&pool, &[1], 2, 3, 99), vec![ks[2].clone()]);
    }
}
