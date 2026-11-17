# Project Overview

## Product

Umuryango is a shared family finance workspace. A family can record income,
expenses, contributions, payments, budgets, and communal projects while
preserving a clear history of who did what and when.

The product should support households with intermittent connectivity. A member
must be able to inspect recently available records and create permitted
mutations while offline; those mutations are synchronized later.

## Applications

- `app/`: native GPUI desktop/mobile client. It owns the local-first user
  experience and SQLite cache/outbox.
- `backend/`: Actix-web API. It owns authentication, authorization, validation,
  conflict handling, synchronization, and PostgreSQL persistence.
- `web/`: Next.js portal. It provides browser access to the same API and
  consumes WebAssembly built from reusable Rust domain logic.
- `crates/shared/`: Rust contracts shared by the native app and backend.

## Initial Domain

The shared crate currently defines `Family`, `Member`, `Transaction`, `Expense`,
`Contribution`, money minor units, command types, and sync push/pull DTOs.

The first backend endpoints are `/health` and `/api/v1/families`. They are
scaffolding, not the complete product API.

## Cross-Client Rule

When a feature is first implemented in `app/` and has deterministic domain or
financial logic that belongs in the browser, implement that logic once in a
WASM-compatible Rust layer and consume it from `web/`. Native-only UI, SQLite,
filesystem, and background synchronization remain app-specific.

## Product Boundaries

The platform is for family financial governance and communal management. Do not
silently turn it into a general banking, lending, investment, payroll, or
government platform without an explicit product decision.
