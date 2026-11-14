//! Lesson content (CURRICULUM.md), compiled from `lessons/*.md` by `build.rs`.
//! The build fails on any lesson that breaks the authoring rules, so everything here is valid.

#[derive(Debug)]
pub struct Lesson {
    pub id: u8,
    pub stage: u8,
    pub title: &'static str,
    /// Completes "By the end, you'll be able to …".
    pub objective: &'static str,
    pub hook: &'static str,
    /// The explanation, one idea per step (Lesson Steps).
    pub steps: &'static [Step],
    pub analogy: &'static str,
    pub analogy_tag: &'static str,
    pub example: &'static str,
    pub mistake: &'static str,
    pub takeaways: &'static [&'static str],
    /// The main path (hook, steps, analogy, example, mistake, takeaways) at a careful 130 words a
    /// minute; "Why?" detail is extra.
    pub read_minutes: u8,
    pub videos: &'static [Video],
    pub sources: &'static [Source],
    pub quiz: &'static [QuizQ],
    /// Three quick questions from memory before the Quiz. Practice only.
    pub recall: &'static [QuizQ],
    pub explain: ExplainBack,
    pub drills: &'static [&'static str],
}

#[derive(Debug)]
pub struct Step {
    pub heading: &'static str,
    /// Main text (at most 60 words), pre-rendered from Markdown.
    pub text_html: &'static str,
    /// Optional deeper detail behind a "Why?" toggle, pre-rendered from Markdown.
    pub why_html: Option<&'static str>,
    /// Optional interactive Widget id (see WIDGETS).
    pub widget: Option<&'static str>,
}

/// Every Widget id a lesson may use; the app has a component for each.
pub const WIDGETS: &[&str] = &["price_drag", "lots_money", "one_percent", "stop_first", "short_risk", "leverage_toggle", "bid_ask", "win_rate", "pip_convert", "session_clock", "surprise_meter", "candle_builder", "candle_merge"];

#[derive(Debug)]
pub struct Video {
    pub id: &'static str,
    pub title: &'static str,
    pub channel: &'static str,
    pub duration: &'static str,
    pub watch_for: &'static str,
    pub fallback_search: &'static str,
    /// Date the link was opened and confirmed (YYYY-MM-DD).
    pub checked: &'static str,
}

impl Video {
    pub fn url(&self) -> String {
        format!("https://www.youtube.com/watch?v={}", self.id)
    }
}

#[derive(Debug)]
pub struct Source {
    pub claim: &'static str,
    pub name: &'static str,
    pub url: &'static str,
    pub checked: &'static str,
}

#[derive(Debug)]
pub struct QuizQ {
    /// Stable within the lesson ("q1"…), so Question Records survive reordering. Recall questions use "r1"….
    pub id: &'static str,
    pub q: &'static str,
    pub options: &'static [&'static str],
    pub answer: usize,
    pub why: &'static str,
}

#[derive(Debug)]
pub struct ExplainBack {
    pub prompt: &'static str,
    pub must_include: &'static [&'static str],
    pub misconceptions: &'static [&'static str],
    /// Correct, gap, misconception: used to tune the rubric in the grader lab.
    pub fixtures: &'static [Fixture],
}

#[derive(Debug)]
pub struct Fixture {
    pub answer: &'static str,
    pub expect: Expect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expect {
    GotIt,
    Almost,
    /// A misconception answer may land on either; it must never be Got it.
    AlmostOrNotYet,
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub fn lesson(id: u8) -> Option<&'static Lesson> {
    LESSONS.iter().find(|l| l.id == id)
}

/// The lesson in French when a French twin exists, otherwise in English.
pub fn lesson_in(fr: bool, id: u8) -> Option<&'static Lesson> {
    if fr {
        if let Some(l) = LESSONS_FR.iter().find(|l| l.id == id) {
            return Some(l);
        }
    }
    lesson(id)
}

/// Every written lesson, each in French where a twin exists.
pub fn lessons_in(fr: bool) -> Vec<&'static Lesson> {
    LESSONS.iter().filter_map(|l| lesson_in(fr, l.id)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn written_lessons_are_contiguous_and_in_their_stages() {
        let ids: Vec<u8> = LESSONS.iter().map(|l| l.id).collect();
        assert_eq!(ids, (1..=ids.len() as u8).collect::<Vec<_>>(), "written lessons run 1..n with no gaps");
        assert!(ids.len() >= 28, "Stages 1–5 are written");
        // Course map v3: Stage 1 = L1–12, Stage 2 = L13–15, Stage 3 = L16–20.
        for l in LESSONS {
            let stage = match l.id {
                1..=12 => 1,
                13..=15 => 2,
                16..=20 => 3,
                21..=23 => 4,
                24..=28 => 5,
                _ => continue,
            };
            assert_eq!(l.stage, stage, "L{} belongs to Stage {stage}", l.id);
        }
    }

    #[test]
    fn every_lesson_reads_in_two_to_five_minutes_on_the_main_path() {
        for l in LESSONS {
            assert!((2..=5).contains(&l.read_minutes), "L{} is {} min", l.id, l.read_minutes);
        }
    }

    #[test]
    fn lessons_are_steps_with_a_recall_set_and_stable_quiz_ids() {
        for l in LESSONS {
            assert!((3..=6).contains(&l.steps.len()), "L{}: 3–6 steps", l.id);
            assert_eq!(l.recall.len(), 3, "L{}: Recall Set", l.id);
            let mut ids: Vec<&str> = l.quiz.iter().chain(l.recall).map(|q| q.id).collect();
            let n = ids.len();
            ids.sort();
            ids.dedup();
            assert_eq!(ids.len(), n, "L{}: question ids are unique", l.id);
            let widgets = l.steps.iter().filter_map(|s| s.widget).count();
            assert!(widgets <= 2, "L{}: at most 2 widgets", l.id);
        }
    }

    #[test]
    fn the_honest_maths_lesson_is_lesson_two() {
        assert_eq!(lesson(2).unwrap().title, "The honest maths of trading");
        assert_eq!(lesson(7).unwrap().title, "Dollar moves, points and pips");
    }

    #[test]
    fn fixtures_cover_correct_gap_and_misconception() {
        for l in LESSONS {
            let e: Vec<Expect> = l.explain.fixtures.iter().map(|f| f.expect).collect();
            assert_eq!(e, [Expect::GotIt, Expect::Almost, Expect::AlmostOrNotYet], "L{}", l.id);
        }
    }

    #[test]
    fn every_lesson_has_a_french_twin() {
        let fr: Vec<u8> = LESSONS_FR.iter().map(|l| l.id).collect();
        let en: Vec<u8> = LESSONS.iter().map(|l| l.id).collect();
        assert_eq!(fr, en, "lessons/fr/ must translate every written lesson");
    }

    #[test]
    fn video_urls() {
        let v = &lesson(1).unwrap().videos[0];
        assert!(v.url().starts_with("https://www.youtube.com/watch?v="));
    }
}
