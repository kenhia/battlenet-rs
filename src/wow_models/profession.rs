use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Professions Index ---

#[bendpoint(endpoint = "data/wow/profession/index" namespace = "static")]
struct ProfessionsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub professions: Vec<NameAndId>,
}

// --- Profession ---

#[bendpoint(endpoint = "data/wow/profession/{id}" url_args = "Id" namespace = "static")]
struct Profession {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub media: KeyAndId,
    pub skill_tiers: Option<Vec<NameAndId>>,
}

// --- Profession Media ---

#[bendpoint(endpoint = "data/wow/media/profession/{id}" url_args = "Id" namespace = "static")]
struct ProfessionMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

// --- Profession Skill Tier ---

#[bendpoint(endpoint = "data/wow/profession/{id1}/skill-tier/{id2}" url_args = "TwoIds" namespace = "static")]
struct ProfessionSkillTier {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub categories: Option<Vec<ProfessionCategory>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionCategory {
    pub name: String,
    pub recipes: Vec<NameAndId>,
}

// --- Recipe ---

#[bendpoint(endpoint = "data/wow/recipe/{id}" url_args = "Id" namespace = "static")]
struct Recipe {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub media: KeyAndId,
}

// --- Recipe Media ---

#[bendpoint(endpoint = "data/wow/media/recipe/{id}" url_args = "Id" namespace = "static")]
struct RecipeMedia {
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
    fn test_professions_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "professions": [
                {"key": {"href": "https://test"}, "name": "Blacksmithing", "id": 164}
            ]
        }"#;
        let result: ProfessionsIndex = json_to_struct(json).unwrap();
        assert_eq!(result.professions.len(), 1);
    }

    #[test]
    fn test_profession() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 164,
            "name": "Blacksmithing",
            "media": {"key": {"href": "https://test"}, "id": 164},
            "skill_tiers": [
                {"key": {"href": "https://test"}, "name": "Shadowlands Blacksmithing", "id": 2751}
            ]
        }"#;
        let result: Profession = json_to_struct(json).unwrap();
        assert_eq!(result.id, 164);
        assert!(result.skill_tiers.is_some());
    }

    #[test]
    fn test_profession_skill_tier() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 2751,
            "name": "Shadowlands Blacksmithing",
            "categories": [{
                "name": "Weapons",
                "recipes": [
                    {"key": {"href": "https://test"}, "name": "Shadowghast Ingot", "id": 35210}
                ]
            }]
        }"#;
        let result: ProfessionSkillTier = json_to_struct(json).unwrap();
        assert_eq!(result.id, 2751);
        assert!(result.categories.is_some());
    }

    #[test]
    fn test_recipe() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 35210,
            "name": "Shadowghast Ingot",
            "media": {"key": {"href": "https://test"}, "id": 35210}
        }"#;
        let result: Recipe = json_to_struct(json).unwrap();
        assert_eq!(result.id, 35210);
    }
}
