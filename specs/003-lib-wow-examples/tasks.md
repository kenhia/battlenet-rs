# Tasks: Library Setup, WoW Retail API Coverage, and Examples

**Input**: Design documents from `/specs/003-lib-wow-examples/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/public-api.md, quickstart.md

**Tests**: Per the constitution (§II TDD), test tasks MUST appear before their corresponding implementation tasks and MUST be confirmed failing before implementation begins. Unit tests use JSON fixture files in `data/` with `json_to_struct()` deserialization — no live API required.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Cargo feature flags, UrlArgs extensions, shared structs, macro extensions — the structural backbone that all user stories depend on.

- [X] T001 Add Cargo feature flags (`wow`, `user`, `wow-classic`, `diablo`, `hearthstone`, `starcraft` stubs) to `Cargo.toml`
- [X] T002 Gate `wow_models` module behind `#[cfg(feature = "wow")]` in `src/lib.rs`
- [X] T003 Add new UrlArgs variants (`Guild`, `TwoIds`, `ThreeIds`, `PlayerExtra`, `TwoStrings`, `Search`) to `src/wow_models.rs`
- [X] T004 Add `SearchResult<T>` and `SearchResultEntry<T>` generic structs to `src/wow_models/core_structs.rs`; include unit test for empty `results: []` deserialization
- [X] T005 [P] Add any additional shared core structs (`KeyHref`, `Media`, etc.) to `src/wow_models/core_structs.rs`
- [X] T006 [P] Extend `bendpoint` macro in `model-macro/src/lib.rs` and `model-macro/src/input.rs` to support new UrlArgs variants (`Guild`, `TwoIds`, `ThreeIds`, `PlayerExtra`, `TwoStrings`, `Search`) and `profile` namespace
- [X] T007 Add `required-features = ["wow"]` to existing `[[example]]` entries in `Cargo.toml` and add `#[cfg(feature = "wow")]` gates to existing integration tests in `tests/`
- [X] T008 Integrate orphaned `src/wow_models/auction_house.rs` into the `wow_models` module tree (declare in `src/wow_models.rs`, add to prelude)
- [X] T009 Verify setup: run `cargo check`, `cargo check --features wow`, `cargo check --features wow,user`, `cargo check --all-features`, `cargo test --all-features`

**Checkpoint**: Feature flags compile, UrlArgs extended, macro updated, existing code still works under feature gates.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: User-token client methods and Profile API infrastructure that MUST be complete before Profile user stories (US3) can be implemented.

**⚠️ CRITICAL**: US3 (Profile APIs) cannot begin until this phase is complete. US2 (Game Data) can begin after Phase 1.

### Tests (TDD — write and confirm failing BEFORE implementation)

- [X] T010 Unit test for `send_request_with_token` in `src/client.rs` (test that the method constructs a request with the provided bearer token)
- [X] T011 Unit test for `get_data_with_token` and `get_json_with_token` in `src/client.rs`

### Implementation

- [X] T012 Implement `send_request_with_token(&self, url: String, token: &str)` method on `BattleNetClient` in `src/client.rs`
- [X] T013 Implement `get_data_with_token<T>` and `get_json_with_token<T>` methods on `BattleNetClient` in `src/client.rs`
- [X] T014 Add `#[cfg(feature = "user")]` conditional module declarations in `src/wow_models.rs` for all Profile API modules (stub `mod` declarations)
- [X] T015 Add Profile API modules to prelude in `src/wow_models.rs` behind `#[cfg(feature = "user")]`

**Checkpoint**: `BattleNetClient` supports user-token requests. Profile module slots exist. `cargo check --features wow,user` passes.

---

## Phase 3: User Story 1 — Selective Library Compilation via Feature Flags (Priority: P1) 🎯 MVP

**Goal**: Consumers compile only what they need. `default` = core only, `wow` = Game Data, `user` = Profile APIs. Stub features exist for forward compat.

