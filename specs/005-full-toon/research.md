# Research: Full Character Download

**Feature**: 005-full-toon | **Date**: 2026-04-12

## R1: Character Profile Endpoint Inventory

**Task**: Catalog all character profile endpoints available for composite download.

**Findings**: 28 structs across 15 modules, all in `profile` namespace:

| # | Struct | Module | UrlArgs | Notes |
|---|--------|--------|---------|-------|
| 1 | CharacterProfileStatus | character_profile | Player | Validation only (is_valid, id) |
| 2 | CharacterProfile | character_profile | Player | Base profile — manual GenerateUrl impl |
| 3 | CharacterAchievementsSummary | character_achievements | Player | |
| 4 | CharacterAchievementStatistics | character_achievements | Player | |
| 5 | CharacterAppearanceSummary | character_appearance | Player | |
| 6 | CharacterCollectionsIndex | character_collections | Player | |
| 7 | CharacterMountsCollectionSummary | character_collections | Player | |
| 8 | CharacterPetsCollectionSummary | character_collections | Player | |
| 9 | CharacterHeirloomsCollectionSummary | character_collections | Player | |
| 10 | CharacterToysCollectionSummary | character_collections | Player | |
| 11 | CharacterEncountersSummary | character_encounters | Player | |
| 12 | CharacterDungeons | character_encounters | Player | |
| 13 | CharacterRaids | character_encounters | Player | |
| 14 | CharacterEquipmentSummary | character_equipment | Player | |
| 15 | CharacterHunterPetsSummary | character_hunter_pets | Player | Class-specific (hunters only) |
| 16 | CharacterMediaSummary | character_media | Player | |
| 17 | CharacterMythicKeystoneProfileIndex | character_mythic_keystone | Player | |
| 18 | CharacterMythicKeystoneSeason | character_mythic_keystone | PlayerExtra | Requires season ID |
| 19 | CharacterProfessionsSummary | character_professions | Player | |
| 20 | CharacterPvpSummary | character_pvp | Player | |
| 21 | CharacterPvpBracketStatistics | character_pvp | PlayerExtra | Brackets: 2v2, 3v3, rbg |
| 22 | CharacterQuests | character_quests | Player | |
| 23 | CharacterCompletedQuests | character_quests | Player | |
| 24 | CharacterReputationsSummary | character_reputations | Player | |
| 25 | CharacterSoulbindsSummary | character_soulbinds | Player | Shadowlands-specific |
| 26 | CharacterSpecializationsSummary | character_specializations | Player | |
| 27 | CharacterStatisticsSummary | character_statistics | Player | |
| 28 | CharacterTitlesSummary | character_titles | Player | |

