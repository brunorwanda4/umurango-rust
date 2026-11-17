# Rust And Actix Implementation Guide

## Backend Baseline

The backend uses Actix-web with Tokio and SQLx PostgreSQL. Keep route modules
organized by domain as the API grows. The current `handlers/families.rs` is a
small scaffold and should move toward a handler/service/repository split when
family operations become stateful.

## Request Flow

1. Parse the request with Serde.
2. Authenticate the caller.
3. Resolve the requested family and membership.
4. Validate domain constraints, including integer money bounds.
5. Execute a transaction where multiple writes must be atomic.
6. Return a stable DTO and synchronization metadata.

## Synchronization

`/api/v1/sync/push` accepts idempotent mutations. The server must persist or
recognize the idempotency key before returning success. `/api/v1/sync/pull`
returns records after a client cursor and must enforce the requested family
scope.

Conflicts must be explicit. Never silently overwrite a newer authoritative
financial record. Return enough metadata for the native app to show and
resolve the conflict.

## Database

Use migrations for schema changes. Add `family_id` indexes to family-owned
tables. Store money as integer columns with a documented currency/minor-unit
contract. Use UTC timestamps and stable UUID identifiers.

## Security

Use Argon2 for password hashing and signed JWTs only after the authentication
contract is defined. Keep secrets in environment or deployment secret stores.
Do not log credentials, tokens, or full financial payloads.
