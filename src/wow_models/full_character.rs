use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::character_achievements::*;
use crate::wow_models::character_appearance::*;
use crate::wow_models::character_collections::*;
use crate::wow_models::character_encounters::*;
use crate::wow_models::character_equipment::*;
use crate::wow_models::character_hunter_pets::*;
use crate::wow_models::character_media::*;
use crate::wow_models::character_mythic_keystone::*;
use crate::wow_models::character_professions::*;
use crate::wow_models::character_profile::*;
use crate::wow_models::character_pvp::*;
use crate::wow_models::character_quests::*;
use crate::wow_models::character_reputations::*;
use crate::wow_models::character_soulbinds::*;
use crate::wow_models::character_specializations::*;
use crate::wow_models::character_statistics::*;
use crate::wow_models::character_titles::*;
use crate::wow_models::mythic_keystone::MythicKeystoneSeasonsIndex;
use crate::wow_models::{GenerateUrl, UrlArgs};

// =============================================================================
// T006: EndpointError
// =============================================================================

/// A failed endpoint call detail.
#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointError {
    pub endpoint: String,
    pub message: String,
}

// =============================================================================
// T007: FullCharacter
// =============================================================================

/// Composite struct containing all downloaded character profile data.
#[derive(Debug, Serialize, Deserialize)]
pub struct FullCharacter {
    pub realm_slug: String,
    pub character_name: String,
    pub has_profile_data: bool,
    pub fetched_at: DateTime<Utc>,
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

// =============================================================================
// T008: CharacterFetcher trait
// =============================================================================

/// Abstraction enabling full_character to work with both
/// BattleNetClient (direct API) and CachedClient (cache-aware).
#[async_trait]
pub trait CharacterFetcher: Send + Sync {
    async fn fetch_endpoint<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send;

    async fn fetch_endpoint_with_token<T>(
        &self,
        url_args: &UrlArgs,
        token: &str,
    ) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send;

    async fn fetch_endpoint_force<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send;

    async fn fetch_endpoint_with_token_force<T>(
        &self,
        url_args: &UrlArgs,
        token: &str,
    ) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send;
}

// =============================================================================
// T009: CharacterFetcher impl for BattleNetClient
// =============================================================================

#[async_trait]
impl CharacterFetcher for BattleNetClient {
    async fn fetch_endpoint<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send,
    {
        self.get_data::<T>(url_args).await
    }

    async fn fetch_endpoint_with_token<T>(
        &self,
        url_args: &UrlArgs,
        token: &str,
    ) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send,
    {
        self.get_data_with_token::<T>(url_args, token).await
    }

    // BattleNetClient has no cache, so force variants are identical
    async fn fetch_endpoint_force<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send,
    {
        self.get_data::<T>(url_args).await
    }

    async fn fetch_endpoint_with_token_force<T>(
        &self,
        url_args: &UrlArgs,
        token: &str,
    ) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send,
    {
        self.get_data_with_token::<T>(url_args, token).await
    }
}

// =============================================================================
// T025: CharacterFetcher impl for CachedClient
// =============================================================================

#[cfg(any(feature = "db-sqlite", feature = "db-postgres"))]
use crate::cache::{cached_client::CachedClient, CacheStore};

#[cfg(any(feature = "db-sqlite", feature = "db-postgres"))]
#[async_trait]
impl<S: CacheStore> CharacterFetcher for CachedClient<S> {
    async fn fetch_endpoint<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send,
    {
        self.get_data::<T>(url_args).await
    }

    async fn fetch_endpoint_with_token<T>(
        &self,
        url_args: &UrlArgs,
        token: &str,
    ) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send,
    {
        self.get_data_with_token::<T>(url_args, token).await
    }

    async fn fetch_endpoint_force<T>(&self, url_args: &UrlArgs) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send,
    {
        self.get_data_force::<T>(url_args).await
    }