**Decision**: Include 26 Player endpoints in the FullCharacter struct. Exclude
only `CharacterProfileStatus` (it's a validation tool, not character data).

`CharacterMythicKeystoneSeason` (PlayerExtra, requires a season ID) is now
included: the orchestration first fetches `MythicKeystoneSeasonsIndex`
(`data/wow/mythic-keystone/season/index`, dynamic namespace) which provides a
`current_season.id` field, then uses that ID as the `extra` in
`UrlArgs::PlayerExtra` to fetch the character's current M+ season data.

**Note**: The existing `MythicKeystoneSeasonsIndex` struct is missing the
`current_season` field — it only has `seasons: Vec<KeyAndId>`. A new
`current_season: KeyAndId` field must be added to that struct. This is a
backward-compatible change (the field already exists in the API response).

PvP bracket statistics (PlayerExtra) are included for the 3 standard brackets
(2v2, 3v3, rbg).

**Total fields in FullCharacter**: 28 endpoint fields (24 Player + 1 M+ season
+ 3 PvP brackets).

## R2: Public vs Token-Required Endpoint Access

**Task**: Determine which endpoints work with client credentials vs requiring a user OAuth token.

**Findings**: The Blizzard Profile API supports two access patterns:

1. **Client credentials** (`get_data()`): All character profile endpoints at
   `profile/wow/character/{realm_slug}/{name}/*` are accessible with client
   credentials (application token). This returns the full public character data.

2. **User OAuth token** (`get_data_with_token()`): Same endpoints can be called
   with a user token. Some endpoints (like protected character data) only return
   full data with a user token. The Account Profile endpoints
   (`profile/user/wow/*`) always require a user token.

The `char-profile.rs` example confirms this — it uses `client.get_data()` (no
user token) to access `CharacterProfile`, `CharacterProfileStatus`, and other
profile endpoints successfully.

**Decision**: The full download uses `get_data()` (client credentials) for all
character profile endpoints by default. When a user token is provided, it uses
`get_data_with_token()` which may return additional data. The `has_profile_data`
flag indicates whether the richer user-token path was used. This matches the
spec's FR-006 and FR-007.

**Rationale**: This is simpler and more flexible — the download works for any
character, not just the authenticated user's characters.

## R3: Composite Struct Design Pattern

**Task**: Determine the best pattern for the FullCharacter composite struct.

**Alternatives considered**:

1. **Flat struct with Option<T> fields**: Each endpoint gets a named field.
   Simple, discoverable, type-safe. Downside: struct definition is large (~30 fields).

2. **HashMap<String, serde_json::Value>**: Dynamic, but loses type safety.
   Consumers must know field names and deserialize manually.

3. **Nested module structs**: Group by category (collections, encounters, pvp).
   More organized but adds indirection.

**Decision**: Flat struct with `Option<T>` fields (Alternative 1).

**Rationale**: Type safety is the primary value proposition of this library.
Consumers get auto-complete and compile-time guarantees. The struct is large
but each field is self-documenting. This matches the user's mental model from
the sprint prep notes.

## R4: Error Capture Strategy

**Task**: Design how per-endpoint errors are captured in the composite struct.

**Findings**: When an endpoint returns an error (e.g., 404 for hunter pets on a
non-hunter), the Blizzard API returns a JSON error body:

```json
{
  "code": 404,
  "type": "BLZWEBAPI00000404",
  "detail": "Not Found"
}
```

**Alternatives considered**:

1. **Per-field Result<T, E>**: Each field is `Result<T, String>` instead of
   `Option<T>`. Provides the error inline but makes the struct harder to use
   (every field access needs unwrapping).

2. **Separate errors Vec**: Keep fields as `Option<T>` and collect errors in a
   `Vec<EndpointError>`. Cleaner for the happy path — consumers check errors
   only if they care.

3. **Per-field Option + separate errors map**: Fields are `Option<T>`, plus a
   `HashMap<String, String>` of endpoint→error mappings. Allows lookup by
   endpoint name.

**Decision**: Alternative 2 — `Option<T>` fields with a separate
`Vec<EndpointError>` where `EndpointError` captures the endpoint name and error
message string.

**Rationale**: Most consumers want to iterate over the populated fields, not
handle errors for each one. The errors vec is available for diagnostics but
doesn't clutter the primary API. When an endpoint fails, its field is `None`
and an entry is added to `errors`.

## R5: Cache Integration Architecture

**Task**: Design how the full download integrates with `CachedClient` from sprint 004.

**Findings**: The `CachedClient<S: CacheStore>` wraps `BattleNetClient` and
provides matching method signatures (`get_data`, `get_data_with_token`,
`get_data_force`, `get_data_with_token_force`). Both `BattleNetClient` and
`CachedClient` have compatible method names.

**Alternatives considered**:

1. **Trait abstraction**: Define a `CharacterFetcher` trait that both
   `BattleNetClient` and `CachedClient` implement. The full download function
   accepts any `impl CharacterFetcher`. Clean abstraction but adds a new trait.

2. **Generic with method delegation**: The full download function is generic
   over a parameter that provides `get_data` / `get_data_with_token`. This
   could be a closure-based approach.

3. **Separate functions per client type**: `full_character()` on
   `BattleNetClient` and `full_character()` on `CachedClient<S>`. Duplicates
   the orchestration logic but avoids abstraction overhead.

4. **Functions taking BattleNetClient directly**: Implement on `BattleNetClient`
   only. For cached usage, consumers access `cached.client` and the caching
   happens per-endpoint through the existing CachedClient methods. But this
   loses per-endpoint cache benefits.

**Decision**: Alternative 1 — Define a `CharacterFetcher` trait with
`fetch_endpoint()` and `fetch_endpoint_with_token()` methods. Implement it for
both `BattleNetClient` and `CachedClient<S>`. The `full_character()` function
is generic over `impl CharacterFetcher`.

**Rationale**: This is the cleanest approach — the orchestration logic is
written once, cache vs non-cache is transparent at the call site, and force
refresh is handled by providing a force-refresh variant of the trait
implementation. The trait is small (2-3 methods) and feature-gated behind `wow`.

## R6: Concurrency Strategy

**Task**: Determine whether to fetch endpoints sequentially or concurrently.

**Findings**: With 27 endpoint calls per character at ~200-300ms each (from
the `account-profile-cached` example), a sequential download takes ~6-8 seconds.
Concurrent fetching with `tokio::join!` or `futures::join_all` could reduce
this to ~300ms (one round-trip with connection reuse). However, the rate limiter
caps burst at 100/s, so 27 concurrent requests are well within limits.

**Decision**: Sequential fetching for v1.

**Rationale**: Simplicity. Sequential is easier to debug, produces deterministic
error ordering, and works naturally with the rate limiter. 27 requests at
100/s takes <1 second of rate-limited time. The larger bottleneck is network
latency, but for a library that's called programmatically (not interactively),
6-8 seconds is acceptable. A future sprint can add concurrent fetching if
performance becomes a concern.

**Alternatives rejected**: `FuturesUnordered` batch — adds complexity around
error handling, makes per-endpoint error tracking harder to reason about, and
would require careful testing for race conditions in error capture.