**Independent Test**: Build with various feature combinations and verify only expected modules compile. Run `cargo test --all-features` and `cargo test` (no features).

### Tests for User Story 1 (TDD — MANDATORY)

- [X] T016 [P] [US1] Compile-gate test: verify `cargo check` (no features) succeeds and wow_models is NOT available, in a scratch test crate or via `trybuild` in `tests/feature_flags_test.rs`
- [X] T017 [P] [US1] Compile-gate test: verify `cargo check --features wow` succeeds and Game Data models ARE available in `tests/feature_flags_test.rs`
- [X] T018 [P] [US1] Compile-gate test: verify `cargo check --features wow,user` succeeds and Profile models ARE available in `tests/feature_flags_test.rs`
- [X] T019 [P] [US1] Compile-gate test: verify stub features (`wow-classic`, `diablo`, `hearthstone`, `starcraft`) compile with no effect in `tests/feature_flags_test.rs`
- [X] T020 [P] [US1] Regression test: verify `redis` feature still works alongside new features in `tests/feature_flags_test.rs`

### Implementation for User Story 1

- [X] T021 [US1] Verify and fix any remaining feature-gate issues across all modules — ensure clean compilation under every feature combination
- [X] T022 [US1] Verify existing tests pass under `--features wow` and `--all-features`

**Checkpoint**: Feature flags work correctly. All combinations compile cleanly. Existing tests pass under `--features wow`.

---

## Phase 4: User Story 2 — Querying WoW Game Data APIs (Priority: P1) 🎯 MVP

**Goal**: Implement typed model structs for all ~130 new WoW Game Data API endpoints. Each endpoint has a model struct, `GenerateUrl` impl, and type aliases.

**Independent Test**: For each new module, a JSON fixture unit test deserializes a captured API response and asserts key field values.

### Tests for User Story 2 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

**Batch 1 — Simple Index/Id modules (Static namespace, None + Id args)**

- [X] T023 [P] [US2] Unit test for `AzeriteEssence` endpoints: capture fixture JSON to `data/azerite-essence-index.json` and `data/azerite-essence-{id}.json`, write deserialization tests in `src/wow_models/azerite_essence.rs` (inline `#[cfg(test)]`) 
- [X] T024 [P] [US2] Unit test for `Covenant` endpoints (index + id + soulbind + conduit): capture fixtures, write tests in `src/wow_models/covenant.rs`
- [X] T025 [P] [US2] Unit test for `Heirloom` endpoints: capture fixtures, write tests in `src/wow_models/heirloom.rs`
- [X] T026 [P] [US2] Unit test for `ModifiedCrafting` endpoints: capture fixtures, write tests in `src/wow_models/modified_crafting.rs`
- [X] T027 [P] [US2] Unit test for `Mount` endpoints (index + id): capture fixtures, write tests in `src/wow_models/mount.rs`
- [X] T028 [P] [US2] Unit test for `Pet` endpoints (index + id + ability): capture fixtures, write tests in `src/wow_models/pet.rs`
- [X] T029 [P] [US2] Unit test for `PlayableClass` endpoints: capture fixtures, write tests in `src/wow_models/playable_class.rs`
- [X] T030 [P] [US2] Unit test for `PlayableRace` endpoints: capture fixtures, write tests in `src/wow_models/playable_race.rs`
- [X] T031 [P] [US2] Unit test for `PlayableSpec` endpoints: capture fixtures, write tests in `src/wow_models/playable_spec.rs`
- [X] T032 [P] [US2] Unit test for `PowerType` endpoints: capture fixtures, write tests in `src/wow_models/power_type.rs`
- [X] T033 [P] [US2] Unit test for `Quest` endpoints: capture fixtures, write tests in `src/wow_models/quest.rs`
- [X] T034 [P] [US2] Unit test for `Reputation` endpoints: capture fixtures, write tests in `src/wow_models/reputation.rs`
- [X] T035 [P] [US2] Unit test for `Title` endpoints: capture fixtures, write tests in `src/wow_models/title.rs`
- [X] T036 [P] [US2] Unit test for `Toy` endpoints: capture fixtures, write tests in `src/wow_models/toy.rs`

