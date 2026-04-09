# Implementation Plan: Repo Baseline Audit & BattleNet API Research

**Branch**: `001-repo-baseline-audit` | **Date**: 2026-04-07 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-repo-baseline-audit/spec.md`

## Summary

Audit the dormant battlenet-rs codebase to establish a green baseline (compile,
lint, test), update the API endpoint inventory against current Blizzard
documentation, refresh dependencies, and create the foundational project
documentation (architecture, installation, usage guides) required by the
constitution before resuming active development.

## Technical Context

**Language/Version**: Rust 1.94.0 (stable, edition 2021)  
**Primary Dependencies**: reqwest 0.12, serde/serde_json 1.x, tokio 1.x,
chrono 0.4, thiserror 1.x, dotenvy 0.15, model-macro (local proc-macro crate)  
**Storage**: N/A (API client library; local `.env` for credentials)  
**Testing**: `cargo test` (8 existing async tokio tests hitting live API)  
**Target Platform**: Linux (primary), cross-platform Rust library  
**Project Type**: Library (Rust crate wrapping BattleNet REST APIs)  
**Performance Goals**: N/A for this audit spec  
**Constraints**: Tests require valid BattleNet API credentials in `.env`; live
API must be accessible  
**Scale/Scope**: ~10 source files, 10 implemented endpoints, ~100+ TBD
endpoints catalogued  

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **I. SDD** — Feature spec exists at `specs/001-repo-baseline-audit/spec.md`
- [x] **II. TDD** — Test plan documented; existing tests validated first,
  any fixes follow TDD; this spec is primarily audit/docs, not new features
- [x] **III. Code Quality** — Pre-commit suite confirmed runnable; known
  issues: `cargo fmt` has 3 diffs, clippy has 1 dead-code error (`Ack` struct)
  — both will be fixed in Phase 1
- [x] **IV. User Docs** — `docs/installation.md` and `docs/usage.md` creation
  is in scope (User Story 5)
- [x] **V. Architecture** — `docs/architecture.md` creation is in scope
  (User Story 2)

## Project Structure

### Documentation (this feature)

```text
specs/001-repo-baseline-audit/
├── plan.md              # This file
├── research.md          # Phase 0: API delta research
├── data-model.md        # Phase 1: Entity inventory
├── quickstart.md        # Phase 1: Quick verification steps
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── auth.rs              # OAuth token management
├── client.rs            # BattleNetClient HTTP wrapper
├── errors.rs            # Error types
├── lib.rs               # Crate root, module declarations
├── namespace.rs         # WoW namespace enum (static/dynamic/profile)
├── region.rs            # BattleNet region enum + URLs
└── wow_models/
    ├── achievement.rs       # 5 endpoints ✅
    ├── auction_house.rs     # Models only, no endpoints yet
    ├── character_profile.rs # 2 endpoints ✅ (1 test missing)
    ├── connected_realm.rs   # 2 endpoints ✅
    ├── core_other.rs        # Re-exports
    ├── core_structs.rs      # Shared structs (16 types)
    └── wow_token.rs         # 1 endpoint ✅

model-macro/
└── src/
    ├── lib.rs           # bendpoint proc macro
    └── input.rs         # Macro attribute parser

pygen/
├── gen_models.py        # YAML → Rust code generator
├── sort_model_yaml.py   # YAML sorter utility
└── models/
    ├── core_structs.yaml
    ├── achievement.yaml
    └── auction_house.yaml

tests/
├── common.rs                # Test setup (client from .env)
├── achievements_test.rs     # 5 tests
├── connected_realm_test.rs  # 2 tests
└── wow_token_test.rs        # 1 test

examples/
├── get-client-token.rs      # Token + API demo
├── char-profile.rs          # Character profile demo
├── auction-play.rs          # Auction data parsing demo
└── panic-test.rs            # Debug/test scaffold

docs/                        # To be created in this spec
```

**Structure Decision**: Single-crate library with companion proc-macro crate.
Existing layout is retained; `docs/` directory created for new documentation.

## Baseline Issues Found

These issues MUST be fixed before proceeding with implementation:

| Issue | File | Description | Fix |
|-------|------|-------------|-----|
| Formatting | `examples/auction-play.rs` | Extra blank line before `fn main()` | `cargo fmt` |
| Formatting | `src/wow_models/character_profile.rs` | Extra blank lines around commented code | `cargo fmt` |
| Formatting | `src/wow_models/wow_token.rs` | Extra blank line before `#[bendpoint]` | `cargo fmt` |
| Dead code | `src/wow_models/character_profile.rs:67` | Unused `Ack` struct | Remove struct |
