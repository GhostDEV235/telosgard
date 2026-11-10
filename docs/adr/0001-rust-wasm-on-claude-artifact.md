# Rust (Leptos → WebAssembly) published as a claude.ai Artifact

The app is written in Rust with Leptos, compiled to WebAssembly and styled with compiled Tailwind. It is published as a claude.ai Artifact rather than run as a local server. The **Learner** prefers Rust. Publishing as an Artifact keeps the features that need Claude: Explain-Back grading and Adherence Verdicts use the page's `sample` capability, and progress, Journal Entries and Trading Plan versions use the `db` capability. These need no API key and no server on the Learner's Mac. The platform's `claude.use()` API is JavaScript-only, so Rust reaches it through a thin `wasm-bindgen` bridge.

## Considered Options

- **Axum server on localhost.** Rejected: Claude grading would need the Learner's own paid API key, and the app would only work while the Mac runs it.
- **Rust for the logic only, with a plain JS/HTML interface.** Rejected: most of the code would not be Rust, which defeats the preference.

## Consequences

- Before any lesson content is written, a proof of concept must confirm that a `.wasm` file loads and calls `claude.use()` on claude.ai. If it fails, return to the Learner before switching architecture.
- No Rust toolchain was installed when this was decided; it must be set up first (rustup, `wasm32-unknown-unknown`, Trunk).
