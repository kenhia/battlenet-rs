use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Quests Index ---

#[bendpoint(endpoint = "data/wow/quest/index" namespace = "static")]
struct QuestsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub categories: HrefLink,
    pub areas: HrefLink,
    pub types: HrefLink,
}

// --- Quest ---

#[bendpoint(endpoint = "data/wow/quest/{id}" url_args = "Id" namespace = "static")]
struct Quest {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
}

// --- Quest Categories Index ---

#[bendpoint(endpoint = "data/wow/quest/category/index" namespace = "static")]
struct QuestCategoriesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub categories: Vec<NameAndId>,
}

// --- Quest Category ---

#[bendpoint(endpoint = "data/wow/quest/category/{id}" url_args = "Id" namespace = "static")]
struct QuestCategory {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub category: String,
    pub quests: Option<Vec<NameAndId>>,
}

// --- Quest Areas Index ---

#[bendpoint(endpoint = "data/wow/quest/area/index" namespace = "static")]
struct QuestAreasIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub areas: Vec<NameAndId>,
}

// --- Quest Area ---

#[bendpoint(endpoint = "data/wow/quest/area/{id}" url_args = "Id" namespace = "static")]
struct QuestArea {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub area: String,
    pub quests: Option<Vec<NameAndId>>,
}

// --- Quest Types Index ---

#[bendpoint(endpoint = "data/wow/quest/type/index" namespace = "static")]
struct QuestTypesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub types: Vec<NameAndId>,
}

// --- Quest Type ---

#[bendpoint(endpoint = "data/wow/quest/type/{id}" url_args = "Id" namespace = "static")]
struct QuestType {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    #[serde(alias = "type")]
    pub type_: String,
    pub quests: Option<Vec<NameAndId>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_quests_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "categories": {"href": "https://test/categories"},
            "areas": {"href": "https://test/areas"},
            "types": {"href": "https://test/types"}
        }"#;
        let result: QuestsIndex = json_to_struct(json).unwrap();
        assert_eq!(result.categories.href, "https://test/categories");
    }

    #[test]
    fn test_quest() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 2,
            "title": "Sharptalon's Claw",
            "description": "Bring Sharptalon's Claw to Senani Thunderheart."
        }"#;
        let result: Quest = json_to_struct(json).unwrap();
        assert_eq!(result.id, 2);
        assert_eq!(result.title, "Sharptalon's Claw");
    }

    #[test]
    fn test_quest_category() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "category": "Epic",
            "quests": [
                {"key": {"href": "https://test"}, "name": "Some Quest", "id": 100}
            ]
        }"#;
        let result: QuestCategory = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.category, "Epic");
    }

    #[test]
    fn test_quest_type() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "type": "Group",
            "quests": null
        }"#;
        let result: QuestType = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.type_, "Group");
    }
}
