# Tasks: ktoons — WoW Character Viewer

**Input**: Design documents from `specs/006-ktoons-gui-app/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Per the constitution, TDD is MANDATORY for new code — test tasks MUST appear before their corresponding implementation tasks and MUST be confirmed failing before implementation begins.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Tauri backend**: `ktoons/src-tauri/src/`
- **Svelte frontend**: `ktoons/src/`
- **Frontend components**: `ktoons/src/lib/components/`
- **Frontend utilities**: `ktoons/src/lib/`
- **Workspace root**: `Cargo.toml`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Scaffold the Tauri + Svelte project and configure the workspace

- [x] T001 Scaffold Tauri 2 + Svelte + TypeScript project in `ktoons/` using `create-tauri-app` (select pnpm, Svelte, TypeScript)
- [x] T002 Add `ktoons/src-tauri` as workspace member in root `Cargo.toml`
- [x] T003 Configure `ktoons/src-tauri/Cargo.toml` with dependencies: `battlenet-rs` (path dep, features: wow, user, db-sqlite), `tauri` 2, `tauri-plugin-oauth` 2, `tauri-plugin-shell` 2, `tokio` 1 (full), `serde`/`serde_json` 1, `reqwest` 0.12 (json)
- [x] T004 Install frontend dependencies: `@tauri-apps/api`, `@fabianlars/tauri-plugin-oauth`, `@tauri-apps/plugin-shell`; install dev dependencies: `vitest`, `@testing-library/svelte`, `jsdom`
- [x] T005 Configure `ktoons/src-tauri/tauri.conf.json` with app identifier (`com.ktoons.app`), window title, and initial window size
- [x] T006 Verify project builds and launches with `pnpm tauri dev` in `ktoons/`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core Tauri backend infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T007 Implement `AppState` struct in `ktoons/src-tauri/src/state.rs` — holds `CachedClient` and `tokio::sync::Mutex<Option<UserToken>>`
- [x] T008 Implement app initialization in `ktoons/src-tauri/src/main.rs` — create `BattleNetClient` from env vars, initialize `SqliteCacheStore` with `app.path().app_data_dir()/cache.db`, create `CachedClient`, manage `AppState`, register plugins (oauth, shell)
- [x] T009 Implement `get_realms` Tauri command in `ktoons/src-tauri/src/commands.rs` — fetch realm search data, return `Vec<{name, slug}>` (per contracts/tauri-commands.md). Depends on T007 (`AppState`)
- [x] T010 [P] Define TypeScript interfaces in `ktoons/src/lib/types.ts` — `RealmEntry`, `CharacterListEntry`, `FullCharacter` (MVP-relevant fields only: profile, equipment, statistics, specializations, media, errors), `AccountCharacterEntry`. Note: `AccountCharacterEntry` is the backend response shape from `login`; `CharacterListEntry` is the frontend nav state (superset, includes quick-lookup characters)
- [x] T011 [P] Implement Tauri invoke wrappers in `ktoons/src/lib/commands.ts` — typed `getRealms()`, `lookupCharacter()`, `login()`, `getCharacter()`, `refreshCharacter()` functions
- [x] T012 [P] Create `LoadingSpinner.svelte` component in `ktoons/src/lib/components/LoadingSpinner.svelte`
- [x] T013 [P] Create `ErrorDisplay.svelte` component in `ktoons/src/lib/components/ErrorDisplay.svelte` — displays error message with retry button, accepts `message` and `onRetry` props

**Checkpoint**: Foundation ready — Tauri backend initializes, CachedClient connects, realm list fetches, shared components and types exist. User story implementation can now begin.

---

## Phase 3: User Story 1 — Quick Lookup a Character (Priority: P1) 🎯 MVP

**Goal**: User enters a character name, selects a realm, clicks Lookup → character summary displays (header, ilvl, gear, stats, specs, portrait)

**Independent Test**: Launch app → enter "Belarsa" and select "Trollbane" → click Lookup → verify all 6 summary sections render with data

### Tests for User Story 1 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [x] T014 [P] [US1] Rust unit test for `lookup_character` command in `ktoons/src-tauri/src/commands.rs` — test with mock/stub `CachedClient`, verify `FullCharacter` JSON serialization, verify 404 returns descriptive error string
- [x] T015 [P] [US1] vitest component test for `LaunchScreen.svelte` in `ktoons/tests/components/LaunchScreen.test.ts` — verify realm dropdown renders from mock data, lookup button triggers invoke, error state renders `ErrorDisplay`
- [x] T016 [P] [US1] vitest component test for `CharacterHeader.svelte` in `ktoons/tests/components/CharacterHeader.test.ts` — verify all header fields render from mock `FullCharacter` data (name, level, race, class, faction, guild, ilvl, portrait)
- [x] T017 [P] [US1] vitest component test for `ErrorDisplay.svelte` in `ktoons/tests/components/ErrorDisplay.test.ts` — verify error message renders, retry button fires `onRetry` callback

### Implementation for User Story 1

- [x] T018 [US1] Implement `lookup_character` Tauri command in `ktoons/src-tauri/src/commands.rs` — calls `full_character()` with `token: None`, returns serialized `FullCharacter` JSON
- [x] T019 [US1] Create `LaunchScreen.svelte` in `ktoons/src/lib/components/LaunchScreen.svelte` — realm dropdown (populated from `getRealms()`), character name input, "Lookup" button, "Login with Battle.net" button (placeholder for US2), loading state, error display
- [x] T020 [P] [US1] Create `CharacterHeader.svelte` in `ktoons/src/lib/components/CharacterHeader.svelte` — displays name, level, race, class, faction, guild, realm, active title, achievement points, equipped/average ilvl, character portrait image
- [x] T021 [P] [US1] Create `EquipmentList.svelte` in `ktoons/src/lib/components/EquipmentList.svelte` — renders equipped items per slot with item name, ilvl, and quality color-coding (POOR→gray, COMMON→white, UNCOMMON→green, RARE→blue, EPIC→purple, LEGENDARY→orange)
- [x] T022 [P] [US1] Create `StatsPanel.svelte` in `ktoons/src/lib/components/StatsPanel.svelte` — displays health, power (with type name), strength, agility, intellect, stamina (effective values)
- [x] T023 [P] [US1] Create `SpecializationsPanel.svelte` in `ktoons/src/lib/components/SpecializationsPanel.svelte` — shows active specialization (highlighted) and list of all available specs
- [x] T024 [US1] Create `CharacterNav.svelte` in `ktoons/src/lib/components/CharacterNav.svelte` — left sidebar listing characters the user has viewed, click to switch, highlights active character
- [x] T025 [US1] Wire up main app page in `ktoons/src/routes/+page.svelte` (or `ktoons/src/App.svelte`) — layout with left nav + main panel, state management for current view (launch screen vs character view), character list store, active character store
- [x] T026 [US1] Implement the Quick Lookup flow end-to-end: LaunchScreen calls `lookupCharacter()` → on success, add character to nav list, switch to character view, render all summary sections; on error, show ErrorDisplay in LaunchScreen
- [x] T027 [US1] Add CSS styling for MVP layout — left nav fixed width, main panel scrollable, functional/clean CSS (no WoW theming), quality color classes for gear

**Checkpoint**: User Story 1 complete — user can look up any character by name+realm and see the full summary. Multiple lookups accumulate in the left nav.

---

## Phase 4: User Story 2 — Login with Battle.net (Priority: P2)

**Goal**: User clicks "Login with Battle.net" → OAuth in browser → app shows all account characters grouped by realm in left nav

**Independent Test**: Click Login → authenticate in browser → verify left nav populates with all account characters grouped by realm → click a character → verify summary displays

### Tests for User Story 2 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [x] T028 [P] [US2] Rust unit test for `login` command in `ktoons/src-tauri/src/commands.rs` — test with mock OAuth callback, verify `Vec<AccountCharacterEntry>` response shape, verify OAuth failure returns descriptive error
- [x] T029 [P] [US2] Rust unit test for `get_character` command in `ktoons/src-tauri/src/commands.rs` — verify user token is forwarded to `full_character()` when present, verify fallback to `None` when no token

### Implementation for User Story 2

- [x] T030 [US2] Implement OAuth flow in `ktoons/src-tauri/src/oauth.rs` — generate CSRF state, construct Blizzard authorize URL with `redirect_uri=http://127.0.0.1:5055/callback`, exchange auth code for token via POST to Blizzard token endpoint, return `UserToken`
- [x] T031 [US2] Implement `login` Tauri command in `ktoons/src-tauri/src/commands.rs` — start OAuth server via `tauri-plugin-oauth` on port 5055, open browser to authorize URL via `tauri-plugin-shell`, wait for callback, exchange code, store `UserToken` in `AppState`, fetch `AccountProfileSummary`, return `Vec<AccountCharacterEntry>`
- [x] T032 [US2] Implement `get_character` Tauri command in `ktoons/src-tauri/src/commands.rs` — calls `full_character()` with user token from `AppState` if available (else `None`), returns serialized `FullCharacter`
- [x] T033 [US2] Update `LaunchScreen.svelte` — wire "Login with Battle.net" button to call `login()`, on success populate nav with account characters grouped by realm, transition to character view. After login, nav clicks MUST use `getCharacter()` (not `lookupCharacter()`) to leverage the user token for user-scoped endpoints
- [x] T034 [US2] Update `CharacterNav.svelte` — support grouped-by-realm display (realm headers with character list underneath), distinguish logged-in mode (grouped) vs quick-lookup mode (flat list). Clicking a character invokes `getCharacter()` when user token is present
- [x] T035 [US2] Handle OAuth errors in the frontend — if login fails, show ErrorDisplay on LaunchScreen with specific message, user can retry or fall back to Quick Lookup
- [x] T036 [US2] Implement mid-session token expiry detection — detect 401 responses from API calls, clear expired `UserToken` from `AppState`, notify frontend to show re-auth prompt, revert to client-token-only behavior until user logs in again (covers spec edge case: "OAuth token expires during use")

