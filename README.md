# Umuryango

Umuryango is a family financial governance and communal management platform.
It helps families manage contributions, expenses, payments, projects, and
shared financial decisions with transparent records and offline-first access.

## Repository Layout

```text
crates/shared/  Shared Rust entities, money types, and sync contracts
backend/        Actix-web API with PostgreSQL and SQLx
app/            GPUI native desktop/mobile client with local SQLite
web/            Next.js web portal using Bun, TypeScript, Tailwind, and daisyUI
context/        Project context for human and AI contributors
```

## Core Principles

- Money is always integer minor units (`i64`), never `f32` or `f64`.
- Every family-owned record is scoped by `family_id`.
- The native client is local-first and queues mutations in `sync_outbox`.
- PostgreSQL is authoritative for synchronized remote state.
- Sync mutations use idempotency keys and must be safe to retry.
- Authorization is enforced on the backend, not only in clients.

## Development

### Rust workspace

```powershell
cargo check --workspace
cargo fmt --all -- --check
cargo run -p umuryango-backend
cargo run -p umuryango-app
```

The backend defaults to `127.0.0.1:8080` and reads `DATABASE_URL` and
`BIND_ADDRESS` from the environment.

### Web portal

Use Bun for JavaScript and TypeScript tooling:

```powershell
cd web
bun install
bun run dev
bun run check
bun run build
```

The web API defaults to `http://localhost:8080/api/v1`. Override it with
`NEXT_PUBLIC_API_URL`.

## Context For Contributors

Read `AGENTS.md` before changing the repository. It points contributors to the
ordered documents under `context/`, including architecture, coding standards,
financial safety, UI guidance, and the progress tracker.
