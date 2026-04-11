use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- PvP Seasons Index ---

#[bendpoint(endpoint = "data/wow/pvp-season/index" namespace = "dynamic")]
struct PvpSeasonsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub seasons: Vec<KeyAndId>,
}

// --- PvP Season ---

#[bendpoint(endpoint = "data/wow/pvp-season/{id}" url_args = "Id" namespace = "dynamic")]
struct PvpSeason {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
}

// --- PvP Leaderboards Index ---

#[bendpoint(endpoint = "data/wow/pvp-season/{id}/pvp-leaderboard/index" url_args = "Id" namespace = "dynamic")]
struct PvpLeaderboardsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub season: KeyAndId,
    pub leaderboards: Vec<NameAndId>,
}

// --- PvP Leaderboard ---

#[bendpoint(endpoint = "data/wow/pvp-season/{id1}/pvp-leaderboard/{id2}" url_args = "TwoIds" namespace = "dynamic")]
struct PvpLeaderboard {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub season: KeyAndId,
    pub name: String,
}

// --- PvP Rewards Index ---

#[bendpoint(endpoint = "data/wow/pvp-season/{id}/pvp-reward/index" url_args = "Id" namespace = "dynamic")]
struct PvpRewardsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub season: KeyAndId,
    pub rewards: Vec<PvpReward>,
}

#[derive(Debug, Deserialize)]
pub struct PvpReward {
    pub bracket: TypeAndName,
    pub achievement: NameAndId,
}

// --- PvP Tiers Index ---

#[bendpoint(endpoint = "data/wow/pvp-tier/index" namespace = "dynamic")]
struct PvpTiersIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub tiers: Vec<KeyAndId>,
}

// --- PvP Tier ---

#[bendpoint(endpoint = "data/wow/pvp-tier/{id}" url_args = "Id" namespace = "dynamic")]
struct PvpTier {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub min_rating: u32,
    pub max_rating: u32,
    pub media: KeyAndId,
}

// --- PvP Tier Media ---

#[bendpoint(endpoint = "data/wow/media/pvp-tier/{id}" url_args = "Id" namespace = "dynamic")]
struct PvpTierMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_pvp_seasons_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "seasons": [
                {"key": {"href": "https://test"}, "id": 33}
            ]
        }"#;
        let result: PvpSeasonsIndex = json_to_struct(json).unwrap();
        assert_eq!(result.seasons.len(), 1);
    }

    #[test]
    fn test_pvp_season() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 33
        }"#;
        let result: PvpSeason = json_to_struct(json).unwrap();
        assert_eq!(result.id, 33);
    }

    #[test]
    fn test_pvp_tier() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "name": "Combatant I",
            "min_rating": 0,
            "max_rating": 1399,
            "media": {"key": {"href": "https://test"}, "id": 1}
        }"#;
        let result: PvpTier = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.min_rating, 0);
    }

    #[test]
    fn test_pvp_rewards_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "season": {"key": {"href": "https://test"}, "id": 33},
            "rewards": [{
                "bracket": {"type": "ARENA_3v3", "name": "3v3"},
                "achievement": {"key": {"href": "https://test"}, "name": "Gladiator", "id": 2091}
            }]
        }"#;
        let result: PvpRewardsIndex = json_to_struct(json).unwrap();
        assert_eq!(result.rewards.len(), 1);
    }
}