**Checkpoint**: User Story 2 complete — OAuth login works, account characters listed by realm, clicking a character fetches and displays its summary. Token expiry is handled gracefully.

---

## Phase 5: User Story 3 — Refresh Character Data (Priority: P2)

**Goal**: User clicks "Refresh" on a displayed character → data re-fetched from API (cache bypassed) → display updates

**Independent Test**: View a character → click Refresh → verify fetched_at timestamp changes and data reflects any API changes

### Tests for User Story 3 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [x] T037 [US3] Rust unit test for `refresh_character` command in `ktoons/src-tauri/src/commands.rs` — verify it calls `full_character_force()` (not `full_character()`), verify user token forwarding when present

### Implementation for User Story 3

- [x] T038 [US3] Implement `refresh_character` Tauri command in `ktoons/src-tauri/src/commands.rs` — calls `full_character_force()` with user token if available, returns serialized `FullCharacter`
- [x] T039 [US3] Add "Refresh" button to the character view in `ktoons/src/routes/+page.svelte` (or character view area) — positioned in character header area, shows loading spinner during refresh, on success updates displayed data, on error shows warning while preserving existing data

**Checkpoint**: User Story 3 complete — Refresh button re-fetches all data bypassing cache and updates the display.

---

## Phase 6: User Story 4 — Cached Data for Fast Repeat Views (Priority: P3)

