# Tasks: Repo Baseline Audit & BattleNet API Research

**Input**: Design documents from `/specs/001-repo-baseline-audit/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, quickstart.md

**Tests**: Per the constitution, TDD is MANDATORY for new code — test tasks MUST appear before their corresponding implementation tasks and MUST be confirmed failing before implementation begins. This spec is primarily audit/documentation; TDD applies to the one code fix (dead code removal).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup

**Purpose**: Ensure the working environment is ready for audit work

- [X] T001 Verify `.env` file exists with valid BattleNet API credentials (BATTLENET_CLIENT_ID, BATTLENET_CLIENT_SECRET, BATTLENET_REGION, BATTLENET_LOCALE)
- [X] T002 Verify Rust stable toolchain is installed and at 1.94.0+ via `rustc --version`
- [X] T003 Run `cargo check` to confirm project compiles without errors

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish a green baseline — MUST be complete before any user story work

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T004 Run `cargo test` and verify all 8 existing tests pass
- [X] T005 Run `cargo fmt --check` and document all formatting diffs
- [X] T006 Run `cargo clippy --all-targets --all-features -- -D warnings` and document all violations

**Checkpoint**: Baseline status is known — proceed to fix issues in US1

---

## Phase 3: User Story 1 — Verify Existing Code Compiles and Tests Pass (Priority: P1) 🎯 MVP

**Goal**: Fix all lint/format issues so the full pre-commit CI suite passes clean

**Independent Test**: `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test` exits 0

### Tests for User Story 1 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [X] T007 [US1] Verify `cargo clippy --all-targets --all-features -- -D warnings` currently fails due to dead-code error on `Ack` struct in src/wow_models/character_profile.rs

### Implementation for User Story 1

- [X] T008 [US1] Run `cargo fmt` to fix all formatting diffs in examples/auction-play.rs, src/wow_models/character_profile.rs, src/wow_models/wow_token.rs
- [X] T009 [US1] Remove unused `Ack` struct from src/wow_models/character_profile.rs (line 67)
- [X] T010 [US1] Re-run `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test` and confirm full CI suite passes clean (exit 0)

**Checkpoint**: Pre-commit suite passes clean — green baseline established

---

## Phase 4: User Story 2 — Document Current Repo State (Priority: P2)

**Goal**: Create accurate inventory of all implemented endpoints/models/tests and write the architecture document

**Independent Test**: Every file in `src/`, `tests/`, `examples/` is reflected in `ModelImplementProgress.md`; `docs/architecture.md` covers all modules

### Implementation for User Story 2

- [X] T011 [P] [US2] Review every file in src/wow_models/ and verify each endpoint's Model/Impl/Test status in ModelImplementProgress.md is accurate
- [X] T012 [P] [US2] Review tests/ directory and verify test counts match ModelImplementProgress.md entries
- [X] T013 [US2] Update ModelImplementProgress.md with any corrections found in T011-T012 (fix status markers, add missing entries)
- [X] T014 [US2] Create docs/architecture.md covering: module layout (src/), BattleNetClient data flow, OAuth token lifecycle, bendpoint proc macro role, pygen code generator, URL construction scheme, namespace/region handling

**Checkpoint**: Repo state is fully documented and accurate

---

## Phase 5: User Story 3 — Research Current BattleNet API Surface (Priority: P2)

**Goal**: Identify all new/changed/removed API endpoints since development paused and update the progress document

**Independent Test**: Every endpoint on the official Blizzard API docs pages is accounted for in `ModelImplementProgress.md`

### Implementation for User Story 3

- [X] T015 [US3] Compare official WoW Game Data APIs page (https://community.developer.battle.net/documentation/world-of-warcraft/game-data-apis) against ModelImplementProgress.md Game Data section — list any new, changed, or removed endpoints
- [X] T016 [US3] Compare official WoW Profile APIs page (https://community.developer.battle.net/documentation/world-of-warcraft/profile-apis) against ModelImplementProgress.md Profile section — list any new endpoints (known new: Account Decor Collection, Account Transmog Collection)
- [X] T017 [US3] Compare official Battle.net OAuth APIs page (https://community.developer.battle.net/documentation/battle-net/oauth-apis) against src/auth.rs and src/client.rs — verify OAuth flow alignment
- [X] T018 [US3] Update ModelImplementProgress.md with all newly discovered endpoints marked as "New — TBD" and any changed paths noted

**Checkpoint**: API surface is fully catalogued and delta documented

---

## Phase 6: User Story 4 — Update Dependencies (Priority: P3)

**Goal**: Refresh all Cargo dependencies to latest compatible versions

**Independent Test**: `cargo test` and `cargo clippy -- -D warnings` pass after update

### Implementation for User Story 4

- [X] T019 [US4] Run `cargo update` to refresh Cargo.lock with latest compatible versions
- [X] T020 [US4] Run `cargo test` and verify all tests still pass after dependency update
- [X] T021 [US4] Run `cargo clippy --all-targets --all-features -- -D warnings` and resolve any new warnings introduced by updated dependencies
- [X] T022 [US4] Run `cargo fmt --check` and confirm no new formatting issues

**Checkpoint**: Dependencies are current and CI suite still passes clean

---

## Phase 7: User Story 5 — Create Initial Project Documentation (Priority: P3)

**Goal**: Create installation, usage, and combined specification documents

**Independent Test**: A new developer can follow the docs to build, test, and use the library

### Implementation for User Story 5

- [X] T023 [P] [US5] Create docs/installation.md covering: Rust toolchain setup, cloning the repo, .env configuration (reference .env-EXAMPLE), building with cargo build, running tests with cargo test
- [X] T024 [P] [US5] Create docs/usage.md covering: adding battlenet-rs as a dependency, creating a BattleNetClient, making API calls (WoW Token example at minimum), handling errors, understanding namespaces/regions
- [X] T025 [US5] Create docs/specification.md as the combined project specification per SDD constitution requirement — synthesize from this spec and any prior project context

**Checkpoint**: All docs exist and are sufficient for onboarding

---

## Phase 8: Polish & Cross-Cutting Concerns (Constitution — mandatory)

**Purpose**: Final verification and documentation sync

- [X] T026 Run full pre-commit suite (CI variant): `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test`
- [X] T027 [P] Verify docs/specification.md contains this spec's changes
- [X] T028 [P] Verify docs/architecture.md is complete and current
- [X] T029 [P] Verify docs/installation.md is accurate and followable
- [X] T030 [P] Verify docs/usage.md has working examples
- [X] T031 Run quickstart.md validation — follow specs/001-repo-baseline-audit/quickstart.md end-to-end

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Foundational — fixes baseline issues
- **US2 (Phase 4)**: Depends on US1 (need clean baseline to audit accurately); can run in parallel with US3
- **US3 (Phase 5)**: Depends on US1; can run in parallel with US2
- **US4 (Phase 6)**: Depends on US1 (need clean baseline before updating deps)
- **US5 (Phase 7)**: Depends on US2 and US3 (docs reference audit results and architecture)
- **Polish (Phase 8)**: Depends on all user stories being complete

### User Story Dependencies

- **US1 (P1)**: Can start after Foundational — No dependencies on other stories
- **US2 (P2)**: Can start after US1 — Parallel with US3
- **US3 (P2)**: Can start after US1 — Parallel with US2
- **US4 (P3)**: Can start after US1 — Independent of US2/US3
- **US5 (P3)**: Depends on US2 + US3 (needs audit results for docs content)

### Within Each User Story

- Tests MUST be written and FAIL before implementation (TDD — non-negotiable)
- Documentation tasks can run in parallel when marked [P]
- Verification before moving to next priority

### Parallel Opportunities

- T011 and T012 (US2 review tasks) can run in parallel
- T023 and T024 (US5 docs) can run in parallel
- T027, T028, T029, T030 (Polish verification) can run in parallel
- US2 and US3 can be worked on simultaneously after US1 completes

---

## Implementation Strategy

### MVP Scope
**User Story 1 only** — establish the green baseline. This is the minimum
viable outcome: the pre-commit suite passes clean, proving the codebase is
in a known-good state.

### Incremental Delivery
1. **US1**: Green baseline (MVP) — ~30 min
2. **US2 + US3**: Full audit + API research (parallel) — builds the knowledge base
3. **US4**: Dependency refresh — quick win after baseline is green
4. **US5**: Documentation creation — synthesizes all prior work
5. **Polish**: Final verification pass
