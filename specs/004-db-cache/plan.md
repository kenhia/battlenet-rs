# Implementation Plan: Database Cache Layer & Rate Limiting

**Branch**: `004-db-cache` | **Date**: 2026-04-10 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/004-db-cache/spec.md`

## Summary

Add a database-backed cache layer (`CachedClient`) and token-bucket rate limiter to the battlenet-rs library. The cache uses `sqlx` with compile-time feature flags for SQLite (`db-sqlite`) and Postgres (`db-postgres`). Static endpoints serve from cache on hit; dynamic/profile endpoints always call the API but cache results. Character data enforces a 30-day TTL per Blizzard ToS 2.R. All model structs gain `Serialize` for cache round-tripping. Rate limiting uses a dual-window (per-second + per-hour) queue with configurable "nice" mode.

## Technical Context

**Language/Version**: Rust 1.94.0 (stable, edition 2021)
**Primary Dependencies**: reqwest 0.12, serde/serde_json 1.x, tokio 1.x, thiserror 1.x, model-macro (local proc-macro), **sqlx 0.8** (new), **chrono 0.4** (existing)
**Storage**: SQLite (WAL mode) via `db-sqlite` feature; PostgreSQL via `db-postgres` feature
**Testing**: `cargo test --all-features` (unit tests with fixture JSON; integration tests with live API)
**Target Platform**: Linux (primary), cross-platform via Rust
**Project Type**: Library (Rust crate)
**Constraints**: 100 req/s and 36,000 req/hr API limits; 30-day character data TTL (ToS 2.R)
**Aspirational Performance**: Cache hits <1ms; rate limiter overhead <100μs (not formally benchmarked)
**Scale/Scope**: ~167 endpoints, 158 bendpoint structs, 18 examples

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Verify each principle before implementation begins and again after design:

- [x] **I. SDD** — Feature spec exists in `/specs/004-db-cache/spec.md`
- [x] **II. TDD** — Test plan documented; tests will be written before implementation (fixture-based unit tests for cache store, rate limiter, and serialization round-trips)
- [x] **III. Code Quality** — Pre-commit suite confirmed runnable: `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test`
- [x] **IV. User Docs** — `docs/installation.md` and `docs/usage.md` updates are in scope (new deps, feature flags, CachedClient usage)
- [x] **V. Architecture** — `docs/architecture.md` update is in scope for polish phase (cache layer, rate limiter, new modules)

## Project Structure

### Documentation (this feature)

```text
specs/004-db-cache/
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
├── lib.rs               # Add cache, rate_limiter module declarations
├── client.rs            # Add Option<Arc<RateLimiter>> field + acquire() in send methods
├── errors.rs            # Add cache/rate-limit error variants
├── namespace.rs         # Existing — untouched
├── cache/
│   ├── mod.rs           # CacheStore trait, CacheEntry struct, CacheError enum
│   ├── cached_client.rs # CachedClient<S: CacheStore> wrapper
│   ├── sqlite.rs        # SQLite CacheStore impl (feature: db-sqlite)
│   └── postgres.rs      # Postgres CacheStore impl (feature: db-postgres)
├── rate_limiter.rs      # RateLimiter struct (dual-window token bucket)
└── wow_models/          # Add Serialize to bendpoint macro + manual structs
    └── ...

model-macro/
└── src/lib.rs           # Update bendpoint to emit Serialize + Deserialize

tests/
├── cache_test.rs        # CacheStore trait + CachedClient tests
├── rate_limiter_test.rs # Rate limiter unit tests
└── serialize_test.rs    # Model round-trip tests
```

**Structure Decision**: New `cache/` module directory under `src/` for cache layer (trait + backend impls + CachedClient), gated behind `db-sqlite`/`db-postgres` features. Rate limiter is a single top-level module compiled unconditionally (no feature gate — it has no sqlx dependency). Tests in `tests/` directory alongside existing test files.

## Constitution Check — Post-Design

*Re-evaluated after Phase 1 design artifacts completed.*

- [x] **I. SDD** — Spec exists; plan, research, data-model, contracts, and quickstart all generated
- [x] **II. TDD** — Test files planned: `cache_test.rs`, `rate_limiter_test.rs`, `serialize_test.rs`; fixture-based approach confirmed in research (R1, R5)
- [x] **III. Code Quality** — No new linting concerns; all new code will follow existing patterns (thiserror for errors, feature-gated modules)
- [x] **IV. User Docs** — `quickstart.md` created; `docs/installation.md` and `docs/usage.md` updates in scope
- [x] **V. Architecture** — Module structure documented in Project Structure section; CachedClient wrapper pattern (R3) preserves existing BattleNetClient interface

All gates pass. No violations detected.

## Complexity Tracking

No constitution violations. All five principles are satisfied.
