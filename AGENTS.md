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
- Keep handlers, screens, and modules focused; reuse existing patterns.
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