**Batch 2 — Modules with Dynamic namespace or TwoIds / Search / special args**

- [X] T037 [P] [US2] Unit test for `AuctionHouse` endpoints (fix orphan): capture fixtures, write tests in `src/wow_models/auction_house.rs`
- [X] T038 [P] [US2] Unit test for `ConnectedRealm` search endpoint: capture fixture for search, write test in `src/wow_models/connected_realm.rs`
- [X] T039 [P] [US2] Unit test for `Creature` endpoints (index + id + families + types + search): capture fixtures, write tests in `src/wow_models/creature.rs`
- [X] T040 [P] [US2] Unit test for `Guild` Game Data endpoints (guild crest): capture fixtures, write tests in `src/wow_models/guild.rs`
- [X] T041 [P] [US2] Unit test for `Item` endpoints (class + subclass + set + item + search): capture fixtures, write tests in `src/wow_models/item.rs`
- [X] T042 [P] [US2] Unit test for `Journal` endpoints (expansion + encounter + instance + search): capture fixtures, write tests in `src/wow_models/journal.rs`
- [X] T043 [P] [US2] Unit test for `MediaSearch` endpoint: capture fixture, write test in `src/wow_models/media_search.rs`
- [X] T044 [P] [US2] Unit test for `MythicKeystone` endpoints (affix + dungeon + leaderboard + raid): capture fixtures, write tests in `src/wow_models/mythic_keystone.rs`
- [X] T045 [P] [US2] Unit test for `Profession` endpoints (index + id + skill-tier + recipe): capture fixtures, write tests in `src/wow_models/profession.rs`
- [X] T046 [P] [US2] Unit test for `Pvp` endpoints (season + tier + leaderboard + reward): capture fixtures, write tests in `src/wow_models/pvp.rs`
- [X] T047 [P] [US2] Unit test for `Realm` endpoints (index + id + search): capture fixtures, write tests in `src/wow_models/realm.rs`
- [X] T048 [P] [US2] Unit test for `RegionApi` endpoints: capture fixtures, write tests in `src/wow_models/region_api.rs`
- [X] T049 [P] [US2] Unit test for `Spell` endpoints (id + media + search): capture fixtures, write tests in `src/wow_models/spell.rs`
- [X] T050 [P] [US2] Unit test for `Talent` endpoints (tree + talent + pvp talent + tech talent): capture fixtures, write tests in `src/wow_models/talent.rs`

### Implementation for User Story 2 (AFTER tests are written and failing)

**Batch 1 — Simple Index/Id modules (Static namespace)**

- [X] T051 [P] [US2] Implement `AzeriteEssence` model structs and `GenerateUrl` in `src/wow_models/azerite_essence.rs`; declare module in `src/wow_models.rs` and add to prelude
- [X] T052 [P] [US2] Implement `Covenant` model structs (covenant + soulbind + conduit) in `src/wow_models/covenant.rs`; declare and add to prelude
- [X] T053 [P] [US2] Implement `Heirloom` model structs in `src/wow_models/heirloom.rs`; declare and add to prelude
- [X] T054 [P] [US2] Implement `ModifiedCrafting` model structs in `src/wow_models/modified_crafting.rs`; declare and add to prelude
- [X] T055 [P] [US2] Implement `Mount` model structs in `src/wow_models/mount.rs`; declare and add to prelude
- [X] T056 [P] [US2] Implement `Pet` model structs (pet + pet ability) in `src/wow_models/pet.rs`; declare and add to prelude
- [X] T057 [P] [US2] Implement `PlayableClass` model structs in `src/wow_models/playable_class.rs`; declare and add to prelude
- [X] T058 [P] [US2] Implement `PlayableRace` model structs in `src/wow_models/playable_race.rs`; declare and add to prelude
- [X] T059 [P] [US2] Implement `PlayableSpec` model structs in `src/wow_models/playable_spec.rs`; declare and add to prelude
- [X] T060 [P] [US2] Implement `PowerType` model structs in `src/wow_models/power_type.rs`; declare and add to prelude
- [X] T061 [P] [US2] Implement `Quest` model structs in `src/wow_models/quest.rs`; declare and add to prelude
- [X] T062 [P] [US2] Implement `Reputation` model structs in `src/wow_models/reputation.rs`; declare and add to prelude
- [X] T063 [P] [US2] Implement `Title` model structs in `src/wow_models/title.rs`; declare and add to prelude
- [X] T064 [P] [US2] Implement `Toy` model structs in `src/wow_models/toy.rs`; declare and add to prelude

