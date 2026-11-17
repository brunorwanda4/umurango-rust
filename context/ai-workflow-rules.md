# AI Workflow Rules

## Before Editing

Read `AGENTS.md`, the ordered context files, and the files directly related to
the request. Search for existing types, routes, helpers, and tests before
introducing new ones.

## During Editing

- Implement the requested scope, not an imagined roadmap.
- Preserve unrelated work in a dirty tree.
- Use `apply_patch` for focused edits.
- Keep generated dependencies and lockfiles consistent with the package manager.
- Never commit credentials, secrets, private keys, or production connection
  strings.
- Do not change money representation to make a UI or database operation easier.
- Do not skip family scope or authorization because an endpoint is currently
  used only by an internal client.

## Verification

Run the smallest useful checks and inspect their output. For Rust changes,
format and check the affected crate. For web changes, run TypeScript and
Oxlint. For API behavior, add or run focused endpoint tests.

State blocked, incomplete, or environment-dependent validation plainly. Do not
claim a build passed when it was not run.

## Documentation

Update the relevant context document when a rule, architecture decision,
workflow, or product boundary changes. Append a concise entry to the progress
tracker for meaningful completed work.
