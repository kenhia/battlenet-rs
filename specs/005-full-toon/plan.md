# Implementation Plan: Full Character Download

**Branch**: `005-full-toon` | **Date**: 2026-04-12 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/005-full-toon/spec.md`

## Summary

Add a composite `FullCharacter` struct and associated download functions to the
battlenet-rs library. A single `full_character()` call fetches all 28 character
profile endpoints (24 Player + 4 PlayerExtra), assembles results into
a typed composite with `Option<T>` fields for graceful degradation, and returns
the whole package. A JSON variant (`full_character_json()`) serializes the struct.
Cache-aware variants use the existing `CachedClient` from sprint 004. All profile
endpoints are accessed with `get_data()` (client credentials) for the base data;
when a user token is provided, additional data is fetched and `has_profile_data`
is set to `true`.

## Technical Context

**Language/Version**: Rust 1.94.0 (stable, edition 2021)
**Primary Dependencies**: reqwest 0.12, serde/serde_json 1.x, tokio 1.x, thiserror 1.x, chrono 0.4, model-macro (local proc-macro)
**Storage**: N/A (uses existing cache layer from sprint 004 when db-sqlite/db-postgres enabled)
**Testing**: `cargo test --features "wow,user,db-sqlite"` (unit tests with fixture JSON; integration tests with live API)
**Target Platform**: Linux (primary), cross-platform via Rust
**Project Type**: Library (Rust crate)
**Constraints**: 100 req/s and 36,000 req/hr API limits (handled by existing rate limiter)
**Scale/Scope**: 28 character profile endpoints per download; ~167 total endpoints in library

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **I. SDD** — Feature spec exists in `/specs/005-full-toon/spec.md`
- [x] **II. TDD** — Test plan documented; tests will be written before implementation (fixture-based unit tests for struct assembly, JSON round-trip, error handling)
- [x] **III. Code Quality** — Pre-commit suite confirmed runnable: `cargo fmt --check && cargo clippy --features "wow,db-sqlite,user" --lib --tests -- -D warnings && cargo test --features "wow,db-sqlite,user" --lib --tests`
- [x] **IV. User Docs** — `docs/usage.md` update in scope (full_character usage, example)
- [x] **V. Architecture** — `docs/architecture.md` update in scope for polish phase (new full_character module)

## Project Structure

### Documentation (this feature)

```text
specs/005-full-toon/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
src/
├── lib.rs               # Unchanged (full_character is within wow_models)
├── client.rs            # Unchanged
├── errors.rs            # Unchanged
├── wow_models/
│   ├── full_character.rs # NEW — FullCharacter struct, EndpointError, download functions
│   └── ...              # Existing character_*.rs modules (unchanged)
└── wow_models.rs        # Add full_character module declaration + prelude re-exports

examples/
└── full-toon.rs         # NEW — example demonstrating full character download

tests/
└── full_character_test.rs # NEW — unit tests for FullCharacter assembly and JSON round-trip
```

**Structure Decision**: Single new module `src/wow_models/full_character.rs` alongside existing character modules. This follows the pattern of other wow_models modules and keeps the composite logic close to the types it aggregate. Tests in `tests/` directory alongside existing test files.

## Constitution Check — Post-Design

*Re-evaluated after Phase 1 design artifacts completed.*

- [x] **I. SDD** — Spec exists; plan, research, data-model, contracts, and quickstart all generated
- [x] **II. TDD** — Test file planned: `full_character_test.rs`; fixture-based approach for struct assembly and JSON round-trip
- [x] **III. Code Quality** — No new linting concerns; all new code follows existing patterns
- [x] **IV. User Docs** — `quickstart.md` created; `docs/usage.md` update in scope
- [x] **V. Architecture** — Module structure documented; no new external dependencies

All gates pass. No violations detected.

## Complexity Tracking

No constitution violations. All five principles are satisfied.
