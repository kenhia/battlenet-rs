# Implementation Plan: Library Setup, WoW Retail API Coverage, and Examples

**Branch**: `003-lib-wow-examples` | **Date**: 2026-04-09 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/003-lib-wow-examples/spec.md`

## Summary

Restructure battlenet-rs around Cargo feature flags (`default`, `wow`, `user`, stubs) so consumers compile only what they need, implement all ~169 remaining WoW retail endpoints (130 Game Data + 39 Profile) using the existing `bendpoint` macro and `pygen` code generator, and provide ≥15 runnable examples covering major endpoint groups.

## Technical Context

**Language/Version**: Rust 1.94.0 (stable, edition 2021)
**Primary Dependencies**: reqwest 0.12, serde/serde_json 1.x, tokio 1.x, thiserror 1.x, model-macro (local proc-macro crate)
**Storage**: N/A (database explicitly out of scope)
**Testing**: cargo test (unit tests with JSON fixtures in `data/`; integration tests against live API in `tests/`)
**Target Platform**: Any platform supporting Rust stable (library crate)
**Project Type**: Library
**Performance Goals**: N/A (wrapper library; latency dominated by upstream API)
**Constraints**: Must preserve backward compatibility with existing public API; `redis` feature untouched
**Scale/Scope**: ~167 new endpoint models across 30+ API categories; ~15+ examples

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **I. SDD** — Feature spec exists in `specs/003-lib-wow-examples/spec.md`
- [x] **II. TDD** — Test plan documented; tests will be written before implementation (unit tests for each endpoint model verifying JSON deserialization against fixture files)
- [x] **III. Code Quality** — Pre-commit suite confirmed runnable: `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test`
- [x] **IV. User Docs** — `docs/installation.md` and `docs/usage.md` updates are in scope (feature flag docs, new endpoint examples)
- [x] **V. Architecture** — `docs/architecture.md` update is in scope for polish phase (feature-gated module tree)

## Project Structure

### Documentation (this feature)

```text
specs/003-lib-wow-examples/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (public API surface)
└── tasks.md             # Phase 2 output (created by /speckit.tasks)
```

### Source Code (repository root)

```text
Cargo.toml                     # Feature flags: default, wow, user, redis, stubs

src/
├── lib.rs                     # Crate root — #[cfg(feature)] gates for modules
├── auth.rs                    # Core: OAuth token response (always compiled)
├── client.rs                  # Core: BattleNetClient + get_data/get_json
├── errors.rs                  # Core: BattleNetClientError enum
├── namespace.rs               # Core: WowNamespace (Static, Dynamic, Profile)
├── region.rs                  # Core: BattleNetRegion enum
├── user_token.rs              # [redis] UserAccessToken + Redis reader
└── wow_models/                # [wow] All WoW retail models
    ├── mod.rs (wow_models.rs) # UrlArgs enum, GenerateUrl trait, prelude
    ├── core_structs.rs        # Shared serde structs (HrefLink, NameAndId, etc.)
    ├── core_other.rs          # Re-exports
    ├── achievement.rs         # (existing, 5 endpoints)
    ├── auction_house.rs       # (existing orphan → integrate, 2 endpoints)
    ├── azerite_essence.rs     # NEW — 4 endpoints
    ├── character_profile.rs   # (existing, 2 endpoints)
    ├── connected_realm.rs     # (existing, 2 endpoints → add search)
    ├── covenant.rs            # NEW — 7 endpoints
    ├── creature.rs            # NEW — 7 endpoints
    ├── guild.rs               # NEW — 4 Guild + 3 Guild Crest = 7 endpoints
    ├── heirloom.rs            # NEW — 2 endpoints
    ├── item.rs                # NEW — 8 endpoints
    ├── journal.rs             # NEW — 8 endpoints
    ├── media_search.rs        # NEW — 1 endpoint
    ├── modified_crafting.rs   # NEW — 5 endpoints
    ├── mount.rs               # NEW — 3 endpoints
    ├── mythic_keystone.rs     # NEW — 10 endpoints (affix+dungeon+leaderboard+raid)
    ├── pet.rs                 # NEW — 6 endpoints
    ├── playable_class.rs      # NEW — 4 endpoints
    ├── playable_race.rs       # NEW — 2 endpoints
    ├── playable_spec.rs       # NEW — 3 endpoints
    ├── power_type.rs          # NEW — 2 endpoints
    ├── profession.rs          # NEW — 6 endpoints
    ├── pvp.rs                 # NEW — 8 endpoints (season+tier)
    ├── quest.rs               # NEW — 8 endpoints
    ├── realm.rs               # NEW — 3 endpoints
    ├── region_api.rs          # NEW — 2 endpoints
    ├── reputation.rs          # NEW — 4 endpoints
    ├── spell.rs               # NEW — 3 endpoints
    ├── talent.rs              # NEW — 12 endpoints (talent+tech talent)
    ├── title.rs               # NEW — 2 endpoints
    ├── toy.rs                 # NEW — 2 endpoints
    ├── wow_token.rs           # (existing, 1 endpoint)
    │
    │   # Profile API modules (require wow+user features)
    ├── account_profile.rs     # NEW — 9 endpoints
    ├── character_achievements.rs  # NEW — 2 endpoints
    ├── character_appearance.rs    # NEW — 1 endpoint
    ├── character_collections.rs   # NEW — 5 endpoints
    ├── character_encounters.rs    # NEW — 3 endpoints
    ├── character_equipment.rs     # NEW — 1 endpoint
    ├── character_hunter_pets.rs   # NEW — 1 endpoint
    ├── character_media.rs         # NEW — 1 endpoint
    ├── character_mythic_keystone.rs # NEW — 2 endpoints
    ├── character_professions.rs   # NEW — 1 endpoint
    ├── character_pvp.rs           # NEW — 2 endpoints
    ├── character_quests.rs        # NEW — 2 endpoints
    ├── character_reputations.rs   # NEW — 1 endpoint
    ├── character_soulbinds.rs     # NEW — 1 endpoint
    ├── character_specializations.rs # NEW — 1 endpoint
    ├── character_statistics.rs    # NEW — 1 endpoint
    └── character_titles.rs        # NEW — 1 endpoint

model-macro/src/
├── lib.rs                     # bendpoint proc macro (extend UrlArgs matching)
└── input.rs                   # Macro attribute parser

tests/                         # Integration tests (one file per module or group)
data/                          # JSON fixture files for unit tests

examples/                      # One per major endpoint group (≥15 total)
```

**Structure Decision**: Single-crate library with feature-gated module subtrees. WoW Game Data models live under `src/wow_models/` gated by `#[cfg(feature = "wow")]`. Profile API models live in the same directory but additionally require `user` feature. The `model-macro` proc-macro crate remains separate (required by Rust). Examples use `[[example]]` entries in `Cargo.toml` with `required-features`.

## Complexity Tracking

No constitution violations to justify.
