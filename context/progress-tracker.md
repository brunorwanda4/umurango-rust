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
- Fixed family setup card text overflow by allowing card text columns to shrink
  and wrap beside their icons and trailing controls.
- Added Actix request logging middleware for timestamped method, status, client
  IP, route, duration, and slow-request console output. Successful `/health`
  checks are quiet, while API requests and error responses remain visible.
- Built reusable native GPUI UI component library in `app/src/components/ui/`
  modeled after `input.rs` and the design tokens in `brand.rs`:
  - `checkbox.rs`: `checkbox_indicator`, `checkbox`, `checkbox_with_description`,
    and `checkbox_with_link`.
  - `card.rs`: `card`, `card_surface`, `card_header`, `card_footer`, `interactive_card`,
    and standardized `option_card`.
  - `input.rs`: `text_input`, `password_input`, `search_input` (with magnifying glass prefix),
    `input_container`, `input_field`, `input_field_with_error`, and `password_field`.
  - `stat_card.rs`: Financial summary metric card with balance, subtitle, optional icon,
    and positive/negative trend indicators.
  - `badge.rs`: Status tags and pills with `BadgeVariant` (`Primary`, `Success`, `Warning`,
    `Error`, `Neutral`, `Info`).
  - `alert.rs`: `alert` and `dismissible_alert` banners with `AlertVariant` mapping to theme
    accents and icons.
  - `divider.rs`: `divider`, `vertical_divider`, and `divider_with_label`.
  - `avatar.rs`: Initial extraction, `avatar_initials`, and `member_avatar` with `AvatarSize`.
  - `modal.rs`: `modal_backdrop`, `modal_card`, `modal_header` with close button, and `modal_footer`.
  - `toggle.rs`: Animated switch components `toggle` and `toggle_field`.
  - `button.rs`: Reusable action buttons: `icon_action_button`, `secondary_button`, and `destructive_button``.
  - Added new Font Awesome Free icons in `app/src/icons.rs` (`ICON_CIRCLE_CHECK`, `ICON_CIRCLE_XMARK`,
    `ICON_CIRCLE_INFO`, `ICON_TRIANGLE_EXCLAMATION`, `ICON_MAGNIFYING_GLASS`, `ICON_WALLET`,
    `ICON_CREDIT_CARD`, `ICON_ARROW_UP`, `ICON_ARROW_DOWN`).
  - Preserved backward compatibility in `app/src/components/mod.rs` and refactored `app/src/auth/family_setup.rs` to use `option_card`.
  - Consolidated `input` and `checkbox` components exclusively under `app/src/components/ui/`, removing redundant wrapper files `app/src/components/input.rs` and `app/src/components/checkbox.rs`, and updated auth views to import directly from `components::ui`.
  - Validated via `cargo fmt --all -- --check`, `cargo check --workspace`, and `cargo test -p umuryango-app --lib`.
- Native app now reopens on the last visited auth screen. Navigation is saved
  to a new local SQLite `app_state` key/value table on every screen change
  (`app/src/navigation.rs`, `app/src/db/mod.rs`). Family setup, create
  family, and join family screens also persist the session token and member
  name so their authenticated calls keep working; these are deleted whenever
  the user moves to a screen that does not need them, and a missing token falls
  back to login. Covered by navigation and `app_state` unit tests.
- Split the native app entrypoint into focused `app/src/app.rs`,
  `app/src/window.rs`, and a minimal `app/src/main.rs` bootstrap without
  changing navigation or screen behavior.
- Replaced the Create Family text-only Add Member button with a compact plus
  icon button and localized tooltip/accessibility label. Applied the secondary
  outline variant so the button follows both dark and light themes.
- Built native Family Financial Dashboard page (`app/src/dashboard/mod.rs`)
  utilizing the existing UI component library (`stat_card`, `badge`, `alert`,
  `member_avatar`, `card_surface`, `secondary_button`, `search_input`, `touch_scrollable`):
  - Top header with family workspace name, active status badge, sync indicator,
    search bar, theme toggle, language toggle, user profile avatar, and sign-out button.
  - Four financial metric overview cards: Total Family Treasury, Monthly Contributions,
    Monthly Expenses, and Emergency Reserve with trend badges.
  - Quick action controls: Record Contribution, Add Expense, New Project, and Sync Now.
  - Left column: Recent Financial Activity with filter tabs (`All`, `Contributions`,
    `Expenses`) and Active Communal Projects with progress bars and contributor counts.
  - Right column: Pending Approvals card with review action, Household Members roster
    with avatars, and Local Outbox & SQLite Sync Health card.
  - Added `AuthScreen::Dashboard` to `navigation.rs` with session persistence,
    wired navigation flows in `app.rs`, and added full English (`en.json`) and
    Kinyarwanda (`rw.json`) localization dictionaries.
  - Validated via `cargo fmt --all -- --check`, `cargo check --workspace`, and all test suites.
- Split dashboard transaction and communal-project presentation into reusable
  components (`app/src/dashboard/components.rs`) and typed dashboard data
  definitions (`app/src/dashboard/data.rs`), reducing the screen module by
  roughly 300 lines without changing navigation or dashboard behavior.
- Added a project-wide “less is more” rule to `AGENTS.md` and the coding
  context so AI changes favor focused components and avoid speculative
  abstractions.
- Applied the feature-folder convention to the dashboard by organizing its
  reusable components and data under `app/src/components/dashboard/`, while
  leaving page state and navigation in `app/src/dashboard/`. This convention
  applies to future features as they grow.
- Generated default DiceBear weave avatars (`https://api.dicebear.com/10.x/weave/svg?seed=...`)
  on new account registration, persisting `avatarUrl` to PostgreSQL `User` table
  (`crates/shared/src/lib.rs`, `backend/src/auth/service.rs`). Added `avatar_url`
  support to `RegisterRequest` and `UserDto`, and added corresponding helper and
  types to `web/src/service/api.ts`.
