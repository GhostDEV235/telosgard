//! Explain-Back grading call: prompt from `gold_core::grading`, one `sample.json` call, strict check.
//! No automatic retry on an unreadable reply (the platform asks pages not to); the UI offers "Try again".

use crate::platform::{self, PlatformError};
use gold_content::{Expect, Lesson};
use gold_core::gate::Grade;
use gold_core::grading::{GradeParseError, GradeResult, GradingInput, build_prompt, parse_grade};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GradeError {
    Platform(PlatformError),
    Unreadable(GradeParseError),
}

impl GradeError {
    pub fn describe(&self) -> String {
        match self {
            GradeError::Platform(PlatformError::NotGranted) => {
                "Claude grading isn't available here. Allow Claude for this page (or sign in), then try again. Nothing was graded.".into()
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
pub async fn judge(input: &gold_core::adherence::AdherenceInput<'_>) -> Result<gold_core::adherence::AdherenceResult, String> {
    let prompt = gold_core::adherence::build_adherence_prompt(input);
    let v = platform::sample_json(&prompt, "default").await.map_err(|e| GradeError::Platform(e).describe().replace("graded", "judged"))?;
    gold_core::adherence::parse_adherence(&v).map_err(|_| "Claude's verdict didn't come back in the expected shape. Nothing was judged; try again.".to_string())
}
