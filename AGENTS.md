# Umuryango Agent Instructions

This is the active control file for human and AI contributors. Read it before
working, then read the context files in the order below:

1. `context/project-overview.md`
2. `context/architecture-context.md`
3. `context/ui-context.md`
4. `context/ui-clarity.md`
5. `context/ui-depth.md`
6. `context/ux-psychology.md`
7. `context/code-standards.md`
8. `context/typescript-safety.md`
9. `context/ai-workflow-rules.md`
10. The end of `context/progress-tracker.md`

## Non-Negotiable Rules

- Never use floating-point values for money. Use shared integer minor units.
- Scope every family-owned resource by `family_id`.
- Enforce membership and permissions in backend code for every protected
  operation.
- Treat PostgreSQL as authoritative and the native SQLite database as a local
  replica plus mutation outbox.
- Make sync operations idempotent and safe to retry.
- Use Bun for all JavaScript and TypeScript commands in `web/`.
- Use Oxlint and TypeScript checks before completing web changes.
- When a capability is implemented in `app/`, expose reusable pure Rust logic
  to `web/` through WebAssembly instead of duplicating financial or domain
  algorithms in TypeScript.
- Keep handlers, screens, and modules focused; reuse existing patterns.
- Less is more: prefer the smallest clear change, avoid over-engineering, and
  create a new component or abstraction only when it has a clear responsibility
  or a real reuse case.
- Keep related pages and reusable components organized by feature. Page
  modules may live under `app/src/<feature>/`, while shared feature
  components belong under `app/src/components/<feature>/`. When a feature
  grows beyond one component, create a dedicated feature folder instead of
  putting every component in one file. `dashboard` is only an example.
- Use the bundled Font Awesome Free SVG icons for native app UI. Add new
  icon constants in `app/src/icons.rs` and render them with
  `icons::icon(...)`; do not duplicate raw asset includes in screens.
- Font Awesome assets live under `assets/fontawesome/`. Paid Font Awesome Pro
  assets must not be added.
- Do not add speculative features or silently weaken validation.

## Working Method

1. Inspect the requested code and its callers before editing.
2. Keep the change limited to the requested scope.
3. Add focused tests for new branching, parsing, money, permissions, or writes.
4. Run the smallest relevant formatter, compiler, linter, or test command.
5. Update the relevant context document and progress tracker.
6. Report incomplete validation or known limitations explicitly.

## Commit Policy

- Commit completed and validated changes promptly after they happen.
- Keep commits focused and use a concise Conventional Commits message.
- Include all related source, tests, lockfiles, and documentation in the same
  commit.
- Do not commit unrelated work, secrets, generated build output, or incomplete
  changes.
- Do not amend an existing commit unless explicitly requested.

Do not treat this file as a product specification. Product behavior belongs in
the context documents and the implementation.