- Added unit tests in `umuryango-shared` and end-to-end integration test
  `backend/tests/auth_avatar_test.rs` against live PostgreSQL verifying avatar
  generation, persistence, and profile/login retrieval.
- Built modular responsive app layout under `app/src/components/layouts/`:
  - `sidebar.rs`: Permanent navigation sidebar on big screens (`>= 768px`) with
    brand wordmark, workspace title, 5 navigation items (`Home`, `Money`,
    `Projects`, `Members`, `Profile`), member profile snippet, theme/language
    toggles, and sign-out control.
  - `footer.rs`: Fixed bottom `app-footer` navigation bar on small screens
    (`< 768px`) providing one-tap access to all 5 sections with touch feedback
    and active status pills.
  - `mobile_header.rs`: Compact top header on mobile screens with workspace
    branding, current section title, theme toggle, and language switch.
  - `nav.rs`: Typed `AppSection` enum, labels, and Font Awesome icon bindings.
  - `responsive.rs`: Breakpoint logic and responsive content width calculation.
  - `sections.rs`: Dedicated section views for Money, Projects, Members, and
    Profile.
  - `app_layout.rs`: Clean layout orchestrator providing seamless responsive
    switching and section routing.
  - Validated via unit tests for navigation order, breakpoint switching,
    content width calculations, and localized label availability.
- Implemented complete backend treasury, communal projects, member roster, and sync services:
  - `backend/src/transactions/`: list transactions with filtering, create transactions (strictly integer minor units, automatic approval for income/admins), authoritative treasury summary calculation, and admin transaction approval.
  - `backend/src/projects/`: list active communal projects with funded %, create projects, and record project contributions.
  - `backend/src/members/`: list active household members with roles and joined dates scoped by `family_id`.
  - `backend/src/sync/`: idempotent outbox push (`FamilySyncMutation`) and pull endpoint (`/api/v1/sync/pull`).
  - Registered all routes in `main.rs` and OpenAPI documentation in `openapi.rs`.
  - Added end-to-end integration test `backend/tests/family_finance_test.rs` covering the full financial and sync lifecycle against PostgreSQL.
- Connected native GPUI app (`app/`) to the backend and SQLite outbox:
  - Added API client methods in `app/src/auth/client.rs` for transactions, projects, members, and sync.
  - Implemented local SQLite outbox queue, retrieval, and acknowledgment in `app/src/db/mod.rs`.
  - Connected `DashboardView` with live treasury stats, activity feed, projects, member roster, pending approval review, and storage sync status.
  - Wired quick actions in `app/src/app.rs` for Record Contribution, Add Expense, New Project, and Sync Now (draining offline SQLite outbox).
