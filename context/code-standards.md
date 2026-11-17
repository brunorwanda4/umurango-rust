# Code Standards

## General

- Prefer the smallest clear implementation that fully solves the task.
- Read callers, schemas, tests, and configuration before changing behavior.
- Keep one responsibility per function and cohesive modules.
- Extract shared logic when a second caller appears; do not build speculative
  abstraction layers.
- Avoid comments that narrate obvious code. Keep comments for invariants,
  security boundaries, compatibility decisions, or non-obvious tradeoffs.

## Rust

- Run `cargo fmt --all -- --check` and the smallest relevant `cargo check`.
- Prefer concrete types and domain enums over strings for controlled values.
- Return errors explicitly; do not hide database, network, or authorization
  failures behind empty success responses.
- Use SQLx bound parameters and migrations for database changes.
- Keep API handlers thin and place business rules in testable functions.
- Use `i64` minor units for all financial amounts.

## API And Security

- Authenticate before accessing protected family data.
- Verify the caller belongs to the requested family.
- Check the permission required for the exact operation.
- Validate ownership and cross-family references.
- Reject malformed or unknown mutation fields where the API contract requires it.
- Do not log passwords, tokens, database URLs, or financial data unnecessarily.

## Web

- Use Bun commands and `bun.lock`.
- Use TypeScript strict mode.
- Run `bun run typecheck` and `bun run lint`.
- Prefer server components for read-only pages and explicit client components
  only for browser state or interaction.
- Use existing daisyUI/Tailwind patterns before adding custom CSS.
