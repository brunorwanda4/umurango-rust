# Progress Tracker

This file records meaningful repository progress. Keep entries concise, newest
entries at the end, and include validation.

## 2026-11-17

- Scaffolded the Rust workspace with `crates/shared`, `backend`, and `app`.
- Added shared family, transaction, expense, contribution, money, and sync
  contracts.
- Added the initial Actix-web backend with PostgreSQL pool configuration,
  `/health`, `/api/v1/families`, and a family migration.
- Added the GPUI native app entrypoint and local SQLite outbox foundation.
- Added the Next.js 15 web portal with Tailwind 4 and daisyUI.
- Standardized web tooling on Bun and Oxlint; `bun run check` passes.
- Rust formatting and workspace crate checks pass.
- Upgraded web dependencies to current Next.js 16, React 19.3, Tailwind 4.3,
  TypeScript 7, daisyUI 5.8, and Oxlint 1.94.
- Upgraded Rust dependencies to current Actix-web 4.20 and SQLx 0.9. GPUI
  remains at its latest published 0.2.2; rusqlite uses 0.37 for native SQLite
  link compatibility with SQLx 0.9.
- Established the cross-client rule: reusable deterministic Rust logic created
  for `app/` is exposed to `web/` through a narrow WebAssembly boundary.
- Implemented native GPUI Sign In page in `app/src/login.rs` and brand definitions
  in `app/src/brand.rs` using official brand colors (emerald #059669, navy #0a0f1d,
  gold #d97706, sky #0284c7) and brand icons. Integrated authentication flow and
  screen switching between Sign In and Dashboard in `app/src/main.rs`.
- Validated via `cargo check -p umuryango-app`, `cargo fmt --all -- --check`,
  `cargo test -p umuryango-shared -p umuryango-backend`, and `bun run check`.
- Added vertical spacing between the login remember-me checkbox and sign-in
  button.
- Added an explicit Font Awesome check icon and controlled checked state to
  the login remember-me checkbox.
- Positioned login validation errors immediately above the Sign In button.
- Replaced the Windows application icon with the Umuryango family-house brand
  icon in `assets/brand/umuryango-app.ico`.

## 2026-11-18

- Extracted reusable GPUI form components (`app/src/components/input.rs`,
  `app/src/components/checkbox.rs`) and shared auth section helpers in
  `app/src/auth/layout.rs`.
- Implemented the native registration view in `app/src/auth/register.rs` with
  name, phone number, and password inputs, localized strings (`en.json` and
  `rw.json`), and Login/Register screen navigation in `app/src/main.rs`.
- Added localized page titles (`Login` / `Create account`) to the shared
  `auth_layout` for both `LoginView` and `RegisterView`.
- Updated `RegisterView` (`app/src/auth/register.rs`) to require explicit
  consent to the Umuryango Terms & Charter via `checkbox_with_link` and added
  the Terms & Charter screen (`app/src/auth/terms.rs`).
- Added the post-registration Family Setup screen (`app/src/auth/family_setup.rs`)
  with **Create Family**, **Join Family**, and **Skip** actions, wired in
  `app/src/main.rs`.
- Added vertical scrolling (`touch_scrollable` in `app/src/components/scroll.rs`)
  supporting finger touch drag (`TouchDragEvent`), pointer drag, and wheel
  scrolling to `TermsView` (`app/src/auth/terms.rs`), and updated the Umuryango
  Terms & Charter copy (free to use, no data used for profit, restricted family
  access, and as-is no-liability disclaimer) in `en.json` and `rw.json`.
- Validated via `cargo fmt --all -- --check`,
  `cargo test -p umuryango-app --lib` (linking GUI test binaries requires host
  `libxkbcommon-x11`).
- Generated full PostgreSQL database schema DDL from `schema.prisma` in
  `backend/schema.sql` and `backend/migrations/0002_prisma_schema.sql` covering
  all 38 enum types, 31 tables, foreign key constraints, and indexes. Preserved
  `schema.prisma` in `backend/schema.prisma`.
- Implemented `tokio-postgres` schema management module (`backend/src/schema.rs`)
  with `apply_schema`, `connect`, `init_db`, and `list_tables`.
- Added `init-schema` CLI binary in `backend/src/bin/init_schema.rs` and library
  exports in `backend/src/lib.rs`.
- Added integration test `backend/tests/schema_test.rs` validating schema
  execution and table presence against PostgreSQL via `tokio-postgres`.
- Added `.env.example` at root workspace and in `backend/.env.example` documenting
  `DATABASE_URL`, `TEST_DATABASE_URL`, `BIND_ADDRESS`, and `NEXT_PUBLIC_API_URL`,
  and updated `.gitignore` to prevent committing local `.env` files.
- Validated via `cargo fmt --all -- --check`, `cargo check -p umuryango-backend`,
  and `cargo test -p umuryango-shared -p umuryango-backend`.
- Implemented backend authentication system in `backend/src/auth/` with modular
  architecture: Argon2id password hashing (`password.rs`), JWT authentication
  (`jwt.rs`), validation rules and DTOs (`dto.rs`), and service logic
  (`service.rs`) handling registration, login, session tracking, primary family
  workspace auto-provisioning, and audit logging.
- Added endpoints `/api/v1/auth/register`, `/api/v1/auth/login`, and `/api/v1/auth/me`
  in `backend/src/handlers/auth.rs`.
- Added interactive OpenAPI documentation with Swagger UI via Utoipa available at
  `/swagger-ui/` and `/api-docs/openapi.json` (`backend/src/openapi.rs`).
- Made `User.email` nullable in PostgreSQL schema to allow phone-based user registration
  without extraneous fields.
- Connected native GPUI app login and register views (`app/src/auth/login.rs` and
  `app/src/auth/register.rs`) with backend authentication via lightweight HTTP client
  (`app/src/auth/client.rs`) while preserving the exact UI design and inputs.
- Validated via `cargo fmt --all -- --check`, `cargo check -p umuryango-backend`,
  `cargo check -p umuryango-app`, `cargo test -p umuryango-app --lib`, and
  `cargo test -p umuryango-shared -p umuryango-backend`.


## 2026-11-18

- Added local `.env` files for workspace and backend PostgreSQL configuration
  using the local `postgres` password, and created `umuryango` and
  `umuryango_test` databases.
- Initialized the main database schema successfully; verified 31 public tables
  are present.
- Implemented Create Family page (`app/src/auth/create_family.rs`) allowing users to create
  a family workspace with family name and invite household members by phone number.
- Implemented Join Family page (`app/src/auth/join_family.rs`) using GPUI-kit's `OtpInput`
  component with 4 uppercase alphanumeric code cells (e.g. "HO92") and auto-focus handling.
- Updated `FamilySetupView` (`app/src/auth/family_setup.rs`) to display pending family
  invitations when members were added by phone number, allowing direct "Accept & Join".
- Added backend family endpoints (`POST /api/v1/families`, `POST /api/v1/families/join`,
  `GET /api/v1/families/invitations`, `POST /api/v1/families/invitations/{id}/accept`),
  unique 4-character join code generator (`generate_join_code`), and OpenAPI Swagger specs.
- Updated database schema with `Family.joinCode` and `FamilyInvite.phone`.
- Wired screens and navigation in `app/src/main.rs`.
- Validated via `cargo fmt --all -- --check`, `cargo check --workspace`,
  `cargo test -p umuryango-shared -p umuryango-backend`, and `cargo test -p umuryango-app --lib`.
- Added `bun run dev:backend`, a Bun-based backend development watcher that
  restarts the Actix server after backend or shared-crate changes and uses
  Cargo incremental builds for fast subsequent startup.
- Updated the local backend and API client defaults to port `4433`.
- Implemented backend multilingual error response localization supporting English (`en`) and Kinyarwanda (`rw`).
- Added backend `i18n` module (`backend/src/i18n.rs`) with `extract_language` supporting `Accept-Language` headers, custom `x-lang` headers, and `?lang=` query parameters.
- Expanded localization dictionaries (`assets/locales/en.json`, `assets/locales/rw.json`) with localized error strings and mapped backend `ServiceError` variants to locale keys.
- Updated `error_to_response` in `backend/src/handlers/mod.rs` and all endpoints in `auth.rs` and `families.rs` to inspect request locale and translate errors accordingly.
- Integrated `set_current_language` and `get_current_language` into app HTTP client (`app/src/auth/client.rs`) to pass `Accept-Language` headers on every backend API call (`api_login`, `api_register`, `api_create_family`, `api_join_family`, `api_get_pending_invitations`, `api_accept_invitation`).
- Synchronized app language preferences with the client on launch and language toggle events in `app/src/main.rs`.
- Validated with unit tests for language extraction and error translations in both English and Kinyarwanda.
- Fixed phone-based account registration by making `User.email` nullable and
  aligning live `Family.joinCode` and `FamilyInvite.phone` columns with the
  Prisma and PostgreSQL schema. Verified registration with a real HTTP request.
- Added concurrent development script (`scripts/dev-all.js`) and root scripts
  in `package.json` (`dev:all`, `dev:both`, `dev:fullstack`, `dev:native:all`,
  `dev:frontend`, `dev:web`) to run backend and frontend services concurrently
  with color-coded log prefixing, unified signal handling (`Ctrl+C`), and
  options for web portal (`--cwd web dev`) or native desktop app (`--native`).
  Updated `README.md` and `.gitignore`.
