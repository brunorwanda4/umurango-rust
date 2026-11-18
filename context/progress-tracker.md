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
- Validated via `cargo fmt --all -- --check`,
  `cargo check -p umuryango-app --all-targets`, and
  `cargo test -p umuryango-app --lib` (linking GUI test binaries requires host
  `libxkbcommon-x11`).