**Goal**: Previously viewed characters load instantly from SQLite cache on repeat access

**Independent Test**: Look up a character (uncached) → note load time → close/relaunch app → look up same character → verify significantly faster load (cache hit)

### Tests for User Story 4 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [x] T040 [US4] Rust integration test for cache behavior in `ktoons/src-tauri/` tests — verify that `CachedClient` creates SQLite DB file in expected path, verify second fetch for same character is served from cache (no API call)

### Implementation for User Story 4

- [x] T041 [US4] Verify `CachedClient` integration works end-to-end — confirm that `lookup_character` and `get_character` commands use the `CachedClient` (not bare `BattleNetClient`), and that the SQLite DB file is created in `app_data_dir()`
- [x] T042 [US4] Add visual indicator for cache status — show `fetched_at` timestamp on the character view so users can see when data was last fetched and decide whether to refresh

**Checkpoint**: User Story 4 complete — cache is transparently active, repeat views are fast, user can see data freshness.

---

## Phase 7: User Story 5 — Partial Failure Handling (Priority: P3)

**Goal**: When individual character data sections fail, available sections still display with a warning listing failures

**Independent Test**: View a character where some endpoints fail (e.g., hunter pets for a non-hunter) → verify available sections render → verify warning lists failed endpoints

### Tests for User Story 5 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [x] T043 [P] [US5] vitest component test for partial failure warning banner in `ktoons/tests/components/PartialFailureWarning.test.ts` — verify warning renders with list of failed endpoints from mock `FullCharacter.errors`, verify warning is hidden when errors array is empty
- [x] T044 [P] [US5] vitest component test for missing section handling in `ktoons/tests/components/` — verify `CharacterHeader` shows full error when `profile` is null (FR-011), verify `EquipmentList`/`StatsPanel`/`SpecializationsPanel` hide gracefully when their data field is null

