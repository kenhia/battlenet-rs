# Tasks: bnauth — Battle.net User OAuth Helper

**Input**: Design documents from `/specs/002-bnauth-oauth-helper/`  
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, quickstart.md  

**Tests**: Per the constitution, TDD is MANDATORY for new code — test tasks MUST appear before their corresponding implementation tasks and MUST be confirmed failing before implementation begins.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup

**Purpose**: Initialize both sub-projects (Python bnauth app + Rust redis feature)

- [x] T001 [P] Initialize `bnauth/` Python project with `uv init --name bnauth` and add dependencies (flask, redis, requests, python-dotenv) via `uv add`
- [x] T002 Add dev dependencies to `bnauth/` via `uv add --dev pytest ruff ty`
- [x] T003 Create `bnauth/.env-EXAMPLE` with all required env vars (`BATTLENET_CLIENT_ID`, `BATTLENET_CLIENT_SECRET`, `BATTLENET_REGION`, `REDISCLI_AUTH`, `BNAUTH_REDIS_HOST`, `BNAUTH_REDIS_PORT`, `BNAUTH_FLASK_PORT`, `FLASK_SECRET_KEY`)
- [x] T004 [P] Add `redis` crate as optional dependency in `Cargo.toml` with `[features] redis = ["dep:redis"]` and `redis = { version = "0.27", optional = true }`
- [x] T005 Run `cargo check --all-features` to verify the new dependency resolves

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Ensure both ecosystems have working CI and base structure before user story work

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T006 Create `bnauth/bnauth/__init__.py` (empty) and `bnauth/bnauth/app.py` with minimal Flask app skeleton (app factory, config loading, env var validation per FR-009/FR-010)
- [x] T007 Create Jinja2 template stubs: `bnauth/templates/index.html`, `bnauth/templates/success.html`, `bnauth/templates/error.html`
- [x] T008 [P] Add `UserTokenNotAvailable` variant to `BattleNetClientError` in `src/errors.rs` and add feature-gated `RedisError(redis::RedisError)` variant
- [x] T009 [P] Add `#[cfg(feature = "redis")] pub mod user_token;` to `src/lib.rs` and create empty `src/user_token.rs` with placeholder struct
- [x] T010 Run `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test` to verify Rust CI still passes with new module

**Checkpoint**: Both projects have working skeletons and CI passes

---

## Phase 3: User Story 1 — Authorize and Store User Token (Priority: P1) 🎯 MVP

**Goal**: Complete OAuth auth code flow in Flask, store token in Redis on `rpi53`

**Independent Test**: Start bnauth on cleo, click auth button, complete login, verify `bnauth:access_token` key exists in Redis with correct TTL

### Tests for User Story 1 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [x] T011 [US1] Write pytest test in `bnauth/tests/test_app.py` that verifies the index route `/` returns 200 with the auth button
- [x] T012 [US1] Write pytest test in `bnauth/tests/test_app.py` that verifies `/authorize` redirects to `oauth.battle.net/authorize` with correct query params (response_type, client_id, scope, state, redirect_uri)
- [x] T013 [US1] Write pytest test in `bnauth/tests/test_app.py` that verifies `/callback` rejects requests with mismatched `state` parameter (CSRF protection per FR-003)
- [x] T014 [US1] Write pytest test in `bnauth/tests/test_app.py` that verifies `/callback` with valid state+code exchanges for token and stores 5 keys in Redis with correct TTL (mock the requests.post to Battle.net and Redis)
- [x] T015 [US1] Write pytest test in `bnauth/tests/test_app.py` that verifies `/callback` displays error page when token exchange fails (FR-008)
- [x] T016 [US1] Write pytest test in `bnauth/tests/test_app.py` that verifies the app fails fast at startup when required env vars (`BATTLENET_CLIENT_ID`, `BATTLENET_CLIENT_SECRET`, `FLASK_SECRET_KEY`, `REDISCLI_AUTH`) are missing (FR-010)
- [x] T017 [US1] Confirm all tests in T011–T016 fail (Red phase)

### Implementation for User Story 1

- [x] T018 [US1] Implement index route `/` in `bnauth/bnauth/app.py` — render `index.html` with auth button, check Redis for existing token status (FR-001)
- [x] T019 [US1] Implement `index.html` template in `bnauth/templates/index.html` — show "Get Battle.net Auth" button (or token status if exists)
- [x] T020 [US1] Implement `/authorize` route in `bnauth/bnauth/app.py` — generate random state, store in session, redirect to Battle.net authorize URL with correct params (FR-002)
- [x] T021 [US1] Implement `/callback` route in `bnauth/bnauth/app.py` — validate state (FR-003), exchange code for token via POST with HTTP Basic auth (FR-004), store 5 keys in Redis with TTL (FR-005, FR-006)
- [x] T022 [US1] Implement `success.html` template in `bnauth/templates/success.html` — display token expiry, granted scopes, success confirmation (FR-007)
- [x] T023 [US1] Implement `error.html` template in `bnauth/templates/error.html` — display error message with "Try Again" link (FR-008)
- [x] T024 [US1] Handle Redis connection failure during token storage — show error page indicating token exchange succeeded but storage failed
- [x] T025 [US1] Re-run all pytest tests and confirm they pass (Green phase)
- [x] T026 [US1] Run `cd bnauth && uv run ruff format --check && uv run ruff check && uv run ty check && uv run pytest -q` to confirm Python CI passes

**Checkpoint**: bnauth Flask app is fully functional — auth flow stores token in Redis

---

## Phase 4: User Story 2 — Read User Token from Redis in battlenet-rs (Priority: P2)