    async fn fetch_endpoint_with_token_force<T>(
        &self,
        url_args: &UrlArgs,
        token: &str,
    ) -> Result<T, BattleNetClientError>
    where
        T: for<'a> Deserialize<'a> + Serialize + GenerateUrl + Send,
    {
        self.get_data_with_token_force::<T>(url_args, token).await
    }
}

// =============================================================================
// T015: try_fetch helper
// =============================================================================

/// Try fetching a single endpoint. On success returns Some(data), on failure
/// returns None and appends an EndpointError to the errors vec.
async fn try_fetch<T, F, Fut>(
    endpoint_name: &str,
    errors: &mut Vec<EndpointError>,
    fetch_fn: F,
) -> Option<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, BattleNetClientError>>,
{
    match fetch_fn().await {
        Ok(data) => Some(data),
        Err(e) => {
            errors.push(EndpointError {
                endpoint: endpoint_name.to_string(),
                message: e.to_string(),
            });
            None
        }
    }
}

// =============================================================================
// T016: M+ current season lookup
// =============================================================================

/// Fetch the current M+ season ID from the seasons index.
/// Returns None if the lookup fails (non-fatal).
async fn get_current_season_id(fetcher: &(impl CharacterFetcher + ?Sized)) -> Option<u64> {
    let result = fetcher
        .fetch_endpoint::<MythicKeystoneSeasonsIndex>(&UrlArgs::None)
        .await;
    match result {
        Ok(index) => Some(index.current_season.id),
        Err(_) => None,
    }
}

// =============================================================================
// T017: full_character() orchestration
// =============================================================================

/// Download all character profile data, returning a composite struct.
/// Uses client credentials; set `token` to Some(&str) for user-scoped data.
pub async fn full_character(
    fetcher: &(impl CharacterFetcher + ?Sized),
    realm_slug: &str,
    name: &str,
    token: Option<&str>,
) -> Result<FullCharacter, BattleNetClientError> {
    fetch_all(fetcher, realm_slug, name, token, false).await
}

// =============================================================================
// T018: full_character_force()
// =============================================================================

/// Download all character profile data, bypassing cache for all endpoints.
pub async fn full_character_force(
    fetcher: &(impl CharacterFetcher + ?Sized),
    realm_slug: &str,
    name: &str,
    token: Option<&str>,
) -> Result<FullCharacter, BattleNetClientError> {
    fetch_all(fetcher, realm_slug, name, token, true).await
}

// =============================================================================
// T021: full_character_json()
// =============================================================================

/// Download all character profile data and return as a JSON string.
pub async fn full_character_json(
    fetcher: &(impl CharacterFetcher + ?Sized),
    realm_slug: &str,
    name: &str,
    token: Option<&str>,
) -> Result<String, BattleNetClientError> {
    let fc = full_character(fetcher, realm_slug, name, token).await?;
    let json = serde_json::to_string(&fc)?;
    Ok(json)
}

// =============================================================================
// T022: full_character_json_force()
// =============================================================================

/// Download all character profile data (bypassing cache) and return as a JSON string.
pub async fn full_character_json_force(
    fetcher: &(impl CharacterFetcher + ?Sized),
    realm_slug: &str,
    name: &str,
    token: Option<&str>,
) -> Result<String, BattleNetClientError> {
    let fc = full_character_force(fetcher, realm_slug, name, token).await?;
    let json = serde_json::to_string(&fc)?;
    Ok(json)
}

/// Internal orchestration — sequential fetch of all endpoints.
async fn fetch_all(
    fetcher: &(impl CharacterFetcher + ?Sized),
    realm_slug: &str,
    name: &str,
    token: Option<&str>,
    force: bool,
) -> Result<FullCharacter, BattleNetClientError> {
    let fetched_at = Utc::now();
    let has_profile_data = token.is_some();
    let mut errors = Vec::new();

    let player = UrlArgs::Player {
        realm_slug: realm_slug.to_string(),
        name: name.to_string(),
    };

    // --- Base profile (fail-fast per FR-008) ---
    let profile: CharacterProfile = if force {
        match token {
            Some(t) => fetcher.fetch_endpoint_with_token_force(&player, t).await?,
            None => fetcher.fetch_endpoint_force(&player).await?,
        }
    } else {
        match token {
            Some(t) => fetcher.fetch_endpoint_with_token(&player, t).await?,
            None => fetcher.fetch_endpoint(&player).await?,
        }
    };

    // --- M+ current season ID (non-fatal) ---
    let current_season_id = get_current_season_id(fetcher).await;

    // --- Macro to reduce boilerplate for Player endpoints ---
    macro_rules! fetch_player {
        ($field_name:expr, $type:ty) => {
            try_fetch($field_name, &mut errors, || async {
                if force {
                    match token {
                        Some(t) => {
                            fetcher
                                .fetch_endpoint_with_token_force::<$type>(&player, t)
                                .await
                        }
                        None => fetcher.fetch_endpoint_force::<$type>(&player).await,
                    }
                } else {
                    match token {
                        Some(t) => fetcher.fetch_endpoint_with_token::<$type>(&player, t).await,
                        None => fetcher.fetch_endpoint::<$type>(&player).await,
                    }
                }
            })
            .await
        };
    }

    macro_rules! fetch_player_extra {
        ($field_name:expr, $type:ty, $extra:expr) => {
            try_fetch($field_name, &mut errors, || async {
                let args = UrlArgs::PlayerExtra {
                    realm_slug: realm_slug.to_string(),
                    name: name.to_string(),
                    extra: $extra.to_string(),
                };
                if force {
                    match token {
                        Some(t) => {
                            fetcher
                                .fetch_endpoint_with_token_force::<$type>(&args, t)
                                .await
                        }
                        None => fetcher.fetch_endpoint_force::<$type>(&args).await,
                    }
                } else {
                    match token {
                        Some(t) => fetcher.fetch_endpoint_with_token::<$type>(&args, t).await,
                        None => fetcher.fetch_endpoint::<$type>(&args).await,
                    }
                }
            })
            .await
        };
    }

    // --- Fetch all remaining endpoints sequentially ---
    let achievements = fetch_player!("achievements", CharacterAchievementsSummary);
    let achievement_statistics =
        fetch_player!("achievement_statistics", CharacterAchievementStatistics);
    let appearance = fetch_player!("appearance", CharacterAppearanceSummary);
    let collections = fetch_player!("collections", CharacterCollectionsIndex);
    let mounts_collection = fetch_player!("mounts_collection", CharacterMountsCollectionSummary);
    let pets_collection = fetch_player!("pets_collection", CharacterPetsCollectionSummary);
    let heirlooms_collection =
        fetch_player!("heirlooms_collection", CharacterHeirloomsCollectionSummary);
    let toys_collection = fetch_player!("toys_collection", CharacterToysCollectionSummary);
    let encounters = fetch_player!("encounters", CharacterEncountersSummary);
    let dungeons = fetch_player!("dungeons", CharacterDungeons);
    let raids = fetch_player!("raids", CharacterRaids);
    let equipment = fetch_player!("equipment", CharacterEquipmentSummary);
    let hunter_pets = fetch_player!("hunter_pets", CharacterHunterPetsSummary);
    let media = fetch_player!("media", CharacterMediaSummary);
    let mythic_keystone_profile = fetch_player!(
        "mythic_keystone_profile",
        CharacterMythicKeystoneProfileIndex
    );

    // M+ current season (PlayerExtra — requires season ID)
    let mythic_keystone_season = if let Some(season_id) = current_season_id {
        fetch_player_extra!(
            "mythic_keystone_season",
            CharacterMythicKeystoneSeason,
            season_id.to_string()
        )
    } else {
        None
    };

    let professions = fetch_player!("professions", CharacterProfessionsSummary);
    let pvp_summary = fetch_player!("pvp_summary", CharacterPvpSummary);

    // PvP brackets (PlayerExtra)
    let pvp_2v2 = fetch_player_extra!("pvp_2v2", CharacterPvpBracketStatistics, "2v2");
    let pvp_3v3 = fetch_player_extra!("pvp_3v3", CharacterPvpBracketStatistics, "3v3");
    let pvp_rbg = fetch_player_extra!("pvp_rbg", CharacterPvpBracketStatistics, "rbg");

    let quests = fetch_player!("quests", CharacterQuests);
    let completed_quests = fetch_player!("completed_quests", CharacterCompletedQuests);
    let reputations = fetch_player!("reputations", CharacterReputationsSummary);
    let soulbinds = fetch_player!("soulbinds", CharacterSoulbindsSummary);
    let specializations = fetch_player!("specializations", CharacterSpecializationsSummary);
    let statistics = fetch_player!("statistics", CharacterStatisticsSummary);
    let titles = fetch_player!("titles", CharacterTitlesSummary);

    Ok(FullCharacter {
        realm_slug: realm_slug.to_string(),
        character_name: name.to_string(),
        has_profile_data,
        fetched_at,
        errors,
        profile: Some(profile),
        achievements,
        achievement_statistics,
        appearance,
        collections,
        mounts_collection,
        pets_collection,
        heirlooms_collection,
        toys_collection,
        encounters,
        dungeons,
        raids,
        equipment,
        hunter_pets,
        media,
        mythic_keystone_profile,
        mythic_keystone_season,
        professions,
        pvp_summary,
        pvp_2v2,
        pvp_3v3,
        pvp_rbg,
        quests,
        completed_quests,
        reputations,
        soulbinds,
        specializations,
        statistics,
        titles,
    })
}
