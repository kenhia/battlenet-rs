# Feature Specification: Library Setup, WoW Retail API Coverage, and Examples

**Feature Branch**: `003-lib-wow-examples`  
**Created**: 2026-04-09  
**Status**: Draft  
**Spec Path**: `specs/003-lib-wow-examples/spec.md` *(SDD: required before implementation)*  
**Input**: User description: "Library Setup, WoW Retail API Coverage, and Examples — covering Cargo feature flags, full WoW Retail endpoint coverage, and working examples."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Selective Library Compilation via Feature Flags (Priority: P1)

A Rust developer adds `battlenet-rs` to their project and wants to only compile the parts of the library they need. They add the crate with `features = ["wow"]` to get World of Warcraft APIs, or `features = ["user"]` to get user-authenticated APIs. Developers who only need core utilities (auth, client, region) use the `default` feature with zero game-specific code compiled in.

**Why this priority**: Feature flags are foundational. Without them, all subsequent API modules and examples cannot be properly organized or gated. This is the structural backbone of the library.

**Independent Test**: Can be fully tested by adding the crate with various feature combinations and verifying only the expected modules compile. Delivers immediate value by reducing compile times and binary size for consumers who only need a subset of APIs.

**Acceptance Scenarios**:

1. **Given** a Rust project depending on `battlenet-rs` with no feature flags, **When** the project compiles, **Then** only core modules (auth, client, errors, namespace, region) are included; no game-specific modules are compiled.
2. **Given** a Rust project depending on `battlenet-rs` with `features = ["wow"]`, **When** the project compiles, **Then** the WoW retail modules are available in addition to core modules.
3. **Given** a Rust project depending on `battlenet-rs` with `features = ["user"]`, **When** the project compiles, **Then** user-authenticated API modules (e.g., Account Profile) are available.
4. **Given** a Rust project depending on `battlenet-rs` with `features = ["wow", "user"]`, **When** the project compiles, **Then** both WoW retail and user-authenticated API modules are available.
5. **Given** the existing `redis` feature, **When** any combination of new features is enabled alongside `redis`, **Then** the `redis` feature continues to work as before with no regressions.
6. **Given** stub features (`wow-classic`, `diablo`, `hearthstone`, `starcraft`) are declared, **When** a consumer enables one of them, **Then** the crate compiles successfully (no code is gated behind them yet, but the features exist for forward compatibility).

---

### User Story 2 - Querying WoW Game Data APIs (Priority: P1)

A Rust developer wants to query any World of Warcraft retail Game Data API endpoint (e.g., items, mounts, spells, professions, PvP seasons, realms, etc.) using a client-credentials token. They call the corresponding typed method on `BattleNetClient`, which returns a strongly-typed Rust struct deserialized from the JSON response.

**Why this priority**: This is the core value proposition of the library — wrapping the ~150 WoW retail API endpoints with typed Rust models. The 5 existing Achievement endpoints, 2 Connected Realm endpoints, and 1 WoW Token endpoint demonstrate the pattern; this story scales it to all remaining Game Data categories.

**Independent Test**: Can be tested by calling any newly-implemented endpoint method and verifying the response deserializes correctly. Each endpoint category is independently valuable.

**Acceptance Scenarios**:

1. **Given** a `BattleNetClient` with valid credentials, **When** the developer calls any Game Data API endpoint method (e.g., mounts index, item classes index), **Then** the response is deserialized into the correct typed struct.
2. **Given** a `BattleNetClient` with valid credentials, **When** the developer calls a Game Data API endpoint's JSON variant, **Then** the raw JSON string is returned.
3. **Given** an endpoint that requires an ID parameter (e.g., a specific mount, spell, or item), **When** the developer passes the appropriate URL arguments, **Then** the correct URL is constructed and the expected data is returned.
4. **Given** an endpoint that uses a Search path (e.g., item search, creature search), **When** the developer calls the search method, **Then** search results are returned as a typed struct.
5. **Given** all Game Data API categories documented in `ModelImplementProgress.md`, **When** checking implementation coverage, **Then** every listed endpoint has a corresponding model struct, URL generation, and type aliases for result/JSON result.

---

### User Story 3 - Querying WoW Profile APIs with a User Token (Priority: P2)

A Rust developer wants to query World of Warcraft Profile APIs (Account Profile, Character Achievements, Character Equipment, Guild, etc.) that require a user OAuth token. The developer provides the token (obtaining it is their responsibility), and the library handles constructing the correct request with the appropriate namespace and token placement.

**Why this priority**: Profile APIs unlock per-player and per-character data, which is a major use case for the library. However, they depend on the caller having a user OAuth token, making them secondary to client-credentials Game Data APIs.

**Independent Test**: Can be tested by providing a valid user token and calling any Profile API endpoint. Each character/account endpoint category is independently valuable.

