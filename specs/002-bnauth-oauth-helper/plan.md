# Implementation Plan: bnauth — Battle.net User OAuth Helper

**Branch**: `002-bnauth-oauth-helper` | **Date**: 2026-04-08 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/002-bnauth-oauth-helper/spec.md`

## Summary

Build a Python Flask app (`bnauth/`) that performs the Battle.net OAuth
authorization code flow via a browser on `cleo`, stores the resulting user
access token in Redis on `rpi53`, and add a feature-gated Redis token reader
to `battlenet-rs` so `kubs0` can consume the token for user-scoped API calls.

## Technical Context

### Deliverable 1: bnauth Flask App (Python)

**Language/Version**: Python 3.13+ (managed with `uv`)
**Primary Dependencies**: Flask, redis, requests, python-dotenv
**Storage**: Redis on `rpi53` (host: `rpi53`, port: 6379, auth: `REDISCLI_AUTH` env var)
**Testing**: pytest + ruff format + ruff check + ty check (constitution Python supplement)
**Target Platform**: Windows (`cleo`) — local dev tool, also runnable on Linux
**Project Type**: Web app (single-page, local-only)
**Performance Goals**: N/A — single-user local tool
**Constraints**: Requires browser; token expires in 24h with no refresh token
**Scale/Scope**: 1 route (index), 1 callback route, ~150 lines of Python

### Deliverable 2: Redis Token Reader (Rust)

**Language/Version**: Rust 1.94.0+ (stable, edition 2021)
**Primary Dependencies**: redis crate (feature-gated), existing battlenet-rs deps
**Storage**: Reads from same Redis on `rpi53`
**Testing**: cargo test (integration test requires live Redis)
**Target Platform**: Linux (`kubs0`), cross-platform Rust library
**Project Type**: Library extension (optional feature)
**Performance Goals**: N/A
**Constraints**: Redis must be reachable from `kubs0`; feature-gated to avoid mandatory redis dependency
**Scale/Scope**: ~1 new module, ~100 lines of Rust

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **I. SDD** — Feature spec exists at `specs/002-bnauth-oauth-helper/spec.md`
- [x] **II. TDD** — Test plan documented; Python tests via pytest, Rust tests
  via cargo test; tests written before implementation per TDD mandate
- [x] **III. Code Quality** — Pre-commit suites confirmed:
  - Rust: `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test`
  - Python: `ruff format --check && ruff check && ty check && pytest -q`
- [x] **IV. User Docs** — `docs/installation.md` and `docs/usage.md` updates
  in scope (bnauth setup + Redis token reader usage)
- [x] **V. Architecture** — `docs/architecture.md` update in scope (bnauth
  component, Redis integration, user token flow)

## Project Structure

### Documentation (this feature)

```text
specs/002-bnauth-oauth-helper/
├── plan.md              # This file
├── research.md          # Phase 0: OAuth flow & Redis patterns
├── data-model.md        # Phase 1: Redis key schema, token entity
├── quickstart.md        # Phase 1: Quick verification steps
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
bnauth/                          # Python Flask app (Deliverable 1)
├── pyproject.toml               # uv-managed project metadata + deps
├── .python-version              # Python version pin for uv
├── .env-EXAMPLE                 # Example env vars
├── bnauth/
│   ├── __init__.py
│   └── app.py                   # Flask app (routes, OAuth, Redis)
├── templates/
│   ├── index.html               # Landing page with auth button
│   ├── success.html             # Token stored confirmation
│   └── error.html               # Error display with retry link
└── tests/
    └── test_app.py              # pytest tests

src/                             # Rust library (existing)
├── user_token.rs                # NEW: Redis token reader (feature-gated)
├── auth.rs                      # Existing OAuth client credentials
├── client.rs                    # Existing BattleNetClient
├── errors.rs                    # Extended with Redis error variants
├── lib.rs                       # Updated: conditionally include user_token
└── wow_models/                  # Existing (unchanged)

tests/
├── user_token_test.rs           # NEW: Redis token reader integration test
├── achievements_test.rs         # Existing (unchanged)
├── connected_realm_test.rs      # Existing (unchanged)
├── wow_token_test.rs            # Existing (unchanged)
└── common.rs                    # Existing (unchanged)
```

**Structure Decision**: The `bnauth/` Flask app is a sub-project within the
`battlenet-rs` repo, managed by `uv` with its own `pyproject.toml`. The Rust
Redis reader is added as a new module `src/user_token.rs`, conditionally
compiled behind the `redis` cargo feature flag.

## Baseline Issues

None. The codebase is in a clean state after spec 001 (green CI baseline).
`cargo fmt --check && cargo clippy -- -D warnings && cargo test` all pass.
