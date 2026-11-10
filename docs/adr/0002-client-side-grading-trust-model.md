# Grading runs in the Learner's browser and is trusted

Explain-Back **Grades** and **Adherence Verdicts** are produced in the browser (the page calls Claude through the Artifact `sample` capability) and written to the Learner's private store by the page itself. A determined Learner could therefore forge a "Got it" or an adherent verdict with browser dev tools. We accept this: there is one Learner, the course exists for their benefit, and forging a pass only cheats themselves. The alternative, a server that grades and signs results, would need a backend and an API key, which [ADR 0001](0001-rust-wasm-on-claude-artifact.md) deliberately avoided.

## Consequences

- Hard Rules are computed deterministically in Rust, not by Claude, so the numeric checks at least are reproducible from the stored Journal Entry.
- If the course is ever offered to other people, this decision must be revisited before any certificate or shared leaderboard exists.
