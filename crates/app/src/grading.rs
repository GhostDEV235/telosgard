//! Explain-Back grading call: prompt from `gold_core::grading`, one `sample.json` call, strict check.
//! No automatic retry on an unreadable reply (the platform asks pages not to); the UI offers "Try again".

use crate::platform::{self, PlatformError};
use crate::rubrics::Rubric;
use gold_core::grading::{GradeParseError, GradeResult, build_prompt, parse_grade};

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

pub async fn grade(rubric: &Rubric, answer: &str) -> Result<GradeResult, GradeError> {
    let prompt = build_prompt(&rubric.input(answer));
    let value = platform::sample_json(&prompt, "default").await.map_err(GradeError::Platform)?;
    parse_grade(&value).map_err(GradeError::Unreadable)
}
