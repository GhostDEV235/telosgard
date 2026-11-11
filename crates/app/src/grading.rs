//! Explain-Back grading call: prompt from `telosgard_core::grading`, one `sample.json` call, strict check.
//! No automatic retry on an unreadable reply (the platform asks pages not to); the UI offers "Try again".

use crate::platform::{self, PlatformError};
use telosgard_content::{Expect, Lesson};
use telosgard_core::gate::Grade;
use telosgard_core::grading::{GradeParseError, GradeResult, GradingInput, build_prompt, parse_grade};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GradeError {
    Platform(PlatformError),
    Unreadable(GradeParseError),
}

impl GradeError {
    pub fn describe(&self) -> String {
        match self {
            GradeError::Platform(PlatformError::NotGranted) => {
                "Claude grading isn't available right now: you may be signed out, or Claude isn't set up on the server yet. Nothing was graded.".into()
            }
            GradeError::Platform(e) => format!("{} Nothing was graded.", e.describe()),
            GradeError::Unreadable(_) => "Claude's reply didn't come back in the expected shape. Nothing was graded; try again.".into(),
        }
    }
}

pub fn input<'a>(l: &'a Lesson, answer: &'a str) -> GradingInput<'a> {
    GradingInput {
        title: l.title,
        objective: l.objective,
        prompt: l.explain.prompt,
        analogy: l.analogy,
        must_include: l.explain.must_include,
        misconceptions: l.explain.misconceptions,
        answer,
    }
}

pub fn accepts(e: Expect, g: Grade) -> bool {
    match e {
        Expect::GotIt => g == Grade::GotIt,
        Expect::Almost => g == Grade::Almost,
        Expect::AlmostOrNotYet => g != Grade::GotIt,
    }
}

pub fn expect_label(e: Expect) -> &'static str {
    match e {
        Expect::GotIt => "Got it",
        Expect::Almost => "Almost",
        Expect::AlmostOrNotYet => "Almost / Not yet",
    }
}

pub async fn grade(lesson: &Lesson, answer: &str) -> Result<GradeResult, GradeError> {
    let prompt = build_prompt(&input(lesson, answer));
    let value = platform::sample_json(&prompt, "default").await.map_err(GradeError::Platform)?;
    parse_grade(&value).map_err(GradeError::Unreadable)
}

/// One Adherence Verdict (or a dispute re-judgement) from Claude. Never assumes a pass.
pub async fn judge(input: &telosgard_core::adherence::AdherenceInput<'_>) -> Result<telosgard_core::adherence::AdherenceResult, String> {
    let prompt = telosgard_core::adherence::build_adherence_prompt(input);
    let v = platform::sample_json(&prompt, "default").await.map_err(|e| GradeError::Platform(e).describe().replace("graded", "judged"))?;
    telosgard_core::adherence::parse_adherence(&v).map_err(|_| "Claude's verdict didn't come back in the expected shape. Nothing was judged; try again.".to_string())
}

/// A Claude-written practice round (5 questions) on these lessons. Never counts towards a Gate.
pub async fn practice(lessons: &[&Lesson], avoid: &[String]) -> Result<Vec<telosgard_core::practice::PracticeQ>, String> {
    use telosgard_core::practice::{PracticeSource, build_practice_prompt, parse_practice};
    let sources: Vec<PracticeSource> = lessons
        .iter()
        .map(|l| PracticeSource { lesson: l.id, title: l.title, objective: l.objective, takeaways: l.takeaways, key_ideas: l.explain.must_include })
        .collect();
    let avoid: Vec<&str> = avoid.iter().map(String::as_str).collect();
    let ids: Vec<u8> = lessons.iter().map(|l| l.id).collect();
    let prompt = build_practice_prompt(&sources, 5, &avoid);
    let v = platform::sample_json(&prompt, "default")
        .await
        .map_err(|e| GradeError::Platform(e).describe().replace("Claude grading isn't available here", "Claude isn't available here").replace("Nothing was graded.", "No questions were written."))?;
    parse_practice(&v, 5, &ids, &avoid).map_err(|_| "Claude's questions didn't come back in a usable shape. Try again.".to_string())
}

/// "Ask Claude about this step": one plain-text tutor call. Errors are messages to show; there is
/// never a stand-in answer. Returns the answer and whether the platform cut it short.
pub async fn ask_step(l: &Lesson, step: usize, question: &str) -> Result<(String, bool), String> {
    use telosgard_core::tutor::{TutorInput, TutorStep, build_tutor_prompt, clean_answer, plain_text};
    let text = |i: usize| plain_text(l.steps[i].text_html);
    let (cur, prev) = (text(step), step.checked_sub(1).map(text));
    let input = TutorInput {
        lesson: l.id,
        title: l.title,
        objective: l.objective,
        current: TutorStep { heading: l.steps[step].heading, text: &cur },
        previous: step.checked_sub(1).zip(prev.as_deref()).map(|(i, t)| TutorStep { heading: l.steps[i].heading, text: t }),
        question,
    };
    match platform::sample_text(&build_tutor_prompt(&input), "default").await {
        Ok(r) => clean_answer(&r.text).map(|a| (a, r.truncated)).ok_or_else(|| "Claude sent back an empty answer. Try asking again.".to_string()),
        Err(PlatformError::NotGranted) => Err("Asking Claude isn't available right now: you may be signed out, or Claude isn't set up on the server yet.".into()),
        Err(e) => Err(e.describe()),
    }
}