**Acceptance Scenarios**:

1. **Given** a `BattleNetClient` and a valid user OAuth token, **When** the developer calls an Account Profile endpoint (e.g., Account Profile Summary), **Then** the response is deserialized into the correct typed struct.
2. **Given** a `BattleNetClient` and a valid user OAuth token, **When** the developer calls a Character Profile endpoint with realm slug and character name, **Then** the correct URL is formed (including the profile namespace) and the response is returned as a typed struct.
3. **Given** a `BattleNetClient` and a valid user OAuth token, **When** the developer calls a Guild endpoint, **Then** guild data is returned as a typed struct.
4. **Given** all Profile API categories documented in `ModelImplementProgress.md`, **When** checking implementation coverage, **Then** every listed Profile endpoint has a corresponding model struct, URL generation, and type aliases.

---

### User Story 4 - Learning from Working Examples (Priority: P2)

A Rust developer new to the library wants to understand how to call various API endpoints. They look in the `examples/` directory and find runnable examples organized by endpoint group. Each example demonstrates initializing the client, calling one or more endpoints, and handling the response.

**Why this priority**: Examples are a developer's first touchpoint with any library. Good examples reduce onboarding time and support questions. They depend on the endpoints being implemented (Stories 2 & 3).

**Independent Test**: Each example can be compiled and, given valid credentials, run successfully against the live API.

**Acceptance Scenarios**:

1. **Given** the `examples/` directory, **When** a developer lists the examples, **Then** there is at least one example per major endpoint group (e.g., Account Profile, Character Achievements, Mounts, Items, PvP, etc.).
2. **Given** any example file, **When** the developer compiles the example with appropriate features enabled, **Then** the example compiles without errors.
3. **Given** a working example and valid Battle.net API credentials in environment variables, **When** the developer runs the example, **Then** it produces meaningful output showing the API response.
4. **Given** examples involving user-authenticated endpoints, **When** the developer reads the example, **Then** it clearly documents that a user OAuth token is required and shows how to provide one.

---

### User Story 5 - Code Generation Compatibility (Priority: P3)

A contributor uses the existing `pygen` YAML-to-Rust code generator or the `bendpoint` proc macro to add new endpoint models. The generated code is compatible with the new feature-flag structure and follows the established patterns for model structs, URL generation, and type aliases.

**Why this priority**: Tooling compatibility ensures the path to adding future endpoints remains efficient. This is lower priority because it is a contributor-facing concern, not a consumer-facing one.

**Independent Test**: Can be tested by generating a model from a YAML definition and verifying the output compiles under the correct feature flag.

**Acceptance Scenarios**:

1. **Given** an endpoint definition in YAML format, **When** the contributor runs `pygen`, **Then** the generated Rust code compiles cleanly under the appropriate feature flag.
2. **Given** the `bendpoint` proc macro, **When** it is used to annotate a new endpoint model, **Then** the generated URL implementation is correct and works with the existing client methods.

---

### Edge Cases

