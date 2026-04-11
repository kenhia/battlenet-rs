# Project Specification: battlenet-rs

**Version**: 0.1.0
**Last Updated**: 2026-04-08

## Purpose

battlenet-rs is a Rust library that wraps the Blizzard BattleNet REST APIs for
World of Warcraft. It provides type-safe access to Game Data and Profile APIs
with automatic OAuth token management.

## Goals

1. Provide a complete, type-safe Rust wrapper for all WoW Game Data and Profile API endpoints
2. Handle OAuth client-credentials authentication transparently
3. Support all BattleNet regions (US, EU, KR, TW, CN) and locales
4. Minimize boilerplate through the `bendpoint` proc macro and `pygen` code generator
5. Maintain high code quality through mandatory pre-commit checks

## Current State

The library covers ~167 endpoints across 50 API categories (30 Game Data + 18
Profile + core). All endpoint models are gated behind cargo feature flags:
`wow` for Game Data APIs, `user` (requires `wow`) for Profile APIs. See
[ModelImplementProgress.md](../ModelImplementProgress.md) for the complete
inventory.

### Feature Flags

| Flag | Purpose |
|------|---------|
| `wow` | Enable WoW Game Data API models (~130 endpoints across 33 modules) |
| `user` | Enable WoW Profile API models (~37 endpoints across 17 modules; requires `wow`) |
| `redis` | Enable Redis-based user token reader (`src/user_token.rs`) |
| `stubs` | Reserved for future classic/other game stubs |

Default (no features) = core client, auth, region, namespace, and error types only.

## Architecture

The library follows a single-crate design with a companion proc-macro crate:

- **Core client** (`src/client.rs`): HTTP client with OAuth token caching
- **Endpoint models** (`src/wow_models/`): Serde structs implementing `GenerateUrl` trait
- **Code generation** (`model-macro/`): `bendpoint` proc macro for boilerplate reduction
- **Python tooling** (`pygen/`): YAML-to-Rust code generator for bulk endpoint creation

See [docs/architecture.md](architecture.md) for detailed technical documentation.

## Quality Standards (Constitution v1.0.0)

The project is governed by 5 constitutional principles:

1. **Specification-Driven Development** — Every feature starts with a spec in `specs/`
2. **Test-Driven Development** — TDD (Red-Green-Refactor) is mandatory for new code
3. **Code Quality** — Pre-commit suite must pass:
   - `cargo fmt --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo test`
4. **User Documentation** — Installation and usage docs maintained from the start
5. **Architecture Currency** — Architecture docs updated with every structural change

## Completed Specifications

### 001: Repo Baseline Audit & BattleNet API Research

**Status**: Complete
**Branch**: `001-repo-baseline-audit`
**Purpose**: Establish a green CI baseline, document the current repository
state, research the current BattleNet API surface, update dependencies, and
create foundational project documentation.

Key outcomes:
- Pre-commit CI suite passing clean
- Accurate endpoint inventory in ModelImplementProgress.md
- 2 new API endpoints identified (Account Decor Collection, Account Transmog Collection)
- Dependencies refreshed to latest compatible versions
- Architecture, installation, and usage documentation created

### 002: bnauth — Battle.net User OAuth Helper

**Status**: Complete
**Branch**: `002-bnauth-oauth-helper`
**Purpose**: Enable user-scoped Battle.net API access by building a Python
Flask app that performs the OAuth authorization code flow and stores the
resulting token in Redis for cross-machine consumption.

Key outcomes:
- Python Flask app (`bnauth/`) performs OAuth authorization code flow via browser
- User access token stored in Redis on `rpi53` with `bnauth:` key prefix and
  TTL matching token lifetime (~24h)
- Feature-gated Rust module (`src/user_token.rs`) reads the token from Redis
  behind the `redis` cargo feature flag
- User token remains architecturally separate from the client credentials token
- CN region explicitly excluded from bnauth scope

Deliverables:
1. **bnauth Flask app** — 3 routes (`/`, `/authorize`, `/callback`), Jinja2
   templates, environment-driven config, fail-fast on missing env vars
2. **Rust Redis token reader** — `UserAccessToken` struct,
   `read_user_token()` function, `RedisError` and `UserTokenNotAvailable`
   error variants

### 003: WoW Game Data & Profile API Models

**Status**: Complete
**Branch**: `003-lib-wow-examples`
**Purpose**: Implement comprehensive WoW API coverage with feature-gated
modules, extended UrlArgs, user-token client methods, and working examples.

Key outcomes:
- Feature flags (`wow`, `user`, `redis`, `stubs`) gate module compilation
- 33 Game Data model modules (~130 endpoints) behind `wow` flag
- 17 Profile API model modules (~37 endpoints) behind `user` flag
- UrlArgs extended with Guild, TwoIds, ThreeIds, PlayerExtra, TwoStrings, Search variants
- `get_data_with_token` / `get_json_with_token` client methods for Profile APIs
- 15 runnable examples covering achievements, auctions, characters, items, mounts, etc.
- Updated pygen code generator for bulk model creation
