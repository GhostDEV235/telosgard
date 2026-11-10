//! Pure domain logic for Gold School. No web, no async, no floats in rules.
//! Vocabulary follows CONTEXT.md at the repo root.

pub mod checklist;
pub mod course;
pub mod drills;
pub mod gate;
pub mod journal;
pub mod money;
pub mod plan;
pub mod readiness;
pub mod review;
pub mod sizing;
pub mod time;

/// Milliseconds since the Unix epoch (what JavaScript's `Date.now()` returns).
pub type Millis = i64;

pub const DAY_MS: Millis = 86_400_000;