**Batch 2 — Modules with Dynamic namespace, TwoIds, Search, or special args**

- [X] T065 [P] [US2] Fix and complete `AuctionHouse` model structs in `src/wow_models/auction_house.rs` (add Commodities endpoint, add `GenerateUrl` impls); already declared in T008
- [X] T066 [P] [US2] Add `ConnectedRealm` search endpoint model to `src/wow_models/connected_realm.rs`
- [X] T067 [P] [US2] Implement `Creature` model structs (families, types, search, media) in `src/wow_models/creature.rs`; declare and add to prelude
- [X] T068 [P] [US2] Implement `Guild` Game Data model structs (guild crest components, border media, emblem media) in `src/wow_models/guild.rs`; declare and add to prelude
- [X] T069 [P] [US2] Implement `Item` model structs (classes, subclass with TwoIds, sets, item, media, search) in `src/wow_models/item.rs`; declare and add to prelude
- [X] T070 [P] [US2] Implement `Journal` model structs (expansion, encounter, instance, media, search) in `src/wow_models/journal.rs`; declare and add to prelude
- [X] T071 [P] [US2] Implement `MediaSearch` model struct in `src/wow_models/media_search.rs`; declare and add to prelude
- [X] T072 [P] [US2] Implement `MythicKeystone` model structs (affix, dungeon, leaderboard with ThreeIds, raid leaderboard with TwoStrings) in `src/wow_models/mythic_keystone.rs`; declare and add to prelude
- [X] T073 [P] [US2] Implement `Profession` model structs (index, profession, skill-tier with TwoIds, recipe, media) in `src/wow_models/profession.rs`; declare and add to prelude
- [X] T074 [P] [US2] Implement `Pvp` model structs (season, tier, leaderboard with TwoIds, reward) in `src/wow_models/pvp.rs`; declare and add to prelude
- [X] T075 [P] [US2] Implement `Realm` model structs (index, realm, search) in `src/wow_models/realm.rs`; declare and add to prelude
- [X] T076 [P] [US2] Implement `RegionApi` model structs in `src/wow_models/region_api.rs`; declare and add to prelude
- [X] T077 [P] [US2] Implement `Spell` model structs (spell, media, search) in `src/wow_models/spell.rs`; declare and add to prelude
- [X] T078 [P] [US2] Implement `Talent` model structs (tree, talent, pvp talent, tech talent tree, tech talent, media) in `src/wow_models/talent.rs`; declare and add to prelude
- [X] T079 [US2] Run `cargo test --features wow` — all Game Data unit tests pass

**Checkpoint**: All ~130 Game Data endpoint models implemented. Every model has a unit test with JSON fixture. `cargo test --features wow` passes.

---

## Phase 5: User Story 3 — Querying WoW Profile APIs with a User Token (Priority: P2)

