# The admin dashboard is built on the existing stack, not the Master Plan's

*Decided 2026-11-13, in a grilling session on "Telosgard — Master Plan.md".*

The Master Plan describes a new system: a Hostinger VPS running Postgres, Drizzle, Fastify and a Next.js dashboard, with lessons stored as database rows. Telosgard was already live on Cloudflare (a Worker with D1, R2 and Better Auth). We kept that, and the dashboard joins it:

- **Hosting stays on Cloudflare** until the system is finished; the move to Hostinger comes after. New admin code uses plain SQL that also runs on Postgres and keeps Cloudflare-specific calls at the edges, so the move is a port, not a rewrite.
- **One API:** the dashboard calls the same Worker under `/api/admin/*`, with role checks on every route. Nothing else touches D1.
- **The screens are Leptos** (`crates/admin`, its own WASM bundle served at `/admin`), so Learners never download dashboard code and it reuses the content and core crates directly. Shared tokens and controls live in `crates/ui`.
- **Lessons stay Markdown in the repo**, checked at build time (structure, word limits, French parity, widgets, drills, sources). Merging to `main` deploys automatically, with a preview link per pull request; that is how a lesson is "published from the phone". The dashboard **shows and steers** lessons (status, pass rates, Explain-Back answers, hiding) but never edits their text.
- **Activity history comes from server-recorded events:** when the app saves a document, the Worker compares it with the stored version and appends events (`lesson_passed`, `quiz_answered`, …) to an `events` table. The student app is unchanged.

## Considered Options

- **Move to the VPS first, as planned.** Rejected for now: weeks of porting (database, API, auth, backups, deploys) and server upkeep before the first dashboard screen.
- **Lessons in the database with an in-app editor.** Rejected for now: the editor plus re-implementing every build-time check would be the biggest single piece of work, and widgets and drills are code that needs a deploy anyway. Revisit if other Instructors write lessons.
- **React SPA (Vite, shadcn/ui, TanStack Table)** or **Next.js.** The Owner chose Leptos. Next.js would also have needed its own server runtime, a second backend.
- **Events sent by the app** (more code paths that can disagree) or **no events** (no funnels, activation or trends).

## Consequences

- The dashboard lives at `/admin` on `telosgard.emmaadjam.workers.dev` until there is a domain; it then moves to `admin.<domain>` behind Cloudflare Access. Admin routes keep one prefix so the move is a routing change.
- Event history starts when it ships; earlier activity is backfilled only from timestamps already stored (approximate).
- Grading still runs in the browser ([ADR 0002](0002-client-side-grading-trust-model.md)), so dashboard numbers are as trustworthy as the app that sends them: fine for teaching analytics, never for money.
