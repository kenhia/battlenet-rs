# Tasks: Database Cache Layer & Rate Limiting

**Input**: Design documents from `/specs/004-db-cache/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Per the constitution, TDD is MANDATORY for new code — test tasks MUST appear before their corresponding implementation tasks and MUST be confirmed failing before implementation begins.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- Source: `src/` at repository root
- Macro crate: `model-macro/src/`
- Tests: `tests/` at repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add new dependencies, feature flags, and module scaffolding

- [X] T001 Add sqlx (optional), async-trait, and log dependencies and db-sqlite/db-postgres feature flags to Cargo.toml
- [X] T002 [P] Create src/cache/mod.rs with sub-module declarations (sqlite, postgres, cached_client) and compile_error! for mutual feature exclusivity
- [X] T003 Add cache module declaration (feature-gated under db-sqlite or db-postgres) and rate_limiter module declaration (unconditional — no feature gate) to src/lib.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core types and traits that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T004 Define CacheError enum (DatabaseError, SerializationError, SchemaInitError), CacheEntry struct, and CacheStore async trait in src/cache/mod.rs
- [X] T005 [P] Add CacheError variant (#[from] CacheError) to BattleNetClientError enum in src/errors.rs
- [X] T006 [P] Add cache_namespace() default method (returns WowNamespace::Static) to GenerateUrl trait in src/wow_models.rs

**Checkpoint**: Foundation ready — user story implementation can now begin

---

## Phase 3: User Story 5 — Model Serialization Round-Trip (Priority: P1) 🎯 MVP

**Goal**: All endpoint model structs gain `Serialize` so data can be cached as JSON and round-tripped without loss

**Independent Test**: Serialize any endpoint struct to JSON and deserialize it back — verify equality. No database or API access needed.

### Tests for User Story 5 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [X] T007 [US5] Write serialization round-trip tests for bendpoint structs (MountsIndex, AchievementCategoriesIndex) and manual structs (HrefLink, TypeAndName, Auction) using fixture JSON files in tests/serialize_test.rs

### Implementation for User Story 5

- [X] T008 [US5] Update bendpoint macro to emit `#[derive(Debug, Serialize, Deserialize)]` (was `#[derive(Debug, Deserialize)]`) and emit `fn cache_namespace() -> WowNamespace` based on namespace attribute in model-macro/src/lib.rs
- [X] T009 [US5] Update serde imports from `use serde::Deserialize` to `use serde::{Deserialize, Serialize}` and add `Serialize` to all `#[derive(Debug, Deserialize)]` manual structs across all 50 wow_models/ module files (28 files have manual structs — use script or sed for bulk update)

**Checkpoint**: All model structs serialize/deserialize — cache layer can store and retrieve typed data

---

## Phase 4: User Story 2 — API Rate Limiting (Priority: P1)

**Goal**: Dual-window token bucket rate limiter (100/s + 36,000/hr) with configurable "nice" mode, integrated into BattleNetClient

**Independent Test**: Issue burst of requests, verify rate limiter spaces them within configured limits. No cache or database needed.

### Tests for User Story 2 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [X] T010 [US2] Write rate limiter unit tests (per-second throttle, per-hour throttle, nice mode, window reset, acquire timing) in tests/rate_limiter_test.rs

### Implementation for User Story 2

- [X] T011 [US2] Implement RateLimiterConfig (Default: 100/s, 36000/hr, nice=false, nice=50/s) and RateLimiter (dual-window token bucket with async acquire()) in src/rate_limiter.rs
- [X] T012 [US2] Integrate rate limiter into BattleNetClient: add Option<Arc<RateLimiter>> field, with_rate_limiter() builder method, call acquire() before HTTP requests in send_request() and send_request_with_token() in src/client.rs

**Checkpoint**: All outbound API calls are rate-limited — safe for production batch operations

---

## Phase 5: User Story 1 — Cache Static Endpoint Responses (Priority: P1)

**Goal**: CachedClient wraps BattleNetClient, caches static-namespace responses, returns from cache on hit, supports force refresh

**Independent Test**: Call a static endpoint twice — first call hits API, second returns from cache without network activity. Force-refresh bypasses cache.

