# Data Model: Full Character Download

**Feature**: 005-full-toon | **Date**: 2026-04-12

## Entities

### FullCharacter

Composite struct containing all downloaded character profile data.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| realm_slug | String | Yes | Realm identifier (e.g., "trollbane") |
| character_name | String | Yes | Character name (e.g., "belarsa") |
| has_profile_data | bool | Yes | `true` if user token was used for enhanced data |
| fetched_at | DateTime\<Utc\> | Yes | Timestamp when download was initiated |
| errors | Vec\<EndpointError\> | Yes | Errors from failed endpoint calls (empty if all succeeded) |
| profile | Option\<CharacterProfile\> | No | Required for success — base character data |
| achievements | Option\<CharacterAchievementsSummary\> | No | Achievement progress |
| achievement_statistics | Option\<CharacterAchievementStatistics\> | No | Achievement statistics |
| appearance | Option\<CharacterAppearanceSummary\> | No | Character appearance |
| collections | Option\<CharacterCollectionsIndex\> | No | Collections index |
| mounts_collection | Option\<CharacterMountsCollectionSummary\> | No | Mount collection |
| pets_collection | Option\<CharacterPetsCollectionSummary\> | No | Pet collection |
| heirlooms_collection | Option\<CharacterHeirloomsCollectionSummary\> | No | Heirloom collection |
| toys_collection | Option\<CharacterToysCollectionSummary\> | No | Toy collection |
| encounters | Option\<CharacterEncountersSummary\> | No | Encounters index |
| dungeons | Option\<CharacterDungeons\> | No | Dungeon encounters |
| raids | Option\<CharacterRaids\> | No | Raid encounters |
| equipment | Option\<CharacterEquipmentSummary\> | No | Equipped gear |
| hunter_pets | Option\<CharacterHunterPetsSummary\> | No | Hunter-only; None for non-hunters |
| media | Option\<CharacterMediaSummary\> | No | Character media/render |
| mythic_keystone_profile | Option\<CharacterMythicKeystoneProfileIndex\> | No | M+ profile |
| mythic_keystone_season | Option\<CharacterMythicKeystoneSeason\> | No | Current M+ season (auto-detected via seasons index) |
| professions | Option\<CharacterProfessionsSummary\> | No | Professions |
| pvp_summary | Option\<CharacterPvpSummary\> | No | PvP summary |
| pvp_2v2 | Option\<CharacterPvpBracketStatistics\> | No | 2v2 arena stats |
| pvp_3v3 | Option\<CharacterPvpBracketStatistics\> | No | 3v3 arena stats |
| pvp_rbg | Option\<CharacterPvpBracketStatistics\> | No | Rated BG stats |
| quests | Option\<CharacterQuests\> | No | Active quests |
| completed_quests | Option\<CharacterCompletedQuests\> | No | Completed quests |
| reputations | Option\<CharacterReputationsSummary\> | No | Faction reputations |
| soulbinds | Option\<CharacterSoulbindsSummary\> | No | Soulbinds (Shadowlands) |
| specializations | Option\<CharacterSpecializationsSummary\> | No | Spec/talent config |
| statistics | Option\<CharacterStatisticsSummary\> | No | Character stats (str, agi, etc.) |
| titles | Option\<CharacterTitlesSummary\> | No | Earned titles |

**Total**: 5 metadata fields + 28 endpoint fields = 33 fields.

### EndpointError

Lightweight record of a failed endpoint call.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| endpoint | String | Yes | Human-readable endpoint name (e.g., "hunter_pets") |
| message | String | Yes | Error message or API detail string |

### CharacterFetcher (Trait)

Abstraction over BattleNetClient and CachedClient for generic download logic.

| Method | Signature | Description |
|--------|-----------|-------------|
| fetch_endpoint | `async fn fetch_endpoint<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>` | Fetch with client credentials |
| fetch_endpoint_with_token | `async fn fetch_endpoint_with_token<T>(&self, url_args: &UrlArgs, token: &str) -> Result<T, BattleNetClientError>` | Fetch with user token |
| fetch_endpoint_force | `async fn fetch_endpoint_force<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>` | Force-refresh (bypass cache) |
| fetch_endpoint_with_token_force | `async fn fetch_endpoint_with_token_force<T>(&self, url_args: &UrlArgs, token: &str) -> Result<T, BattleNetClientError>` | Force-refresh with user token |

**Trait bounds on T**: `for<'a> Deserialize<'a> + Serialize + GenerateUrl`

## Relationships

```
FullCharacter
 ├── CharacterProfile (required — download fails if this fails)
 ├── 27 x Option<endpoint struct> (graceful degradation)
 └── Vec<EndpointError> (captures failures)

CharacterFetcher trait
 ├── impl for BattleNetClient (direct API calls)
 └── impl for CachedClient<S> (cache-aware calls)
```

## State Transitions

The download has no persistent state. Each call is independent:

1. **Initiate** → Record `fetched_at` timestamp
2. **Fetch base profile** → If error, return Err immediately (FR-008)
3. **Fetch current M+ season ID** → Call `MythicKeystoneSeasonsIndex` for `current_season.id` (failure is non-fatal → skip M+ season field)
4. **Fetch remaining endpoints** → Each success → `Some(data)`; each failure → `None` + `EndpointError`
5. **Assemble** → Build `FullCharacter` struct
6. **Return** → Ok(FullCharacter) or Err if base profile failed

## Validation Rules

- `realm_slug` must be non-empty lowercase slug (validated by URL construction)
- `character_name` must be non-empty (validated by URL construction)
- If base profile (`CharacterProfile`) fails, the entire download fails (FR-008)
- PvP bracket names must be one of: "2v2", "3v3", "rbg"
