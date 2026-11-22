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
- Added a project-wide â€œless is moreâ€ rule to `AGENTS.md` and the coding
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
  - Configured Dark Cozy and Light Cozy palettes with tactile sticker outlines (`2.5px`â€“`3px`), rounded geometry (`32px` cards, `22px` buttons, `20px` inputs, `10px` checkboxes), and hard unblurred bottom rim shadows.
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
- Connected all 5 dashboard pages and modals in `app/` to authoritative PostgreSQL backend endpoints and removed all static placeholder arrays (`TRANSACTIONS`, `PROJECTS`, `FAMILY_MEMBERS`, `INCOME_STREAMS`, `EXPENSE_CATEGORIES`, `LOANS`, `APPROVALS`, `MONTHLY_CASHFLOW`):
  - Added `backend/src/loans/` and `backend/src/handlers/loans.rs` (`GET/POST /api/v1/families/{family_id}/loans`, `POST /api/v1/families/{family_id}/loans/{id}/repay`), `POST /api/v1/families/{family_id}/transactions/{id}/reject`, and `POST /api/v1/families/{family_id}/members/invite`, plus shared DTOs (`LoanDto`, `CreateLoanRequest`, `LoanRepaymentRequest`, `InviteFamilyMemberRequest`).
  - Derived `income_streams`, `expense_categories`, `monthly_cashflow`, `approvals`, and member contribution progress dynamically from authoritative backend transactions and member records, and added an interactive entry modal (`EntryModalKind`) for recording income, expenses, loans, projects, and member invites with SQLite outbox fallback.
- Modularized `app/src/dashboard/entry_modal/` into cohesive components (`actions.rs`, `submit.rs`, `shell.rs`, `color_picker.rs`, `icon_picker.rs`), decomposing `category_dialog.rs` into subcomponents (`render_color_picker`, `render_icon_picker`, `render_category_type_switcher`) and reducing `mod.rs` from 790 lines to 118 lines. Validated with `cargo fmt`, `cargo check -p umuryango-app`, and test suites.
- Created reusable styled `select` component in `app/src/components/ui/select.rs` wrapping GPUI Kit's `Select` and `SelectState` with Cozy Kawaii tactile styling (`select_input`, `select_container`, `stateful_select_container`, `select_field`, `select_field_with_action`). Replaced text input and quick category chips in `transaction_form.rs` with the styled dropdown while preserving the `+ Category` dialog trigger. Bound and synchronized category selection with `DashboardView`. Validated with `cargo fmt --all`, `cargo check -p umuryango-app`, and test suites.
- Built reusable Cozy Kawaii styled Toast notification system in `app/src/components/ui/toast.rs` using GPUI Base's `ToastStack`, `Toast`, and `ToastManager` primitives (`ToastVariant`, `ToastItem`, `render_toast_item`, `render_toast_stack`). Integrated responsive floating `ToastStack` into `DashboardView` (`app/src/dashboard/mod.rs`), hooked auto-dismissing 50ms clock with hover-pause semantics, and wired toasts for all application actions: income creation, expense creation, offline queueing, project creation, loan issuance, member invitation, custom category creation, project boosts, loan repayments, transaction approvals/rejections, member contribution logging, and reminders. Validated with `cargo fmt --all`, `cargo check -p umuryango-app`, and test suites.

## 2026-11-21

- Implemented Android safe area insets handling and phone navigation bar padding across native GPUI app (`app/`):
  - Created `app/src/safe_area.rs` managing physical-pixel `SafeArea` insets (left, top, right, bottom), insets change notification flag, and JNI bridges (`Java_dev_gpui_mobile_GpuiActivity_nativeSetInsets`, `Java_dev_gpui_mobile_MainActivity_nativeSetInsets`, `Java_dev_gpui_mobile_example_MainActivity_nativeSetInsets`) connected with Android's `ViewCompat.setOnApplyWindowInsetsListener`.
  - Added reactive insets change watcher in `UmuryangoApp::new` (`app/src/app.rs`) polling atomic updates and triggering `cx.notify()` when insets change.
  - Converted physical insets to logical device pixels using `window.scale_factor()`, applying `pad_bottom` to `app-footer` navigation bar (`app/src/components/layouts/footer.rs`, `app_layout.rs`, `app/src/dashboard/mod.rs`), floating toast stack, and non-dashboard auth views (`login`, `register`, `terms`, `family_setup`) so navigation buttons and CTAs remain safe and accessible above 3-button phone navigation footers (Back, Home, Recent Apps).
  - Configured edge-to-edge Android Gradle project under `crates/mobile/android` with `GpuiActivity.java` window insets listener, `MainActivity.java`, and Umuryango branding.
  - Validated with unit tests in `app/src/safe_area.rs` for insets updates and JNI invocation, `cargo test -p umuryango-shared -p umuryango-backend -p umuryango-app --lib`, `cargo clippy --workspace -- -D warnings`, and `bun run check`.