### Tests for User Story 1 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [X] T013 [US1] Write CacheStore impl tests (initialize, get/put/delete, schema creation), static caching behavior tests (cache hit, cache miss, force refresh), and cache write failure test (FR-025: API response returned even when store.put() fails) in tests/cache_test.rs

### Implementation for User Story 1

- [X] T014 [P] [US1] Implement SqliteCacheStore (pool creation, WAL mode, initialize schema, get/put/delete/delete_character/refresh_character_timestamp) in src/cache/sqlite.rs
- [X] T015 [P] [US1] Implement PostgresCacheStore (pool creation, initialize schema, get/put/delete/delete_character/refresh_character_timestamp) in src/cache/postgres.rs
- [X] T016 [US1] Implement CachedClient<S: CacheStore> with new(), new_with_ttl(), get_data<T>(), get_data_force<T>(), get_json<T>(), get_data_with_token<T>(), get_data_with_token_force<T>(), static namespace cache-first logic, and cache write failure handling (log::warn! on put failure, still return API response per FR-025) in src/cache/cached_client.rs

**Checkpoint**: Static endpoints are cached — repeated identical requests return instantly from database

---

## Phase 6: User Story 3 — Cache Dynamic/Profile Endpoint Responses (Priority: P2)

**Goal**: CachedClient always fetches from API for dynamic/profile namespaces but stores results in cache with timestamps

**Independent Test**: Fetch a dynamic endpoint, verify API is always called, confirm response is stored in cache with correct namespace and timestamp.

### Tests for User Story 3 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [X] T017 [US3] Write tests for dynamic (always-fetch-and-cache) and profile (fetch-cache-with-timestamp) namespace behavior in tests/cache_test.rs

### Implementation for User Story 3

- [X] T018 [US3] Extend CachedClient to handle dynamic namespace (always call API, cache afterward) and profile namespace (always call API, cache with fetched_at + character_id + realm_slug + char_name) in src/cache/cached_client.rs

**Checkpoint**: All three namespace types (static, dynamic, profile) handled correctly by CachedClient

---

## Phase 7: User Story 4 — 30-Day TTL Enforcement for Character Data (Priority: P2)

**Goal**: Cached profile data older than 30 days is automatically validated via CharacterProfileStatus; invalid/changed characters are purged

**Independent Test**: Insert cache entry with fetched_at > 30 days ago, request it, verify CharacterProfileStatus validation call occurs and correct purge/refresh behavior.

### Tests for User Story 4 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [X] T019 [US4] Write TTL enforcement tests: expired entry triggers validation, valid+matching refreshes timestamp, valid+mismatched purges and re-fetches, invalid/404 purges, <30 days skips validation, transient failure returns cached data in tests/cache_test.rs

### Implementation for User Story 4

- [X] T020 [US4] Implement 30-day TTL auto-validation in CachedClient: check fetched_at on profile cache retrieval, call CharacterProfileStatus, purge on invalid/mismatched, refresh timestamp on valid, graceful handling of transient validation failures in src/cache/cached_client.rs

**Checkpoint**: Blizzard ToS 2.R compliance — cached character data is automatically validated after 30 days

---

## Phase 8: User Story 6 — Multi-Reader Database Access (Priority: P3)

**Goal**: Multiple processes can read/write the cache concurrently without corruption or blocking

**Independent Test**: Spawn two async tasks — one writing, one reading — verify both complete without errors.

### Tests for User Story 6 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [X] T021 [US6] Write concurrent read/write test (spawn writer + reader tasks, verify no errors or data corruption) in tests/cache_test.rs

### Implementation for User Story 6

- [X] T022 [US6] Ensure SQLite WAL mode is enabled on pool creation and Postgres connection pool supports concurrent access in src/cache/sqlite.rs and src/cache/postgres.rs

**Checkpoint**: Cache is safe for multi-process deployment architectures

---

## Phase 9: User Story 7 — README Terms of Use Link (Priority: P3)

**Goal**: README includes a visible link to Blizzard's API Terms of Use

**Independent Test**: Read README.md and confirm the link is present.

