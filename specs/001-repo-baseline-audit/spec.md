# Feature Specification: Repo Baseline Audit & BattleNet API Research

**Feature Branch**: `001-repo-baseline-audit`  
**Created**: 2026-04-07  
**Status**: Draft  
**Spec Path**: `specs/001-repo-baseline-audit/spec.md` *(SDD: required before implementation)*  
**Input**: Audit the current state of the battlenet-rs repository after a long hiatus, research current BattleNet/WoW APIs for changes, and bring the project into a "ready to resume development" state.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Verify Existing Code Compiles and Tests Pass (Priority: P1)

As a developer returning to this project after a long hiatus, I need to confirm
that the existing codebase compiles cleanly and all existing tests pass, so I
have a known-good baseline before making any changes.

**Why this priority**: Nothing else matters if the existing code is broken.
Establishing a green baseline is the prerequisite for all other work.

**Independent Test**: Run the full pre-commit suite (`cargo fmt --check`,
`cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`) and
confirm zero failures.

**Acceptance Scenarios**:

1. **Given** the repository at its current state on `main`, **When** I run
   `cargo check`, **Then** the project compiles with no errors.
2. **Given** a valid `.env` file with BattleNet API credentials, **When** I run
   `cargo test`, **Then** all 8 existing tests pass.
3. **Given** the full pre-commit suite, **When** I run `cargo fmt --check` and
   `cargo clippy --all-targets --all-features -- -D warnings`, **Then** there
   are zero formatting or lint violations (or any violations are documented and
   addressed).

---

### User Story 2 - Document Current Repo State (Priority: P2)

As a developer, I need a clear inventory of what has been implemented, what is
partially done, and what is missing so I can plan future work efficiently.

**Why this priority**: Without an accurate inventory, future specs will be
built on guesswork. This audit informs the priority and scope of all subsequent
sprints.

**Independent Test**: Review the updated `ModelImplementProgress.md` and
`docs/architecture.md` and confirm they accurately reflect every source file,
model struct, test, and example in the repository.

**Acceptance Scenarios**:

1. **Given** the current codebase, **When** I compare every file in `src/`,
   `tests/`, and `examples/` against the progress document, **Then** every
   implemented endpoint, model, and test is accurately listed with its current
   status (✅ / ❌ / partial).
2. **Given** the updated documentation, **When** I read the architecture
   document, **Then** I can understand the module layout, data flow (client →
   request → model deserialization), and the role of the proc macro crate
   without reading source code.

---

### User Story 3 - Research Current BattleNet API Surface (Priority: P2)

As a developer, I need to know what API endpoints Blizzard currently offers
for Battle.net OAuth and World of Warcraft (retail) so I can identify any new
endpoints added since development paused and flag any that may have been
deprecated or changed.

**Why this priority**: Same tier as US2 — the API research and the repo audit
together form the complete "state of the world" picture needed before coding
resumes.

**Independent Test**: Compare the API reference pages at
`community.developer.battle.net` against the progress document and confirm
every current API category and endpoint is accounted for, with notes on any
additions, removals, or path changes.

**Acceptance Scenarios**:

1. **Given** the official WoW Game Data APIs documentation, **When** I compare
   it to `ModelImplementProgress.md`, **Then** every endpoint listed by
   Blizzard is present in the progress document (newly discovered endpoints
   are marked as "New — TBD").
2. **Given** the official WoW Profile APIs documentation, **When** I compare
   it to `ModelImplementProgress.md`, **Then** every profile endpoint is
   present, including any new Account Profile collection types (e.g., Decor,
   Transmogs) added since the original document was written.
3. **Given** the official Battle.net OAuth APIs documentation, **When** I
   review the existing `auth.rs` and `client.rs`, **Then** the OAuth flow
   implementation aligns with the current documented endpoints and there are no
   breaking changes.

---

### User Story 4 - Update Dependencies (Priority: P3)

As a developer, I need all Cargo dependencies updated to current compatible
versions so the project benefits from bug fixes, security patches, and
performance improvements accumulated during the hiatus.

**Why this priority**: Dependency freshness is important but lower risk than
verifying the baseline or doing the API audit. Dependencies should be updated
after confirming the existing code compiles.

**Independent Test**: After running `cargo update` and addressing any breaking
changes, the full pre-commit suite passes clean.

**Acceptance Scenarios**:

1. **Given** the current `Cargo.toml`, **When** I run `cargo update`, **Then**
   `Cargo.lock` is refreshed with the latest compatible versions.
2. **Given** updated dependencies, **When** I run `cargo test`, **Then** all
   existing tests still pass.
3. **Given** updated dependencies, **When** I run `cargo clippy --all-targets
   --all-features -- -D warnings`, **Then** there are no new warnings
   introduced by the updates.

---

### User Story 5 - Create Initial Project Documentation (Priority: P3)

As a developer (or future contributor), I need installation, setup, and usage
guides so I can get the project running without tribal knowledge.

**Why this priority**: Documentation is a constitution requirement (Principle
IV) and should be established during this foundational spec, but does not
block the audit or research work.

