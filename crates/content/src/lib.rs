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
    /// The explanation, pre-rendered from Markdown.
    pub body_html: &'static str,
    pub analogy: &'static str,
    pub analogy_tag: &'static str,
    pub example: &'static str,
    pub mistake: &'static str,
    pub takeaways: &'static [&'static str],
    /// Estimated at a careful 130 words a minute.
    pub read_minutes: u8,
    pub videos: &'static [Video],
    pub sources: &'static [Source],
    pub quiz: &'static [QuizQ],
    pub explain: ExplainBack,
    pub drills: &'static [&'static str],
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_one_is_complete_and_ordered() {
        let ids: Vec<u8> = LESSONS.iter().map(|l| l.id).collect();
        assert_eq!(ids, (1..=11).collect::<Vec<_>>());
        assert!(LESSONS.iter().all(|l| l.stage == 1));
    }

    #[test]
    fn every_lesson_is_a_five_to_eight_minute_read() {
        for l in LESSONS {
            assert!((5..=8).contains(&l.read_minutes), "L{} is {} min", l.id, l.read_minutes);
        }
    }

    #[test]
    fn fixtures_cover_correct_gap_and_misconception() {
        for l in LESSONS {
            let e: Vec<Expect> = l.explain.fixtures.iter().map(|f| f.expect).collect();
            assert_eq!(e, [Expect::GotIt, Expect::Almost, Expect::AlmostOrNotYet], "L{}", l.id);
        }
    }

    #[test]
    fn video_urls() {
        let v = &lesson(1).unwrap().videos[0];
        assert!(v.url().starts_with("https://www.youtube.com/watch?v="));
    }
}
