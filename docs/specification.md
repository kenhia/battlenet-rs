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

The library is functional with 10 implemented endpoints covering 4 API
categories (Achievement, Character Profile, Connected Realm, WoW Token). The
full BattleNet API surface includes approximately 100+ endpoints across 30+
categories. See [ModelImplementProgress.md](../ModelImplementProgress.md) for
the complete inventory.

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

**Status**: In Progress
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

## Planned Work

- **002 (planned)**: Local database tool for WoW character data — populate and
  maintain a local database of information on WoW characters
