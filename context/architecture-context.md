# Architecture Context

## System Shape

```text
GPUI app -> local SQLite + sync_outbox -> Actix-web API -> PostgreSQL
Next.js web portal ---------------------> Actix-web API
                 crates/shared contracts
```

The backend is the trust boundary. Clients may optimize UX and cache data, but
they must not be trusted for family membership, role, amount, ownership, or
authorization decisions.

## Data Ownership

- PostgreSQL is the authoritative remote store.
- SQLite is a local cache and pending mutation queue.
- Every family-owned table has a `family_id` and appropriate database indexes.
- A mutation has a stable idempotency key before it leaves the client.
- Replaying an accepted mutation returns the original result rather than
  creating a duplicate financial record.

## Rust Conventions

- Rust workspace members are `crates/shared`, `backend`, and `app`.
- Backend uses Actix-web, Tokio, SQLx, and PostgreSQL.
- Native app uses GPUI and rusqlite; encryption must be preserved when the local
  database implementation moves from scaffold to production.
- Shared DTOs and enums belong in `crates/shared`, not duplicated per client.

## Backend Layers

Keep HTTP handlers thin:

1. Extract and authenticate the request.
2. Validate the DTO and family scope.
3. Call a focused service or domain operation.
4. Persist through SQLx repositories/queries.
5. Return the established response or error envelope.

Do not construct SQL by concatenating request data. Use bound SQLx parameters
and committed migrations.

## Web Runtime

The web portal is Next.js 16 with the App Router, TypeScript, Tailwind CSS 4,
daisyUI, Bun, and Oxlint. Bun is the package manager and command runtime for
web development; the API remains a Rust service.