**Independent Test**: A new developer can follow `docs/installation.md` and
`docs/usage.md` to clone the repo, configure credentials, build the project,
run tests, and make a sample API call.

**Acceptance Scenarios**:

1. **Given** `docs/installation.md`, **When** a new developer follows the
   steps, **Then** they can build the project and run tests.
2. **Given** `docs/usage.md`, **When** a developer reads the usage guide,
   **Then** they can make at least one API call (e.g., WoW Token price) using
   the library.

---

### Edge Cases

- What if existing tests fail due to BattleNet API changes (e.g., field
  renames, removed endpoints)? — Document the failure, fix the model or test,
  and note the change in the API delta section of the progress document.
- What if a dependency has a major version bump with breaking changes? —
  Assess the scope of breakage; if trivial, fix inline. If extensive, create a
  separate follow-up spec for the migration.
- What if the BattleNet API documentation itself is incomplete or inconsistent
  with live responses? — Note the discrepancy in the progress document and
  verify against a live API call where possible.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The project MUST compile cleanly with `cargo check` on the
  current stable Rust toolchain.
- **FR-002**: All existing tests MUST pass with `cargo test` when valid API
  credentials are configured in `.env`.
- **FR-003**: The `ModelImplementProgress.md` document MUST be updated to
  accurately reflect every implemented endpoint, model, test, and their
  current status.
- **FR-004**: The progress document MUST include all current WoW Game Data API
  endpoints from the official Blizzard documentation, with new endpoints
  (added since the last update) clearly marked.
- **FR-005**: The progress document MUST include all current WoW Profile API
  endpoints, including newer collection types (Account Decor, Account
  Transmogs).
- **FR-006**: An architecture document (`docs/architecture.md`) MUST be
  created describing the module layout, data flow, `bendpoint` proc macro
  role, Python code generator, and URL construction scheme.
- **FR-007**: An installation guide (`docs/installation.md`) MUST be created
  covering Rust toolchain setup, `.env` configuration, and how to build and
  test.
- **FR-008**: A usage guide (`docs/usage.md`) MUST be created showing how to
  use the library to make API calls, with at least one complete example.
- **FR-009**: `Cargo.toml` and `Cargo.lock` MUST be updated so all
  dependencies are at their latest compatible versions.
- **FR-010**: Any new formatting, lint, or clippy warnings introduced by the
  updates or discovered during audit MUST be resolved.
- **FR-011**: The combined specification (`docs/specification.md`) MUST be
  created or updated per the SDD constitution requirement.
- **FR-012**: Battle.net OAuth API endpoints MUST be reviewed against the
  current documentation and any discrepancies with `auth.rs`/`client.rs`
  noted and addressed.

### Key Entities

- **API Endpoint**: A single REST endpoint offered by Blizzard — defined by
  its HTTP method, path, namespace, and required parameters.
- **Model Struct**: A Rust struct that deserializes the JSON response from an
  API endpoint — may be generated via the `bendpoint` proc macro or the Python
  YAML generator.
- **Implementation Status**: The per-endpoint tracking of Model defined, URL
  generation implemented, and test written/passing.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `cargo fmt --check && cargo clippy --all-targets --all-features
  -- -D warnings && cargo test` exits 0 on the completed branch.
- **SC-002**: `ModelImplementProgress.md` lists 100% of endpoints from the
  current official Blizzard WoW Game Data and Profile API documentation pages,
  with accurate status markers.
- **SC-003**: `docs/architecture.md` exists and covers all modules in `src/`,
  the `model-macro/` crate, the `pygen/` generator, and data flow.
- **SC-004**: `docs/installation.md` and `docs/usage.md` exist and are
  sufficient for a new developer to build, test, and use the library.
- **SC-005**: All Cargo dependencies are at their latest compatible versions
  with no newly introduced warnings.
- **SC-006**: Any API endpoint additions or changes discovered during research
  are documented with "New" or "Changed" tags in the progress document.

## Assumptions

- The developer has valid BattleNet API credentials (client ID and secret)
  configured in a `.env` file — obtaining credentials is out of scope for
  this spec.
- Rust stable toolchain (edition 2021) is installed and available.
- The BattleNet API is accessible and returning data (i.e., no extended
  maintenance window during this sprint).
- World of Warcraft Classic APIs are out of scope for this spec — they will
  be addressed in a future spec per the project's README stretch goals.
- The Python code generator (`pygen/`) is documented in the architecture
  document but not modified in this spec — updating the generator itself is
  future work.
- The `bendpoint` proc macro crate is documented but not modified — expanding
  the macro is future work.

## Polish Phase Checklist *(SDD/TDD — mandatory)*

The following MUST be completed before the feature branch is merged:

- [ ] `docs/specification.md` updated with changes from this spec
- [ ] `docs/architecture.md` updated to reflect any structural changes
- [ ] `docs/installation.md` updated if setup steps changed
- [ ] `docs/usage.md` updated with new usage examples
- [ ] All tests written first (TDD) and passing
- [ ] Pre-commit suite passes clean (CI variant)