**Goal**: Implement typed model structs for all ~37 new Profile API endpoints that require a user OAuth token. Each uses `WowNamespace::Profile` and the `get_data_with_token` / `get_json_with_token` methods.

**Independent Test**: For each new Profile module, a JSON fixture unit test deserializes a captured API response and asserts key field values.

### Tests for User Story 3 (TDD — MANDATORY: write and confirm failing BEFORE implementation)

- [X] T080 [P] [US3] Unit test for `AccountProfile` endpoints (9 endpoints): capture fixture JSON, write tests in `src/wow_models/account_profile.rs`
- [X] T081 [P] [US3] Unit test for `CharacterAchievements` endpoints: capture fixtures, write tests in `src/wow_models/character_achievements.rs`
- [X] T082 [P] [US3] Unit test for `CharacterAppearance` endpoint: capture fixture, write test in `src/wow_models/character_appearance.rs`
- [X] T083 [P] [US3] Unit test for `CharacterCollections` endpoints (5): capture fixtures, write tests in `src/wow_models/character_collections.rs`
- [X] T084 [P] [US3] Unit test for `CharacterEncounters` endpoints (3): capture fixtures, write tests in `src/wow_models/character_encounters.rs`
- [X] T085 [P] [US3] Unit test for `CharacterEquipment` endpoint: capture fixture, write test in `src/wow_models/character_equipment.rs`
- [X] T086 [P] [US3] Unit test for `CharacterHunterPets` endpoint: capture fixture, write test in `src/wow_models/character_hunter_pets.rs`
- [X] T087 [P] [US3] Unit test for `CharacterMedia` endpoint: capture fixture, write test in `src/wow_models/character_media.rs`
- [X] T088 [P] [US3] Unit test for `CharacterMythicKeystone` endpoints (2, including PlayerExtra): capture fixtures, write tests in `src/wow_models/character_mythic_keystone.rs`
- [X] T089 [P] [US3] Unit test for `CharacterProfessions` endpoint: capture fixture, write test in `src/wow_models/character_professions.rs`
- [X] T090 [P] [US3] Unit test for `CharacterProfile` (move existing to `user` gate): verify existing tests still pass under `--features wow,user` in `src/wow_models/character_profile.rs`
- [X] T091 [P] [US3] Unit test for `CharacterPvp` endpoints (2, including PlayerExtra): capture fixtures, write tests in `src/wow_models/character_pvp.rs`
- [X] T092 [P] [US3] Unit test for `CharacterQuests` endpoints (2): capture fixtures, write tests in `src/wow_models/character_quests.rs`
- [X] T093 [P] [US3] Unit test for `CharacterReputations` endpoint: capture fixture, write test in `src/wow_models/character_reputations.rs`
- [X] T094 [P] [US3] Unit test for `CharacterSoulbinds` endpoint: capture fixture, write test in `src/wow_models/character_soulbinds.rs`
- [X] T095 [P] [US3] Unit test for `CharacterSpecializations` endpoint: capture fixture, write test in `src/wow_models/character_specializations.rs`
- [X] T096 [P] [US3] Unit test for `CharacterStatistics` endpoint: capture fixture, write test in `src/wow_models/character_statistics.rs`
- [X] T097 [P] [US3] Unit test for `CharacterTitles` endpoint: capture fixture, write test in `src/wow_models/character_titles.rs`
- [X] T098 [P] [US3] Unit test for `Guild` Profile endpoints (4, using Guild UrlArgs): capture fixtures, write tests in `src/wow_models/guild.rs`

### Implementation for User Story 3 (AFTER tests are written and failing)