- Replaced mobile launcher icons in `crates/mobile/android/app/src/main/res/` with the official Umuryango family icon: generated adaptive foregrounds (`drawable-{mdpi,hdpi,xhdpi,xxhdpi,xxxhdpi}/ic_launcher_foreground.png`) centered within Android's 72dp safe zone, updated `ic_launcher_background` to `#FF000000` in `colors.xml`, and generated legacy square and circular launcher icons (`mipmap-{mdpi,hdpi,xhdpi,xxhdpi,xxxhdpi}/ic_launcher.png`, `ic_launcher_round.png`) across all standard densities.

## 2026-11-22

- Fixed MSVC linker multiply-defined symbol error (`LNK2005` / `LNK1169`) when compiling `umuryango-app` binary on Windows:
  - `safe_area.rs` was being compiled both as a library module (`pub mod safe_area;` in `lib.rs`) and as a binary module (`mod safe_area;` in `main.rs`), resulting in duplicate unmangled JNI export symbols (`Java_dev_gpui_mobile_*_nativeSetInsets`) when `main.rs` linked `libumuryango_app.rlib`.
  - Switched `app/src/main.rs` to import `safe_area` via `use umuryango_app::safe_area;` matching the `umuryango_app::db` pattern.
  - Validated via `cargo build --bin umuryango-app`, `cargo test -p umuryango-app`, `cargo fmt --check`, and `cargo clippy -p umuryango-app`.

- Fixed window titlebar control button icon visibility and aligned styling to clean titlebar controls:
  - Explicitly specified `.text_color(idle_text)` and `.group_hover(id, move |s| s.text_color(hover_text))` on `Svg` icons in `app/src/components/layouts/window_titlebar.rs` so GPUI's `paint` method does not skip rendering them when `style.text.color` is `None`.
  - Used regular stroked icons (`free-regular-svg-icons`) for `window-restore` and `window-maximize` so the restore state displays clean outlined overlapping squares matching native caption controls.
  - Removed idle box borders, shadows, and opaque card backgrounds from window caption buttons so they sit cleanly on the titlebar, with smooth rounded hover highlights (`palette.bg_elevated` for minimize/maximize, `palette.error` red for close).
  - Added test lock in `safe_area.rs` to eliminate parallel test races on global static state.
  - Validated via `cargo test -p umuryango-app`, `cargo fmt --check`, and `cargo clippy -p umuryango-app`.

- Conditionally hid the Planned Projects preview card on the Home dashboard when no planned projects exist:
  - Evaluated `self.projects.iter().any(|p| !p.is_completed)` in `app/src/dashboard/home.rs`.
  - When no active/planned projects exist, omitted `projects_preview_card` via `when_some`, letting `cashflow_insights_card` cleanly occupy the row without an empty card container.
  - Validated via `cargo test -p umuryango-app`, `cargo fmt --check`, and `cargo clippy -p umuryango-app`.

- Adopted the dialog component pattern in `app/src/dashboard/entry_modal/project_form.rs`:
  - Structured `render_project_form` using DialogContent from crate::components::ui::dialog.
  - Added standalone open_project_dialog function leveraging GPUI Kit's declarative dialog architecture (window.open_dialog, DialogHeader, DialogTitle, DialogDescription, DialogFooter, and ButtonVariants) matching category_dialog.rs.
  - Added `render_project_dialog` alias to support uniform dialog component rendering.
  - Validated via cargo test -p umuryango-app (all 31 tests passing), cargo fmt --check, and cargo clippy -p umuryango-app -- -D warnings.

- Added Escape (Esc) key dismiss support to dialog and modal backdrops:
  - Added is_escape_key helper recognizing escape and esc keystrokes.
  - Updated dialog_backdrop and dialog_backdrop_action in `app/src/components/ui/dialog.rs` with .on_key_down to invoke close callbacks when Esc is pressed.
  - Updated `render_modal_backdrop` in `app/src/dashboard/entry_modal/shell.rs` to close the active entry modal on Esc.
  - Added auto-focus to entry_title_input upon opening modals in `app/src/dashboard/entry_modal/actions.rs` to immediately capture keyboard events.
  - Validated via cargo test -p umuryango-app (32 tests passing including 	est_is_escape_key), cargo fmt --check, and cargo clippy -p umuryango-app -- -D warnings.

