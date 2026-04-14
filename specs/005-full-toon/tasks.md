# Tasks: Full Character Download

**Input**: Design documents from `/specs/005-full-toon/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Per the constitution, TDD is MANDATORY for new code — test tasks MUST appear before their corresponding implementation tasks and MUST be confirmed failing before implementation begins.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prerequisite changes to existing code and new module scaffolding

- [x] T001 Add `current_season: KeyAndId` field to `MythicKeystoneSeasonsIndex` struct in `src/wow_models/mythic_keystone.rs` and verify deserialization with sample JSON containing `current_season`
- [x] T002 Add `full_character` module declaration and prelude re-exports in `src/wow_models.rs`
- [x] T003 Add `[[example]]` entry for `full-toon` in `Cargo.toml`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core types and trait that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Tests (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [x] T004 [P] Test `FullCharacter` struct can be constructed with all fields populated in `tests/full_character_test.rs`
- [x] T005 [P] Test `EndpointError` struct construction and field access in `tests/full_character_test.rs`

### Implementation

- [x] T006 Define `EndpointError` struct with `Serialize` + `Deserialize` in `src/wow_models/full_character.rs`
- [x] T007 Define `FullCharacter` struct (5 metadata + 28 endpoint `Option<T>` fields) with `Serialize` + `Deserialize` in `src/wow_models/full_character.rs`
- [x] T008 Define `CharacterFetcher` async trait with 4 methods (`fetch_endpoint`, `fetch_endpoint_with_token`, `fetch_endpoint_force`, `fetch_endpoint_with_token_force`) in `src/wow_models/full_character.rs`
- [x] T009 Implement `CharacterFetcher` for `BattleNetClient` (delegates to `get_data` / `get_data_with_token`; force variants are identical since no cache) in `src/wow_models/full_character.rs`

**Checkpoint**: Foundation ready — `FullCharacter`, `EndpointError`, `CharacterFetcher` trait, and `BattleNetClient` impl all compile

---

## Phase 3: User Story 1 — Full Toon Download as Struct (Priority: P1) 🎯 MVP

**Goal**: A single `full_character()` call fetches all 28 character profile endpoints, assembles results into a typed `FullCharacter` struct with `Option<T>` fields. Failed endpoints degrade gracefully to `None` + `EndpointError`. Base profile failure aborts the entire download.

**Independent Test**: Call `full_character()` with a known character. Verify common endpoints populate, class-specific misses are `None` with error detail, and `has_profile_data` reflects token presence.

### Tests for User Story 1 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [x] T010 [P] [US1] Test `full_character()` with known character populates profile + common endpoints (≥20 of 28) in `tests/full_character_test.rs`
- [x] T011 [P] [US1] Test `full_character()` with non-existent character returns `Err` (not partial struct) in `tests/full_character_test.rs`
- [x] T012 [P] [US1] Test graceful degradation — endpoint failure yields `None` field + entry in `errors` vec in `tests/full_character_test.rs`
- [x] T013 [P] [US1] Test `has_profile_data` is `false` without token, `true` with token in `tests/full_character_test.rs`
- [x] T014 [P] [US1] Test M+ current season auto-detection — `mythic_keystone_season` populated when seasons index succeeds in `tests/full_character_test.rs`

### Implementation for User Story 1

- [x] T015 [US1] Implement internal `try_fetch` helper — wraps a single endpoint fetch, returns `Option<T>` and optionally appends `EndpointError` to errors vec in `src/wow_models/full_character.rs`
- [x] T016 [US1] Implement M+ current season lookup — fetch `MythicKeystoneSeasonsIndex`, extract `current_season.id`, return `Option<u64>` in `src/wow_models/full_character.rs`
- [x] T017 [US1] Implement `full_character()` orchestration — sequential fetch of base profile (fail-fast), then 27 remaining endpoints using `try_fetch`, using `UrlArgs::Player` for most and `UrlArgs::PlayerExtra` for PvP brackets and M+ season in `src/wow_models/full_character.rs`
- [x] T018 [US1] Implement `full_character_force()` — same orchestration using `fetch_endpoint_force` / `fetch_endpoint_with_token_force` in `src/wow_models/full_character.rs`

**Checkpoint**: `full_character()` and `full_character_force()` return a populated `FullCharacter` struct for any character. US1 is fully functional and independently testable.

---

## Phase 4: User Story 2 — Full Toon Download as JSON (Priority: P1)

**Goal**: `full_character_json()` returns the same data as a serialized JSON string. Calls the struct version internally and serializes via `serde_json`.

**Independent Test**: Call `full_character_json()`, verify output is valid JSON that round-trips back to `FullCharacter` without data loss.

### Tests for User Story 2 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [x] T019 [P] [US2] Test `full_character_json()` returns valid JSON string in `tests/full_character_test.rs`
- [x] T020 [P] [US2] Test JSON round-trip — deserialize JSON output back to `FullCharacter`, verify all fields match in `tests/full_character_test.rs`

### Implementation for User Story 2

- [x] T021 [US2] Implement `full_character_json()` — calls `full_character()` then `serde_json::to_string()` in `src/wow_models/full_character.rs`
- [x] T022 [US2] Implement `full_character_json_force()` — calls `full_character_force()` then `serde_json::to_string()` in `src/wow_models/full_character.rs`

**Checkpoint**: JSON variants work. Round-trip serialize/deserialize preserves all data.

---

## Phase 5: User Story 3 — Cache-Aware Full Download (Priority: P2)

**Goal**: `full_character()` works transparently with `CachedClient`, leveraging per-endpoint caching from sprint 004. Second call for the same character returns from cache (≥10x faster).

**Independent Test**: Call `full_character()` twice with a `CachedClient`. Verify second call is significantly faster. Call `full_character_force()` and verify all endpoints are re-fetched.

### Tests for User Story 3 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [x] T023 [P] [US3] Test `CachedClient` implements `CharacterFetcher` (compile-time check + basic call) in `tests/full_character_test.rs`
- [x] T024 [P] [US3] Test cached `full_character()` second call is ≥10x faster than first in `tests/full_character_test.rs`

### Implementation for User Story 3

- [x] T025 [US3] Implement `CharacterFetcher` for `CachedClient<S: CacheStore>` — delegates to `get_data`/`get_data_with_token` (non-force) and `get_data_force`/`get_data_with_token_force` (force), feature-gated behind `db-sqlite` or `db-postgres` in `src/wow_models/full_character.rs`

**Checkpoint**: Cache integration works. Same `full_character()` orchestration transparently uses cache when available.

---

## Phase 6: User Story 4 — Full Toon Example (Priority: P3)

**Goal**: A working example that demonstrates full character download with struct and JSON output.

**Independent Test**: `cargo run --example full-toon --features "wow,user"` prints character data.

- [x] T026 [US4] Create `examples/full-toon.rs` — demonstrate `full_character()` with client credentials, print summary (name, level, populated fields count, errors count), then `full_character_json()` excerpt in `examples/full-toon.rs`

**Checkpoint**: Example runs and produces readable character output.

---

## Phase 7: Polish & Cross-Cutting Concerns (Constitution — mandatory)

**Purpose**: Documentation updates and final validation

- [x] T027 Run full pre-commit suite: `cargo fmt --check && cargo clippy --features "wow,db-sqlite,user,redis" --lib --tests --examples -- -D warnings && cargo test --features "wow,db-sqlite,user,redis" --lib --tests`
- [x] T028 [P] Update `docs/specification.md` with sprint 005 changes
- [x] T029 [P] Update `docs/architecture.md` to document the `full_character` module and `CharacterFetcher` trait
- [x] T030 [P] Update `docs/usage.md` with `full_character()` usage examples and code snippets
- [x] T031 Run quickstart.md validation (build, test, run example, JSON round-trip)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **US1 Struct (Phase 3)**: Depends on Foundational (Phase 2)
- **US2 JSON (Phase 4)**: Depends on US1 (Phase 3) — calls `full_character()` internally
- **US3 Cache (Phase 5)**: Depends on Foundational (Phase 2) — independent of US1/US2 at the trait level, but needs US1 for integration testing
- **US4 Example (Phase 6)**: Depends on US1 (Phase 3) and US2 (Phase 4)
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **US1 (P1)**: Depends only on Foundational — this is the MVP
- **US2 (P1)**: Depends on US1 — trivial wrapper
- **US3 (P2)**: Can start trait impl after Foundational, but integration tests need US1
- **US4 (P3)**: Depends on US1 + US2

### Within Each User Story

- Tests MUST be written and FAIL before implementation (TDD — non-negotiable)
- Internal helpers before orchestration functions
- Core function before force variant
- Story complete before moving to next priority

### Parallel Opportunities

Within Phase 2:
- T004, T005 can run in parallel (independent test cases)

Within Phase 3 (US1):
- T010, T011, T012, T013, T014 can all run in parallel (independent test cases in same file)

Within Phase 4 (US2):
- T019, T020 can run in parallel (independent test cases)

Within Phase 5 (US3):
- T023, T024 can run in parallel (independent test cases)

Within Phase 7:
- T028, T029, T030 can run in parallel (independent doc files)

---

## Implementation Strategy

### MVP Scope

**US1 alone** (Phase 1 + Phase 2 + Phase 3) delivers a working `full_character()` function. This is the minimum viable feature.

### Incremental Delivery

1. **MVP**: US1 — struct download works with `BattleNetClient`
2. **+JSON**: US2 — trivial addition, one-liner wrapping US1
3. **+Cache**: US3 — adds `CachedClient` support via trait impl
4. **+Example**: US4 — onboarding example for new users

### Key Files

| File | Action | Phase |
|------|--------|-------|
| `src/wow_models/mythic_keystone.rs` | Modify (add `current_season` field) | 1 |
| `src/wow_models.rs` | Modify (add module + prelude exports) | 1 |
| `Cargo.toml` | Modify (add example entry) | 1 |
| `src/wow_models/full_character.rs` | **Create** (main implementation) | 2–5 |
| `tests/full_character_test.rs` | **Create** (all tests) | 2–5 |
| `examples/full-toon.rs` | **Create** (example) | 6 |
| `docs/specification.md` | Modify | 7 |
| `docs/architecture.md` | Modify | 7 |
| `docs/usage.md` | Modify | 7 |