- Validated via `cargo fmt --all -- --check`, `cargo check --workspace`, `cargo test -p umuryango-shared -p umuryango-backend`, `cargo test -p umuryango-app`, and `bun run check` in `web/`.
- Implemented extensible multi-style UI theme architecture and Cozy Kawaii aesthetic across native GPUI app (`app/`):
  - Created `app/src/theme/mod.rs` with `UiStyle` (`CozyKawaii`, `NeoBrutalism`, `Claymorphism3D`), `CozyMood` (`Matcha`, `Peach`, `Berry`), and `StyleTokens` stored in GPUI global state (`cx.set_global(...)`) and synced with `gpui-kit` / `rune-kit` theme tokens.
  - Configured Dark Cozy and Light Cozy palettes with tactile sticker outlines (`2.5px`–`3px`), rounded geometry (`32px` cards, `22px` buttons, `20px` inputs, `10px` checkboxes), and hard unblurred bottom rim shadows.
  - Updated all shared UI components (`card`, `input`, `button`, `checkbox`, `alert`, `avatar`, `badge`, `divider`, `modal`, `stat_card`, `toggle`), dashboard components, layout shell (`sidebar`, `footer`, `mobile_header`), and auth screens to consume `StyleTokens`.
  - Updated `LoginView` and `auth/layout.rs` with the custom kawaii family hearth icon, "Forgot PIN?" link, visibility toggle, default-checked remember-me checkbox, tactile primary sign-in button, and live Dark/Light Cozy + `CozyMood` header switcher.
- Refactored all 5 dashboard pages (`Home`, `Money`, `Projects`, `Members`/`People`, `Profile`) and layout chrome (`sidebar`, `mobile_header`/top navbar, `footer`) to match the Cozy Kawaii Family Finance HTML reference while reusing shared GPUI components:
  - Extended `StyleTokens` (`app/src/theme/mod.rs`) with semantic financial tokens (`income_bg/text/border`, `expense_bg/text/border`, `loan_bg/text/border`), `bg_elevated`, `bg_nav`, and `CozyMood::swatch_color`.
  - Added reusable UI primitives (`kawaii_progress_bar` in `app/src/components/ui/progress.rs`, `segmented_tab_bar` & `filter_pill_group` in `app/src/components/ui/segmented.rs`, `pill_button`, `pill_button_colored`, `compact_primary_button`, `ghost_icon_button` in `button.rs`, and `card_elevated` & `icon_badge_box` in `card.rs`).
  - Added reusable dashboard feature widgets (`treasury_hero_card`, `pending_approvals_banner`, `cashflow_insights_card`, `project_preview_row`, `project_goal_card`, `approval_request_card`, `income_stream_card`, `expense_category_card`, `loan_card`, and `member_pledge_card`) in `app/src/components/dashboard/`.
  - Updated desktop sidebar with nested Money sub-navigation (`Overview`, `Income`, `Expenses`, `Loan & Borrowed`), `+ Quick Entry` CTA, and bottom Live Family Treasury card; updated top navbar with ghost back button, dynamic section title/badge, Dark/Light Cozy pill toggle, 3 pastel mood dots (`Matcha`, `Peach`, `Berry`), and pending approvals bell.
- Downloaded and bundled Google Font `Fredoka` (`assets/fonts/Fredoka.ttf`), registered it with GPUI's `text_system().add_fonts(...)` in `app/src/theme/mod.rs`, and configured `StyleTokens.font_family = "Fredoka"` (synced with `gpui-kit` `Theme.font_family` and root layout containers) whenever `UiStyle::CozyKawaii` is active.
- Localized all 5 Cozy Kawaii dashboard pages (`Home`, `Money` with `Overview`/`Income`/`Expenses`/`Loan & Borrowed` sub-tabs, `Projects`, `Members`/`People`, and `Profile`), layout chrome (`sidebar`, `mobile_header`/top navbar, `sections`), and reusable dashboard feature components (`app/src/components/dashboard/`) in both English (`assets/locales/en.json`) and Kinyarwanda (`assets/locales/rw.json`), with dictionary parity unit tests in `app/src/i18n.rs`.
- Replaced native OS window chrome with a style-aware custom window titlebar (`app/src/components/layouts/window_titlebar.rs`) and client-side decorations (`app/src/window.rs`, `app/src/app.rs`):
  - Configured `TitlebarOptions { appears_transparent: true, .. }`, `app_owns_titlebar_drag: true`, and `WindowDecorations::Client`.
  - Built tactile Cozy Kawaii window control buttons for Minimize (`window.minimize_window()` / `WindowControlArea::Min`), Maximize / Restore (`window.zoom_window()` / `WindowControlArea::Max`), and Close (`window.remove_window()` / `WindowControlArea::Close`), plus draggable titlebar area (`WindowControlArea::Drag`, double-click to maximize, right-click system menu) that adapts its geometry (`window_control_radius`) to `UiStyle`.