- [X] T099 [P] [US3] Implement `AccountProfile` model structs (9 endpoints, using None + TwoIds) in `src/wow_models/account_profile.rs`
- [X] T100 [P] [US3] Implement `CharacterAchievements` model structs in `src/wow_models/character_achievements.rs`
- [X] T101 [P] [US3] Implement `CharacterAppearance` model struct in `src/wow_models/character_appearance.rs`
- [X] T102 [P] [US3] Implement `CharacterCollections` model structs in `src/wow_models/character_collections.rs`
- [X] T103 [P] [US3] Implement `CharacterEncounters` model structs in `src/wow_models/character_encounters.rs`
- [X] T104 [P] [US3] Implement `CharacterEquipment` model struct in `src/wow_models/character_equipment.rs`
- [X] T105 [P] [US3] Implement `CharacterHunterPets` model struct in `src/wow_models/character_hunter_pets.rs`
- [X] T106 [P] [US3] Implement `CharacterMedia` model struct in `src/wow_models/character_media.rs`
- [X] T107 [P] [US3] Implement `CharacterMythicKeystone` model structs (PlayerExtra arg) in `src/wow_models/character_mythic_keystone.rs`
- [X] T108 [P] [US3] Implement `CharacterProfessions` model struct in `src/wow_models/character_professions.rs`
- [X] T109 [P] [US3] Move existing `CharacterProfile` behind `#[cfg(feature = "user")]` gate in `src/wow_models/character_profile.rs`
- [X] T110 [P] [US3] Implement `CharacterPvp` model structs (PlayerExtra arg) in `src/wow_models/character_pvp.rs`
- [X] T111 [P] [US3] Implement `CharacterQuests` model structs in `src/wow_models/character_quests.rs`
- [X] T112 [P] [US3] Implement `CharacterReputations` model struct in `src/wow_models/character_reputations.rs`
- [X] T113 [P] [US3] Implement `CharacterSoulbinds` model struct in `src/wow_models/character_soulbinds.rs`
- [X] T114 [P] [US3] Implement `CharacterSpecializations` model struct in `src/wow_models/character_specializations.rs`
- [X] T115 [P] [US3] Implement `CharacterStatistics` model struct in `src/wow_models/character_statistics.rs`
- [X] T116 [P] [US3] Implement `CharacterTitles` model struct in `src/wow_models/character_titles.rs`
- [X] T117 [P] [US3] Implement `Guild` Profile endpoint model structs (activity, achievements, roster, using Guild UrlArgs + Profile namespace) in `src/wow_models/guild.rs`; use `#[cfg(feature = "user")]` on individual structs/impls for Profile-only endpoints within the shared guild module
- [X] T118 [US3] Run `cargo test --features wow,user` — all Profile API unit tests pass

**Checkpoint**: All ~39 Profile API endpoint models implemented. Every model has a unit test with JSON fixture. `cargo test --features wow,user` passes.

---

## Phase 6: User Story 4 — Learning from Working Examples (Priority: P2)

**Goal**: At least 15 runnable examples in `examples/`, one per major endpoint group. Each example demonstrates `BattleNetClient` usage and is declared in `Cargo.toml` with `required-features`.

**Independent Test**: Each example compiles with `cargo build --example <name> --features <features>`. Examples with valid credentials run and produce output.

### Tests for User Story 4 (TDD — MANDATORY)

- [X] T119 [US4] Compile-check test: verify all examples compile with `cargo build --examples --all-features`

### Implementation for User Story 4

**Game Data Examples (require `--features wow`)**