- What happens when a developer enables `wow` but not `user`, and tries to use an Account Profile type? The code should not compile — Profile API types requiring a user token should be gated behind the `user` feature (or a combined `wow` + `user` gate).
- What happens when the Blizzard API returns an unexpected JSON structure for an endpoint? The strongly-typed deserialization returns an error through the existing error type.
- What happens when a Search endpoint returns zero results? The model handles empty result sets gracefully (e.g., an empty list).
- What happens when the `auction_house.rs` module (currently orphaned and not compiled) is brought into the feature-gated module tree? Existing code that does not depend on it remains unaffected.
- What happens when `character_profile.rs` (currently always compiled) is moved behind `#[cfg(feature = "user")]`? This is a breaking change for any consumer who uses character profile types without enabling the `user` feature. Acceptable because the library is unpublished; consumers must add `features = ["wow", "user"]`.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The library MUST define Cargo feature flags: `default` (core only), `wow` (WoW retail Game Data APIs), `user` (user-authenticated APIs), and stub features `wow-classic`, `diablo`, `hearthstone`, `starcraft`.
- **FR-002**: The `default` feature MUST include only core modules: auth, client, errors, namespace, region. No game-specific code compiles by default.
- **FR-003**: The `wow` feature MUST gate all World of Warcraft retail Game Data API models and implementations behind conditional compilation.
- **FR-004**: The `user` feature MUST gate all user-authenticated API models (Account Profile, Character Profile endpoints requiring OAuth) behind conditional compilation.
- **FR-005**: The existing `redis` feature MUST continue to function as-is alongside any combination of new features.
- **FR-006**: The library MUST provide typed structs for every WoW retail Game Data API endpoint listed in `ModelImplementProgress.md` (~140 endpoints across 30+ categories including: Achievement, Auction House, Azerite Essence, Connected Realms, Covenant, Creature, Guild Crest, Heirloom, Item, Journal, Media Search, Modified Crafting, Mount, Mythic Keystone Affix, Mythic Keystone Dungeon, Mythic Keystone Leaderboard, Mythic Raid Leaderboard, Pet, Playable Class, Playable Race, Playable Specialization, Power Type, Profession, PvP Season, PvP Tier, Quests, Realm, Region, Reputations, Spell, Talent, Tech Talent, Title, Toy, WoW Token).
- **FR-007**: Each endpoint model MUST implement URL generation to produce the correct API URL given a client and appropriate arguments.
- **FR-008**: Each endpoint model MUST have typed result aliases and JSON result aliases following the established pattern.
- **FR-009**: The library MUST provide typed structs for every WoW retail Profile API endpoint listed in `ModelImplementProgress.md` (Account Profile, Character Achievements, Character Appearance, Character Collections, Character Encounters, Character Equipment, Character Hunter Pets, Character Media, Character Mythic Keystone Profile, Character Professions, Character Profile, Character PvP, Character Quests, Character Reputations, Character Soulbinds, Character Specializations, Character Statistics, Character Titles, Guild).
- **FR-010**: Profile API endpoints MUST correctly use the profile namespace variant when constructing URLs.
- **FR-011**: The `examples/` directory MUST contain at least one runnable example per major endpoint group.
- **FR-012**: Each example MUST compile when the appropriate features are enabled.
- **FR-013**: The existing orphaned `auction_house.rs` module MUST be integrated into the WoW module tree under the `wow` feature gate.
- **FR-014**: All code MUST pass linting with zero warnings and formatting checks with zero violations.
- **FR-015**: All new endpoint models MUST have corresponding unit tests.
- **FR-016**: The library MUST provide methods on `BattleNetClient` to send requests using a caller-provided user OAuth token (`send_request_with_token`, `get_data_with_token`, `get_json_with_token`).

### Key Entities

- **BattleNetClient**: The central HTTP client managing credentials, tokens, region, and locale. Provides typed and raw API call methods.
- **Endpoint Model Struct**: A deserializable struct matching the JSON shape of a specific API endpoint's response. Implements URL generation.
- **URL Arguments**: An enum representing the different types of URL parameters endpoints require (none, single ID, player realm+name, and potentially new variants for guild, search, nested IDs, etc.).
- **Feature Flag**: A Cargo feature controlling conditional compilation of entire module subtrees (core, wow, user, redis).
- **Namespace**: Determines the API namespace qualifier (static, dynamic, profile) appended with the region string.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All WoW retail API endpoints documented in `ModelImplementProgress.md` have corresponding typed model structs and implementations.
- **SC-002**: A consumer using only the `wow` feature compiles the library with WoW Game Data APIs and no user-token code.
- **SC-003**: At least 15 working examples exist in `examples/`, covering the major endpoint groups.
- **SC-004**: A new contributor can add a new endpoint model and have it compile under the correct feature flag using existing patterns or code generation tools.
- **SC-005**: The full test suite passes with zero failures across all new and existing tests.
- **SC-006**: Linting and formatting checks pass with zero issues.
- **SC-007**: The existing `redis` feature and user token module continue to work identically to their pre-sprint behavior.

## Assumptions

- The Blizzard Battle.net API documentation and endpoint inventory in `ModelImplementProgress.md` is accurate as of the 2026-04-08 audit date.
- The existing client pattern of typed/raw API calls with URL generation trait and URL arguments enum is the correct architecture to scale to all endpoints.
- The URL arguments enum may need additional variants (e.g., for guild realm+name, for search query parameters, for nested IDs like profession/skill-tier) but the enum-based approach is retained.
- The `bendpoint` proc macro and `pygen` code generator are available and functional for reducing boilerplate.
- Token management for user-authenticated (OAuth) endpoints is the caller's responsibility; the library only requires the token to be passed in.
- Database storage layer is explicitly out of scope for this sprint.
- The `wow-classic`, `diablo`, `hearthstone`, and `starcraft` features are stubs only — no code is gated behind them in this sprint.
- Examples will use environment variables (via `dotenvy`) for API credentials, following the existing pattern in the current examples.

## Polish Phase Checklist *(SDD/TDD — mandatory)*

The following MUST be completed before the feature branch is merged:

- [ ] `docs/specification.md` updated with changes from this spec
- [ ] `docs/architecture.md` updated to reflect feature-flag module structure
- [ ] `docs/installation.md` updated with feature flag usage instructions
- [ ] `docs/usage.md` updated with new endpoint examples and feature flag documentation
- [ ] All tests written first (TDD) and passing
- [ ] Pre-commit suite passes clean (CI variant)
