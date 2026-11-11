# Telosgard moves from a claude.ai Artifact to Cloudflare (Pages, a Worker, D1, Access)

*Decided 2026-11-11. Supersedes the hosting part of [ADR 0001](0001-rust-wasm-on-claude-artifact.md); the Rust → WebAssembly app itself stays.*

Telosgard is served from **Cloudflare Pages**. A **Worker** (Pages Functions) sits behind it with three jobs: store the Learner's documents in **Cloudflare D1**, keep chart screenshots in **R2**, and call the **Anthropic API** for grading, verdicts, practice questions and Ask Claude, holding the API key as a secret. **Cloudflare Access** guards the whole site (an email one-time code or Google), and the Worker trusts only the identity Access signs. The claude.ai Artifact is frozen at version 21 and no longer updated.

The Learner asked for Postgres, but wants nothing outside Cloudflare. Cloudflare doesn't host Postgres: D1 is SQLite underneath, and Hyperdrive only connects to a Postgres hosted elsewhere. So D1 was chosen, and PlanetScale Postgres billed through Cloudflare was turned down.

## Considered Options

- **Keep the Artifact and add Cloudflare beside it.** Rejected: two backends to test on every change, and progress wouldn't be shared between them.
- **PlanetScale Postgres via Hyperdrive, billed through Cloudflare.** Rejected: it's Postgres, but operated by PlanetScale and paid; the Learner wants Cloudflare only.
- **Full in-app accounts (Clerk, Supabase Auth).** Deferred: Access is enough while it's the Learner and a few testers, and needs no sign-in code. Revisit before selling to customers.

## Consequences

- The app keeps one seam to its platform: `bridge.js` gets a Cloudflare twin with the same exports (`init`, `dbGet/Set/Delete/List`, `sampleJson`, `sampleText`, `uploadFromInput`), so the Rust code and all course logic stay unchanged.
- D1 stores the same kind-tagged documents as before (`user_id`, `doc_id`, `kind`, `body` as JSON, `updated_at`), so the Learner's saved progress migrates as-is and can later move to normalised tables or to Postgres behind the Worker.
- Claude calls now cost money on the Learner's Anthropic account. The Worker only accepts calls from a signed-in user, uses a fixed model per call type, caps output length, and rate-limits each user.
- Claude grading still runs on prompts built in the client ([ADR 0002](0002-client-side-grading-trust-model.md) stands): the Learner could tamper with their own grades, and that is accepted.
- Steps only the Learner can do: create the Cloudflare account and log in with `wrangler`, create an Anthropic API key and store it as a Worker secret, and turn on Access for the site.