- [X] T120 [P] [US4] Create example `examples/mounts.rs` — fetch mounts index and a specific mount; add `[[example]]` to `Cargo.toml`
- [X] T121 [P] [US4] Create example `examples/items.rs` — fetch item classes and a specific item; add `[[example]]` to `Cargo.toml`
- [X] T122 [P] [US4] Create example `examples/achievements.rs` — fetch achievement categories index and a specific achievement; add `[[example]]` to `Cargo.toml`
- [X] T123 [P] [US4] Create example `examples/professions.rs` — fetch professions index and a specific profession; add `[[example]]` to `Cargo.toml`
- [X] T124 [P] [US4] Create example `examples/pvp-seasons.rs` — fetch PvP seasons index and a specific season; add `[[example]]` to `Cargo.toml`
- [X] T125 [P] [US4] Create example `examples/realms.rs` — fetch realms index and a specific realm; add `[[example]]` to `Cargo.toml`
- [X] T126 [P] [US4] Create example `examples/creatures.rs` — fetch creature families and types; add `[[example]]` to `Cargo.toml`
- [X] T127 [P] [US4] Create example `examples/spells.rs` — fetch a spell by ID; add `[[example]]` to `Cargo.toml`
- [X] T128 [P] [US4] Create example `examples/talents.rs` — fetch talent tree index; add `[[example]]` to `Cargo.toml`
- [X] T129 [P] [US4] Create example `examples/pets.rs` — fetch pets index and a specific pet; add `[[example]]` to `Cargo.toml`
- [X] T130 [P] [US4] Create example `examples/mythic-keystone.rs` — fetch mythic keystone dungeon index and affixes; add `[[example]]` to `Cargo.toml`
- [X] T131 [P] [US4] Create example `examples/auctions.rs` — fetch auctions for a connected realm; add `[[example]]` to `Cargo.toml`

**Profile API Examples (require `--features wow,user`)**

- [X] T132 [P] [US4] Create example `examples/account-profile.rs` — fetch account profile summary with user token; add `[[example]]` to `Cargo.toml`
- [X] T133 [P] [US4] Create example `examples/character-collections.rs` — fetch character mounts and pets; add `[[example]]` to `Cargo.toml`
- [X] T134 [P] [US4] Create example `examples/character-equipment.rs` — fetch character equipment; add `[[example]]` to `Cargo.toml`

**Existing Examples**

- [X] T135 [US4] Update existing example `examples/char-profile.rs` to use `--features wow,user` pattern and user token
- [X] T136 [US4] Update existing example `examples/get-client-token.rs` — ensure it works without `wow` feature (core only)
- [X] T137 [US4] Re-run T119: verify all 15+ examples compile: `cargo build --examples --all-features`

**Checkpoint**: 15+ examples exist. All compile. Game Data examples run with client credentials. Profile examples document user token requirement.

---

## Phase 7: User Story 5 — Code Generation Compatibility (Priority: P3)

**Goal**: The `bendpoint` macro and `pygen` generator produce code compatible with the new feature-flag structure and extended UrlArgs.

**Independent Test**: Generate a model from YAML, verify it compiles under the correct feature flag.

### Tests for User Story 5 (TDD — MANDATORY)

- [X] T138 [P] [US5] Unit test for `bendpoint` macro with new UrlArgs variants (`Guild`, `TwoIds`, `ThreeIds`, `PlayerExtra`, `TwoStrings`, `Search`) — compile-test in `model-macro/` or `tests/`
- [X] T139 [P] [US5] Test: run `pygen` on an existing YAML model definition, verify output compiles under `--features wow`

### Implementation for User Story 5

- [X] T140 [US5] Verify `bendpoint` macro handles all new UrlArgs variants correctly (may already work from T006 — confirm and fix any edge cases in `model-macro/src/lib.rs`)
- [X] T141 [US5] Update `pygen/gen_models.py` to generate feature-flag-aware module declarations if needed
- [X] T142 [US5] Verify end-to-end: generate a model via YAML → pygen → compile under correct feature flag

**Checkpoint**: Macro and code generator work with the new feature-flag architecture.

---

## Phase 8: Polish & Cross-Cutting Concerns (Constitution — mandatory)

**Purpose**: Documentation updates, quality gates, and constitution compliance.

