use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Reputation Factions Index ---

#[bendpoint(endpoint = "data/wow/reputation-faction/index" namespace = "static")]
struct ReputationFactionsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub factions: Vec<NameAndId>,
    pub root_factions: Vec<NameAndId>,
}

// --- Reputation Faction ---

#[bendpoint(endpoint = "data/wow/reputation-faction/{id}" url_args = "Id" namespace = "static")]
struct ReputationFaction {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}

// --- Reputation Tiers Index ---

#[bendpoint(endpoint = "data/wow/reputation-tiers/index" namespace = "static")]
struct ReputationTiersIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub reputation_tiers: Vec<NameAndId>,
}

// --- Reputation Tiers ---

#[derive(Debug, Deserialize)]
pub struct ReputationTier {
    pub name: String,
    pub min_value: i64,
    pub max_value: i64,
    pub id: u32,
}

#[bendpoint(endpoint = "data/wow/reputation-tiers/{id}" url_args = "Id" namespace = "static")]
struct ReputationTiers {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub tiers: Vec<ReputationTier>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_reputation_factions_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "factions": [
                {"key": {"href": "https://test"}, "name": "Stormwind", "id": 72}
            ],
            "root_factions": [
                {"key": {"href": "https://test"}, "name": "Alliance", "id": 469}
            ]
        }"#;
        let result: ReputationFactionsIndex = json_to_struct(json).unwrap();
        assert_eq!(result.factions.len(), 1);
        assert_eq!(result.root_factions.len(), 1);
    }

    #[test]
    fn test_reputation_faction() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 72,
            "name": "Stormwind",
            "description": "The capital city of the Alliance."
        }"#;
        let result: ReputationFaction = json_to_struct(json).unwrap();
        assert_eq!(result.id, 72);
        assert_eq!(result.name, "Stormwind");
    }

    #[test]
    fn test_reputation_tiers() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 2,
            "tiers": [
                {"name": "Hated", "min_value": -42000, "max_value": -6000, "id": 0},
                {"name": "Hostile", "min_value": -6000, "max_value": -3000, "id": 1}
            ]
        }"#;
        let result: ReputationTiers = json_to_struct(json).unwrap();
        assert_eq!(result.id, 2);
        assert_eq!(result.tiers.len(), 2);
        assert_eq!(result.tiers[0].name, "Hated");
        assert_eq!(result.tiers[0].min_value, -42000);
    }
}
