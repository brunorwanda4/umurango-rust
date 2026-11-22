# Architecture Context

## System Shape

```text
GPUI app -> local SQLite + sync_outbox -> Actix-web API -> PostgreSQL
Next.js web portal -> Rust WebAssembly domain layer -> Actix-web API
                 \-> crates/shared contracts
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

- Rust workspace members are `crates/shared`, `backend`, `app`, and `crates/mobile`.
- Backend uses Actix-web, Tokio, SQLx, and PostgreSQL.
- Native app uses GPUI (`app/`) and rusqlite, sharing its UI and domain library across desktop (`app/src/main.rs`) and Android (`crates/mobile/src/lib.rs`); encryption must be preserved when the local database implementation moves from scaffold to production.
- Shared DTOs and enums belong in `crates/shared`, not duplicated per client.
- Pure, deterministic domain logic created for `app/` must be organized so it
  can compile to WebAssembly for `web/`. GPUI, SQLite, filesystem, and native
  synchronization workers stay outside the WebAssembly boundary.
- WebAssembly improves consistency and client-side computation; it does not
  replace backend authorization, validation, or authoritative persistence.

## WebAssembly Boundary

When a native app capability also belongs in the web portal:

1. Keep the reusable calculation or domain operation in a Rust crate with no
   native-only dependencies.
2. Expose a narrow `wasm-bindgen` API with serializable inputs and outputs.
3. Package the generated module for `web/` and keep the TypeScript wrapper
   typed and small.
4. Keep secrets, family permission checks, sync acceptance, and final money
   writes on the backend.

Do not compile the GPUI UI or local SQLite layer to WebAssembly as a shortcut.

## Backend Layers

Keep HTTP handlers thin:

1. Extract and authenticate the request.
2. Validate the DTO and family scope.
3. Call a focused service or domain operation.
4. Persist through SQLx repositories/queries.
5. Return the established response or error envelope.

Do not construct SQL by concatenating request data. Use bound SQLx parameters
and committed migrations.

## Backend Deployment (Render & Neon)

- `backend/src/config.rs` resolves `DATABASE_URL` (falling back to
  `PRODUCTION_DATABASE_URL`) and `BIND_ADDRESS` (falling back to `0.0.0.0:$PORT`
  when `PORT` is injected by Render).
- On startup, `umuryango-backend` warms the database pool and ensures the
  idempotent schema (`backend/schema.sql`) is applied via SQLx over TLS
  (`tls-rustls-ring`), supporting managed PostgreSQL providers such as Neon
  (`?sslmode=require&channel_binding=require`).
- `render.yaml` defines the Render Blueprint for `umuryango-backend` with
  `/health` health checks.

## Backend Request Logging

- Actix request logging middleware prints method, status, client IP, route,
  elapsed milliseconds, and slow-request markers.
- Successful `/health` checks stay quiet; errors and all API routes are logged.
- Request middleware must not log authorization tokens, passwords, request
  bodies, or financial values.

## Web Runtime

The web portal is Next.js 16 with the App Router, TypeScript, Tailwind CSS 4,
daisyUI, Bun, and Oxlint. Bun is the package manager and command runtime for
web development; the API remains a Rust service.

## Native Local State

- The `app_state` SQLite table stores small native UI state as key/value
  pairs (for example `navigation.last_screen`).
- Session tokens are only persisted while the user is on a screen that requires
  authentication and are removed on navigation to public screens. This store
  is unencrypted for now and must move under the local database encryption
  work before production.

## Mobile Safe Area & Insets

- Android and mobile platforms draw edge-to-edge behind system chrome.
- System insets (status bars, camera cutouts, and 3-button navigation footers) are forwarded from Android's ViewCompat.setOnApplyWindowInsetsListener via JNI (Java_dev_gpui_mobile_GpuiActivity_nativeSetInsets).
- Insets are stored in physical pixels in `app/src/safe_area.rs` and dynamically scaled to logical pixels via window.scale_factor().
- Bottom insets are applied to the `app-footer` navigation bar and non-dashboard auth views to ensure interactive controls remain completely clear of system footers.

## Native App Self-Update & Release Pipeline (Windows & Android)

- `app/src/updater.rs` checks GitHub Releases (`https://api.github.com/repos/brunorwanda4/umurango-rust/releases/latest`, overridable via `UMURYANGO_UPDATE_REPO` or `UMURYANGO_UPDATE_URL`) against the compiled `CARGO_PKG_VERSION`.
- When a newer semantic version is found, `DashboardView` surfaces a notification banner and an interactive Update & Restart modal (`app/src/components/dashboard/update_modal.rs`), plus an Application Updates card in the Profile screen.
- **Desktop (Windows `.exe`)**: On confirmation, the updater downloads `umuryango-app-windows-x86_64.exe` into `<exe>.new.tmp`, renames the running executable to `<exe>.old`, moves the new binary into place, spawns the updated binary, and exits; `cleanup_old_binary()` removes `<exe>.old` on the next launch.
- **Mobile (Android `.apk`)**: `crates/mobile/src/lib.rs` registers an `AndroidUpdateInstaller` (`PlatformUpdateInstaller` trait in `app/src/updater.rs`). On confirmation, the updater downloads `umuryango-app-android-arm64.apk` into Android's `FileProvider` update cache directory (`MainActivity.updateCachePath()`) and invokes `MainActivity.requestInstallUpdate(...)` via JNI to launch the Android system package installer (or prompt for unknown-sources install permission).
- **Build & Release Automation**:
  - Local build scripts (`scripts/build-release.js`): `bun run build:windows`, `bun run build:android`, and `bun run build:all` output release artifacts and `.sha256` checksums into `dist/`.
  - `.github/workflows/release.yml` automatically builds both `umuryango-app-windows-x86_64.exe` and `umuryango-app-android-arm64.apk` in parallel and publishes them to a GitHub Release when a new version tag (`v*.*.*`) or bumped `Cargo.toml` version is pushed (`bun run release`).