- [X] T143 Run full pre-commit suite (CI variant): `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all-features`
- [X] T144 [P] Update `docs/specification.md` — merge spec changes from this sprint
- [X] T145 [P] Update `docs/architecture.md` — document feature-gated module tree, Profile API token flow, UrlArgs enum
- [X] T146 [P] Update `docs/installation.md` — add feature flag usage instructions (`features = ["wow"]`, `features = ["wow", "user"]`, etc.)
- [X] T147 [P] Update `docs/usage.md` — add new endpoint examples, feature flag documentation, Profile API examples
- [X] T148 [P] Update `ModelImplementProgress.md` — mark all implemented endpoints as ✅
- [X] T149 Run quickstart.md validation: execute all verification steps from `specs/003-lib-wow-examples/quickstart.md`
- [X] T150 Final quality gate: `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all-features` — zero warnings, zero failures

**Checkpoint**: All docs updated. All tests pass. Pre-commit suite clean. Ready to ship.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 (T001–T009) — BLOCKS US3 (Profile APIs)
- **US1 (Phase 3)**: Depends on Phase 1 — can start after T009
- **US2 (Phase 4)**: Depends on Phase 1 — can start after T009 (independent of Phase 2)
- **US3 (Phase 5)**: Depends on Phase 1 AND Phase 2 (needs user-token methods)
- **US4 (Phase 6)**: Depends on US2 and US3 (needs endpoints to exist)
- **US5 (Phase 7)**: Depends on Phase 1 (T006 macro extension)
- **Polish (Phase 8)**: Depends on all user stories being complete

### User Story Dependencies

- **US1 (Feature Flags)** → Independent, can start after Phase 1
- **US2 (Game Data)** → Independent, can start after Phase 1; **can run in parallel with US1 and US3**
- **US3 (Profile APIs)** → Depends on Phase 2 (user-token client methods); **can run in parallel with US2**
- **US4 (Examples)** → Depends on US2 + US3 implementations existing
- **US5 (Code Gen)** → Independent after Phase 1 T006; **can run in parallel with US2/US3**

### Within Each User Story

1. Tests MUST be written and FAIL before implementation (TDD — non-negotiable per constitution §II)
2. JSON fixtures captured from live API before test code
3. Model structs before `GenerateUrl` impls
4. Module declared in `wow_models.rs` and added to prelude
5. All tests green before story is considered complete

### Parallel Opportunities

**Maximum parallelism within US2**: All 28 test tasks (T023–T050) can run in parallel. All 28 implementation tasks (T051–T078) can run in parallel (each is a separate `.rs` file with no inter-dependencies).

**Cross-story parallelism**: US1, US2, US3, and US5 can all proceed in parallel after their prerequisites are met.

---

## Summary

| Phase | Story | Task Range | Task Count |
|-------|-------|------------|------------|
| 1 - Setup | — | T001–T009 | 9 |
| 2 - Foundational | — | T010–T015 | 6 |
| 3 - US1 Feature Flags | US1 | T016–T022 | 7 |
| 4 - US2 Game Data | US2 | T023–T079 | 57 |
| 5 - US3 Profile APIs | US3 | T080–T118 | 39 |
| 6 - US4 Examples | US4 | T119–T137 | 19 |
| 7 - US5 Code Gen | US5 | T138–T142 | 5 |
| 8 - Polish | — | T143–T150 | 8 |
| **Total** | | | **150** |

### MVP Scope

**Minimum viable delivery**: Phase 1 (Setup) + Phase 3 (US1 Feature Flags) + Phase 4 (US2 Game Data, partial — start with Batch 1 simple modules)

This gives consumers: feature flags working, and the first wave of Game Data endpoint models with typed structs and tests.

### Independent Test Criteria per Story

- **US1**: `cargo check` / `cargo check --features wow` / `cargo check --features wow,user` all succeed with correct module visibility
- **US2**: `cargo test --features wow` — all Game Data fixture tests pass
- **US3**: `cargo test --features wow,user` — all Profile API fixture tests pass
- **US4**: `cargo build --examples --all-features` — all examples compile
- **US5**: `pygen` output compiles; `bendpoint` macro works with all UrlArgs variants
