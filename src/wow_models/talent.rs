use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Talent Tree Index ---

#[bendpoint(endpoint = "data/wow/talent-tree/index" namespace = "static")]
struct TalentTreeIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub spec_talent_trees: Vec<NameAndId>,
}

// --- Talent Tree ---

#[bendpoint(endpoint = "data/wow/talent-tree/{id}" url_args = "Id" namespace = "static")]
struct TalentTree {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub playable_class: NameAndId,
    pub name: String,
}

// --- Talent Tree Nodes ---

#[bendpoint(endpoint = "data/wow/talent-tree/{id1}/playable-specialization/{id2}" url_args = "TwoIds" namespace = "static")]
struct TalentTreeNodes {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub spec_talent_trees: Vec<NameAndId>,
}

// --- Talents Index ---

#[bendpoint(endpoint = "data/wow/talent/index" namespace = "static")]
struct TalentsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub talents: Vec<NameAndId>,
}

// --- Talent ---

#[bendpoint(endpoint = "data/wow/talent/{id}" url_args = "Id" namespace = "static")]
struct Talent {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: Option<String>,
    pub spell: Option<NameAndId>,
    pub playable_class: Option<NameAndId>,
}

// --- PvP Talents Index ---

#[bendpoint(endpoint = "data/wow/pvp-talent/index" namespace = "static")]
struct PvpTalentsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub pvp_talents: Vec<NameAndId>,
}

// --- PvP Talent ---

#[bendpoint(endpoint = "data/wow/pvp-talent/{id}" url_args = "Id" namespace = "static")]
struct PvpTalent {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub spell: NameAndId,
    pub playable_specialization: NameAndId,
}

// --- Tech Talent Tree Index ---

#[bendpoint(endpoint = "data/wow/tech-talent-tree/index" namespace = "static")]
struct TechTalentTreeIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub talent_trees: Vec<NameAndId>,
}

// --- Tech Talent Tree ---

#[bendpoint(endpoint = "data/wow/tech-talent-tree/{id}" url_args = "Id" namespace = "static")]
struct TechTalentTree {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
}

// --- Tech Talents Index ---

#[bendpoint(endpoint = "data/wow/tech-talent/index" namespace = "static")]
struct TechTalentsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub talents: Vec<NameAndId>,
}

// --- Tech Talent ---

#[bendpoint(endpoint = "data/wow/tech-talent/{id}" url_args = "Id" namespace = "static")]
struct TechTalent {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub media: KeyAndId,
}

// --- Tech Talent Media ---

#[bendpoint(endpoint = "data/wow/media/tech-talent/{id}" url_args = "Id" namespace = "static")]
struct TechTalentMedia {
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
    fn test_talent_tree_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "spec_talent_trees": [
                {"key": {"href": "https://test"}, "name": "Warrior", "id": 1}
            ]
        }"#;
        let result: TalentTreeIndex = json_to_struct(json).unwrap();
        assert_eq!(result.spec_talent_trees.len(), 1);
    }

    #[test]
    fn test_talent_tree() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "playable_class": {"key": {"href": "https://test"}, "name": "Warrior", "id": 1},
            "name": "Class Talents"
        }"#;
        let result: TalentTree = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
    }

    #[test]
    fn test_talent_optional_fields() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 42
        }"#;
        let result: Talent = json_to_struct(json).unwrap();
        assert_eq!(result.id, 42);
        assert!(result.name.is_none());
        assert!(result.spell.is_none());
        assert!(result.playable_class.is_none());
    }

    #[test]
    fn test_pvp_talent() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "spell": {"key": {"href": "https://test"}, "name": "Gladiator's Medallion", "id": 208683},
            "playable_specialization": {"key": {"href": "https://test"}, "name": "Arms", "id": 71}
        }"#;
        let result: PvpTalent = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
    }

    #[test]
    fn test_tech_talent() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "name": "Volatile Solvent",
            "description": "Using Fleshcraft with nearby corpses increases your stats.",
            "media": {"key": {"href": "https://test"}, "id": 1}
        }"#;
        let result: TechTalent = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert!(result.description.is_some());
    }

    #[test]
    fn test_talent_tree_nodes() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 786,
            "spec_talent_trees": [
                {"key": {"href": "https://test"}, "name": "Arms", "id": 71}
            ]
        }"#;
        let result: TalentTreeNodes = json_to_struct(json).unwrap();
        assert_eq!(result.id, 786);
    }
}