### Implementation for User Story 5

- [x] T045 [US5] Add partial failure warning component to character view — check `FullCharacter.errors` array, if non-empty display a collapsible warning banner listing each failed endpoint and its error message
- [x] T046 [US5] Handle missing sections gracefully in all display components (`CharacterHeader`, `EquipmentList`, `StatsPanel`, `SpecializationsPanel`) — if the corresponding `FullCharacter` field is `null`, hide the section (don't show empty/broken UI); if `profile` is null, show full error (FR-011)

**Checkpoint**: User Story 5 complete — partial failures show available data with warnings, base profile failure shows full error.

---

## Phase 8: Polish & Cross-Cutting Concerns (Constitution — mandatory)

**Purpose**: Documentation updates, code quality, and final validation

- [x] T047 Run full pre-commit suite for Rust: `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test` (from workspace root, verify `ktoons/src-tauri` crate passes)
- [x] T048 [P] Run frontend checks: `pnpm run check` (svelte-check) and `pnpm run lint` (if configured) in `ktoons/`
- [x] T049 [P] Update `docs/specification.md` — merge ktoons feature specification
- [x] T050 [P] Update `docs/architecture.md` — add ktoons application architecture (Tauri + Svelte + battlenet-rs integration, data flow diagram)
- [x] T051 [P] Update `docs/installation.md` — add ktoons prerequisites (system deps, node.js, pnpm) and setup steps
- [x] T052 [P] Update `docs/usage.md` — add ktoons usage guide (launch, quick lookup, login, refresh)
- [x] T053 Validate quickstart.md — follow the steps on a clean checkout and confirm the app builds and runs

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Foundational — the MVP
- **US2 (Phase 4)**: Depends on Foundational; integrates with US1 components (CharacterNav, main page)
- **US3 (Phase 5)**: Depends on US1 (needs character view to exist)
- **US4 (Phase 6)**: Depends on US1 (needs character fetch to exist); mostly verification
- **US5 (Phase 7)**: Depends on US1 (needs display components to exist)
- **Polish (Phase 8)**: Depends on all desired user stories being complete

### User Story Dependencies

- **US1 (P1)**: Can start after Foundational — no story dependencies
- **US2 (P2)**: Can start after Foundational — integrates with US1's nav and page layout
- **US3 (P2)**: Depends on US1's character view existing
- **US4 (P3)**: Depends on US1's fetch pipeline existing; mostly verification/polish
- **US5 (P3)**: Depends on US1's display components existing

### Within Each User Story

- **Tests MUST be written and FAIL before implementation (TDD — non-negotiable)**
- Backend commands before frontend components that call them
- Shared/reusable components before pages that compose them
- Core flow before polish (error handling, styling)

### Parallel Opportunities

- T010, T011, T012, T013 (Foundational frontend tasks) can all run in parallel
- T014, T015, T016, T017 (US1 tests) can all run in parallel
- T020, T021, T022, T023 (US1 display components) can all run in parallel
- T028, T029 (US2 tests) can run in parallel
- T043, T044 (US5 tests) can run in parallel
- T049, T050, T051, T052 (Polish docs) can all run in parallel
- US4 and US5 can run in parallel with each other (both depend on US1 only)

---

## Parallel Example: User Story 1 Display Components

```bash
# These four component tasks can all be done simultaneously (different files, no deps):
T020: CharacterHeader.svelte
T021: EquipmentList.svelte
T022: StatsPanel.svelte
T023: SpecializationsPanel.svelte
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001–T006)
2. Complete Phase 2: Foundational (T007–T013)
3. Complete Phase 3: User Story 1 — tests first (T014–T017), then implementation (T018–T027)
4. **STOP and VALIDATE**: Launch app, look up a character, verify all 6 sections display
5. Demo-ready MVP

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. Add US1 → Quick Lookup works → **MVP!**
3. Add US2 → OAuth + account character list + token expiry handling → Primary workflow
4. Add US3 → Refresh button → Data stays fresh
5. Add US4 + US5 → Cache verification + partial failure polish
6. Polish → Docs, code quality, validation