- [X] T023 [US7] Add Blizzard Developer API Terms of Use link (https://www.blizzard.com/en-us/legal/a2989b50-5f16-43b1-abec-2ae17cc09dd6/blizzard-developer-api-terms-of-use) to README.md

**Checkpoint**: Compliance visibility for contributors and consumers

---

## Phase 10: Polish & Cross-Cutting Concerns (Constitution — mandatory)

**Purpose**: Documentation updates, validation, and code cleanup

- [X] T024 Run full pre-commit suite: `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test`
- [X] T025 [P] Update docs/specification.md with cache layer and rate limiting spec changes
- [X] T026 [P] Update docs/architecture.md with new cache/ module structure, rate limiter, CachedClient pattern
- [X] T027 [P] Update docs/installation.md with new dependencies (sqlx, async-trait, log) and feature flags (db-sqlite, db-postgres)
- [X] T028 [P] Update docs/usage.md with CachedClient usage examples and rate limiter configuration
- [X] T029 Run quickstart.md validation steps (build with each feature flag, run tests)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 completion — BLOCKS all user stories
- **US5 Serialization (Phase 3)**: Depends on Phase 2 — BLOCKS US1 and US3 (cache needs Serialize)
- **US2 Rate Limiting (Phase 4)**: Depends on Phase 2 only — independent of US5
- **US1 Static Cache (Phase 5)**: Depends on Phase 3 (US5) completion
- **US3 Dynamic/Profile Cache (Phase 6)**: Depends on Phase 5 (US1) — extends CachedClient
- **US4 TTL Enforcement (Phase 7)**: Depends on Phase 6 (US3) — profile caching must exist
- **US6 Multi-Reader (Phase 8)**: Depends on Phase 5 (US1) — cache stores must exist
- **US7 README (Phase 9)**: No code dependencies — can be done anytime
- **Polish (Phase 10)**: Depends on all desired phases being complete

### User Story Dependencies

```
Phase 1 (Setup)
    └─► Phase 2 (Foundational)
            ├─► Phase 3 (US5 Serialize) ──► Phase 5 (US1 Static Cache) ──► Phase 6 (US3 Dynamic/Profile) ──► Phase 7 (US4 TTL)
            │                                       └─► Phase 8 (US6 Multi-Reader)
            └─► Phase 4 (US2 Rate Limiting) [independent]
Phase 9 (US7 README) [independent — anytime]
Phase 10 (Polish) [after all stories]
```

### Within Each User Story

- Tests MUST be written and FAIL before implementation (TDD — non-negotiable)
- Models/types before services
- Core implementation before integration
- Story complete before moving to next dependent story

### Parallel Opportunities

**Phase 1**: T002 ∥ T003 (different files)
**Phase 2**: T005 ∥ T006 (different files, both after T004)
**Phase 3**: T009 handles all manual structs in one bulk pass (after T008)
**Phase 4**: US2 can run in parallel with US5 (Phase 3) since they have no mutual dependency
**Phase 5**: T014 ∥ T015 (SQLite and Postgres impls are independent)
**Phase 10**: T025 ∥ T026 ∥ T027 ∥ T028 (different doc files)

---

## Parallel Example: User Story 5 (Serialization)

```text
# T008 (macro update) then T009 (bulk import + derive update across all 50 files):
T008: Update bendpoint macro           ─► T009: Bulk update all wow_models/ files
```

## Parallel Example: User Story 1 (Static Cache)

```text
# After T013 (tests written):
T014: Implement SqliteCacheStore    ─┐
T015: Implement PostgresCacheStore  ─┘─ parallel (different files)
# Then:
T016: Implement CachedClient (depends on T014/T015)
```

---

## Implementation Strategy

### MVP First (US5 + US2 + US1)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL — blocks all stories)
3. Complete Phase 3: US5 — Model Serialization
4. Complete Phase 4: US2 — Rate Limiting
5. Complete Phase 5: US1 — Cache Static Endpoints
6. **STOP and VALIDATE**: Static caching + rate limiting is functional and testable
7. This delivers the highest-value increment: cached static lookups + API safety

### Incremental Delivery

1. Setup + Foundational → Infrastructure ready
2. US5 Serialization → All models can round-trip (MVP building block)
3. US2 Rate Limiting → Library is API-safe (independent value)
4. US1 Static Cache → Cached static lookups (core MVP!)
5. US3 Dynamic/Profile Cache → Full namespace coverage
6. US4 TTL Enforcement → ToS compliance
7. US6 Multi-Reader → Production-ready concurrency
8. US7 README → Documentation compliance
9. Polish → Docs updated, validation complete

Each story adds value without breaking previous stories.