**Goal**: Add feature-gated Rust module that reads the user access token from Redis

**Independent Test**: With a token stored in Redis, `cargo test --features redis` passes; `UserAccessToken` is returned with correct fields

### Tests for User Story 2 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [x] T027 [US2] Write integration test in `tests/user_token_test.rs` that reads a `UserAccessToken` from Redis when `bnauth:access_token` exists (requires live Redis with a token set)
- [x] T028 [US2] Write integration test in `tests/user_token_test.rs` that returns `UserTokenNotAvailable` error when `bnauth:access_token` key does not exist
- [x] T029 [US2] Write integration test in `tests/user_token_test.rs` that returns `RedisError` when Redis is unreachable (FR-016)
- [x] T030 [US2] Confirm tests T027–T029 fail (Red phase)

### Implementation for User Story 2

- [x] T031 [US2] Implement `UserAccessToken` struct in `src/user_token.rs` with fields: `access_token`, `token_type`, `expires_at`, `scope`, `obtained_at` (per data-model.md)
- [x] T032 [US2] Implement `read_user_token()` function in `src/user_token.rs` — connect to Redis via `BNAUTH_REDIS_HOST`, `BNAUTH_REDIS_PORT`, `REDISCLI_AUTH` env vars, GET all 5 `bnauth:*` keys, return `UserAccessToken` or error (FR-013, FR-014, FR-015, FR-016)
- [x] T033 [US2] Re-run `cargo test --features redis` and confirm tests pass (Green phase)
- [x] T034 [US2] Run `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --features redis` to confirm full Rust CI passes
- [x] T035 [US2] Write end-to-end Python test in `bnauth/tests/test_e2e.py` that stores a mock token in Redis, then reads it back and calls one user-scoped Battle.net endpoint (e.g., `GET /profile/user/wow`) to verify SC-002

> **Note (FR-017)**: User token / client credentials token separation is enforced architecturally — `UserAccessToken` in `src/user_token.rs` is a distinct type from the client token in `src/auth.rs`. No runtime mixing is possible.

**Checkpoint**: End-to-end flow works — Python stores token, Rust reads it, API call verified

---

## Phase 5: User Story 3 — Re-authorize After Token Expiry (Priority: P3)

**Goal**: Verify that re-auth overwrites expired/existing keys seamlessly

**Independent Test**: Delete Redis keys (simulating expiry), re-run auth flow, verify new keys appear with fresh TTLs

### Tests for User Story 3 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [x] T036 [US3] Write pytest test in `bnauth/tests/test_app.py` that verifies re-authorization overwrites existing Redis keys with new token data and fresh TTLs
- [x] T037 [US3] Confirm test T036 fails (Red phase)

### Implementation for User Story 3

- [x] T038 [US3] Verify that the Redis SET commands in `/callback` (T021) use `EX` TTL flag which inherently overwrites existing keys — no additional code needed; if not, update to ensure overwrites work
- [x] T039 [US3] Update `index.html` template to show "Re-authorize" button text and current token expiry when a valid token already exists in Redis
- [x] T040 [US3] Re-run pytest and confirm all tests pass (Green phase)

**Checkpoint**: Re-auth flow is verified — token overwrite works, UI shows current status

---

## Phase 6: Polish & Cross-Cutting Concerns (Constitution — mandatory)

**Purpose**: Documentation updates, final CI validation, quickstart verification

- [x] T041 Run full Rust pre-commit suite: `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --features redis`
- [x] T042 Run full Python pre-commit suite: `cd bnauth && uv run ruff format --check && uv run ruff check && uv run ty check && uv run pytest -q`
- [x] T043 [P] Update `docs/specification.md` with bnauth feature description and user token flow
- [x] T044 [P] Update `docs/architecture.md` with bnauth component diagram, Redis integration, user token vs client token separation
- [x] T045 [P] Update `docs/installation.md` with bnauth setup instructions (uv, .env, Developer Portal redirect URI registration)
- [x] T046 [P] Update `docs/usage.md` with user token examples (running bnauth, reading token from Rust with `--features redis`)
- [x] T047 Run quickstart.md validation — follow `specs/002-bnauth-oauth-helper/quickstart.md` end-to-end

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Foundational — Python-only, no Rust dependency
- **US2 (Phase 4)**: Depends on Foundational + US1 (needs token in Redis to test against)
- **US3 (Phase 5)**: Depends on US1 (verifies re-auth behavior)
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **US1 (P1)**: Can start after Foundational — No dependencies on other stories
- **US2 (P2)**: Can start Rust implementation after Foundational, but integration tests require US1 to have stored a token in Redis
- **US3 (P3)**: Depends on US1 (tests re-authorization on existing flow)

### Within Each User Story

- Tests MUST be written and FAIL before implementation (TDD — non-negotiable)
- Route implementation before template implementation
- Core flow before error handling
- Verification before moving to next priority

### Parallel Opportunities

- T001 and T004 (Python setup, Rust cargo feature) can run in parallel
- T008 and T009 (Rust error variants, module stub) can run in parallel
- T011–T016 (all US1 tests) can be written in parallel
- T043–T046 (all doc updates) can run in parallel

---

## Implementation Strategy

### MVP Scope
**User Story 1 only** — the Flask app can authorize and store a token in Redis.
This is the minimum viable outcome: the user can get a token from any machine
with a browser, and the token is available in Redis for any consumer.

### Incremental Delivery
1. **US1**: Flask app stores token in Redis (MVP)
2. **US2**: Rust reads token from Redis (completes end-to-end flow)
3. **US3**: Re-auth verification (polish, mostly testing)
4. **Polish**: Documentation and final CI validation