- Added spring-animated chevron transition to sidebar navigation:
  - Updated `app/src/components/layouts/sidebar.rs` to animate chevrons using GPUI Kit's spring animation framework (`SpringConfig::new(280.0, 24.0, 1.0)`, `SpringAnimation`, `AnimationPhase`, `.with_spring(...)`).
  - Added `sidebar_chevron_icon(phase_value)` pure helper dynamically transitioning between `icons::ICON_CHEVRON_RIGHT` and `icons::ICON_CHEVRON_DOWN` as the navigation section expands or collapses.
  - Interpolated vertical position (`y_offset`) and active/muted text tinting for smooth, responsive feedback when toggling the Money section.
  - Added unit test `test_sidebar_chevron_icon_selection` to verify icon switching across phase thresholds.
  - Validated via `cargo test -p umuryango-app` (all 33 tests passing), `cargo fmt --check`, and `cargo clippy -p umuryango-app -- -D warnings`.
- Unified chevron SVG viewBoxes and alignment across directional states:
  - Fixed aspect ratio and alignment discrepancy between Font Awesome `chevron-right` (`320x512`), `chevron-down` (`448x512`), `chevron-left` (`320x512`), and `chevron-up` (`448x512`) in `assets/fontawesome/free-solid-svg-icons/`.
  - Normalized all 4 chevron SVGs to consistent `viewBox="0 0 512 512"` centered at (256, 256), ensuring exact identical stroke weight, arm proportions, tip curvature, and visual bounds across all orientations.
  - Sized the sidebar chevron container with fixed `w(px(14.0)).h(px(14.0))` and removed unnatural vertical shifting, eliminating jumping and keeping icons optically centered with navigation labels.
  - Validated via `cargo test -p umuryango-app` (33 tests passing), `cargo fmt --check`, and `cargo clippy -p umuryango-app -- -D warnings`.

- Removed pledge contribution tracking from the Members / People screen:
  - Replaced the monthly circle pledge hero card and progress bar in `app/src/dashboard/members.rs` with a clean Family Members count header (`dashboard.household_members`) and Invite Member button.
  - Added `member_card` in `app/src/components/dashboard/components.rs` displaying avatar, full name, role, status badge, and contact phone number without contribution progress or 'Log 25k' buttons.
  - Added `phone: Option<SharedString>` to `MemberContributionRow` in `app/src/components/dashboard/data.rs` populated from `FamilyMemberDto.phone_number`.
  - Updated `members_section` in `app/src/components/layouts/sections.rs` to align with the simplified member roster layout.
  - Validated via `cargo test -p umuryango-app` (33 tests passing), `cargo fmt --check`, and `cargo clippy -p umuryango-app -- -D warnings`.
- Removed reminder action buttons from member roster cards:
  - Streamlined `member_card` in `app/src/components/dashboard/components.rs` to display household member identity (avatar, name, role, status pill, and contact phone) without reminder triggers or buttons.
  - Simplified `render_people_screen` in `app/src/dashboard/members.rs` and `members_section` in `app/src/components/layouts/sections.rs` to directly render clean member cards.
  - Validated via `cargo test -p umuryango-app` (33 tests passing), `cargo fmt --check`, and `cargo clippy -p umuryango-app -- -D warnings`.
- Adopted the dialog component pattern in `app/src/dashboard/entry_modal/transaction_form.rs`:
  - Structured `render_transaction_form` using `DialogContent` from `crate::components::ui::dialog`.
  - Added `render_transaction_dialog` alias to support uniform dialog component rendering.
  - Added `open_transaction_dialog` leveraging GPUI Kit's declarative dialog architecture (`window.open_dialog`, `DialogHeader`, `DialogTitle`, `DialogDescription`, `DialogFooter`, and `ButtonVariants`) matching `project_form.rs` and `category_dialog.rs`.
  - Validated via `cargo test -p umuryango-app` (33 tests passing), `cargo fmt --check`, and `cargo clippy -p umuryango-app -- -D warnings`.
- Added date/time, account selector, and note textarea options to transaction modal:
  - Added `entry_account_select` (`Mobile Money`, `Cash`, `Bank Account`), `entry_date_input`, and `entry_notes_input` to `DashboardView`.
  - Added date/time input field with calendar icon prefix and quick 1-click 'Today' and 'Yesterday' selectors in `app/src/dashboard/entry_modal/transaction_form.rs`.
  - Added account select field (`Mobile Money`, `Cash`, `Bank Account`) and multiline notes textarea styled with `stateful_textarea_container`.
  - Updated transaction submission in `app/src/dashboard/entry_modal/submit.rs` to assemble account, date, and note details into `CreateTransactionRequest.notes` and toast notifications.
  - Updated English (`en.json`) and Kinyarwanda (`rw.json`) localization dictionaries with matching keys for all new controls.
  - Validated via `cargo test -p umuryango-app` (33 tests passing), `cargo fmt --check`, and `cargo clippy -p umuryango-app -- -D warnings`.
