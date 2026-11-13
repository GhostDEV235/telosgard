# Staff roles: Owner by secret only, two-factor for every staff sign-in

*Decided 2026-11-13. Extends [ADR 0004](0004-better-auth-invite-only.md).*

Four roles: **Owner**, **Instructor**, **Support** and **Learner** (see `CONTEXT.md` for what each may do). The Owner is never stored as data: it comes only from the `OWNER_EMAILS` secret, so no bug or admin action can create a second Owner. Instructor and Support are a `role` column on the user, set by the Owner in the dashboard. **Full Access** stops being the `FULL_ACCESS_EMAILS` secret and becomes a per-Learner switch, because it is not a role (a reviewer is still a Learner). Every permission is checked in the Worker by name; the UI only hides what the API already refuses.

Because the dashboard sits at `/admin` on the workers.dev address with no Cloudflare Access in front (no domain yet), sign-in is the only barrier, so **every staff account needs two-factor** (Better Auth's TOTP plugin, ten backup codes). Without it, `/admin` shows only the setup screen. Dashboard sessions last 12 hours, and sensitive actions (delete, export, roles, Full Access) ask for the code again. Learners never need two-factor.

## Considered Options

- **Owner as a role in the database.** Rejected: one compromised Owner session or bad query could mint more Owners.
- **Passkeys now.** Deferred: passkeys are bound to a domain, so they would have to be registered again after the move.

## Consequences

- Losing both the authenticator and the backup codes means a hand-run recovery command for the Owner; there is no email route yet.
- Instructors never see who a Learner is; Support never reads Journal Entries.
