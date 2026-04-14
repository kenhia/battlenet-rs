# Public API Contract: Full Character Download

**Feature**: 005-full-toon | **Date**: 2026-04-12

## Module: `battlenet_rs::wow_models::full_character`

### Types

```rust
/// Composite struct containing all downloaded character profile data.
pub struct FullCharacter {
    pub realm_slug: String,
    pub character_name: String,
    pub has_profile_data: bool,
    pub fetched_at: chrono::DateTime<chrono::Utc>,
    pub errors: Vec<EndpointError>,

    // Character profile endpoints (28 fields)
    pub profile: Option<CharacterProfile>,
    pub achievements: Option<CharacterAchievementsSummary>,
    pub achievement_statistics: Option<CharacterAchievementStatistics>,
    pub appearance: Option<CharacterAppearanceSummary>,
    pub collections: Option<CharacterCollectionsIndex>,
    pub mounts_collection: Option<CharacterMountsCollectionSummary>,
    pub pets_collection: Option<CharacterPetsCollectionSummary>,
    pub heirlooms_collection: Option<CharacterHeirloomsCollectionSummary>,
    pub toys_collection: Option<CharacterToysCollectionSummary>,
    pub encounters: Option<CharacterEncountersSummary>,
    pub dungeons: Option<CharacterDungeons>,
    pub raids: Option<CharacterRaids>,
    pub equipment: Option<CharacterEquipmentSummary>,
    pub hunter_pets: Option<CharacterHunterPetsSummary>,
    pub media: Option<CharacterMediaSummary>,
    pub mythic_keystone_profile: Option<CharacterMythicKeystoneProfileIndex>,
    pub mythic_keystone_season: Option<CharacterMythicKeystoneSeason>,
    pub professions: Option<CharacterProfessionsSummary>,
    pub pvp_summary: Option<CharacterPvpSummary>,
    pub pvp_2v2: Option<CharacterPvpBracketStatistics>,
    pub pvp_3v3: Option<CharacterPvpBracketStatistics>,
    pub pvp_rbg: Option<CharacterPvpBracketStatistics>,
    pub quests: Option<CharacterQuests>,
    pub completed_quests: Option<CharacterCompletedQuests>,
    pub reputations: Option<CharacterReputationsSummary>,
    pub soulbinds: Option<CharacterSoulbindsSummary>,
    pub specializations: Option<CharacterSpecializationsSummary>,
    pub statistics: Option<CharacterStatisticsSummary>,
    pub titles: Option<CharacterTitlesSummary>,
}

/// A failed endpoint call detail.
pub struct EndpointError {
    pub endpoint: String,
    pub message: String,
}
```

### Trait: `CharacterFetcher`

```rust
/// Abstraction enabling full_character to work with both
/// BattleNetClient (direct API) and CachedClient (cache-aware).
#[async_trait]
pub trait CharacterFetcher {
    async fn fetch_endpoint<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>
    where T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send;

    async fn fetch_endpoint_with_token<T>(&self, url_args: &UrlArgs, token: &str) -> Result<T, BattleNetClientError>
    where T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send;

    async fn fetch_endpoint_force<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>
    where T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send;

    async fn fetch_endpoint_with_token_force<T>(&self, url_args: &UrlArgs, token: &str) -> Result<T, BattleNetClientError>
    where T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send;
}
```

### Functions

```rust
/// Download all character profile data, returning a composite struct.
/// Uses client credentials; set `token` to Some(&str) for user-scoped data.
pub async fn full_character(
    fetcher: &impl CharacterFetcher,
    realm_slug: &str,
    name: &str,
    token: Option<&str>,
) -> Result<FullCharacter, BattleNetClientError>

/// Download all character profile data, bypassing cache for all endpoints.
pub async fn full_character_force(
    fetcher: &impl CharacterFetcher,
    realm_slug: &str,
    name: &str,
    token: Option<&str>,
) -> Result<FullCharacter, BattleNetClientError>

/// Download all character profile data and return as a JSON string.
pub async fn full_character_json(
    fetcher: &impl CharacterFetcher,
    realm_slug: &str,
    name: &str,
    token: Option<&str>,
) -> Result<String, BattleNetClientError>

/// Download all character profile data (force refresh) and return as JSON.
pub async fn full_character_json_force(
    fetcher: &impl CharacterFetcher,
    realm_slug: &str,
    name: &str,
    token: Option<&str>,
) -> Result<String, BattleNetClientError>
```

## Usage Examples

### Basic (no cache, no user token)

```rust
use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::full_character::*;

let client = BattleNetClient::new_from_environment();
let toon = full_character(&client, "trollbane", "belarsa", None).await?;
println!("Level: {}", toon.profile.unwrap().level);
```

### With user token

```rust
let toon = full_character(&client, "trollbane", "belarsa", Some(&token)).await?;
assert!(toon.has_profile_data);
```

### With cache

```rust
use battlenet_rs::cache::cached_client::CachedClient;
use battlenet_rs::cache::sqlite::SqliteCacheStore;

let store = SqliteCacheStore::new("sqlite:cache.db").await?;
let cached = CachedClient::new(client, store).await?;
let toon = full_character(&cached, "trollbane", "belarsa", None).await?;
```

### JSON output

```rust
let json = full_character_json(&client, "trollbane", "belarsa", None).await?;
println!("{json}");
```
