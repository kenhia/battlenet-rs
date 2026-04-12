use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Creature Families Index ---

#[bendpoint(endpoint = "data/wow/creature-family/index" namespace = "static")]
struct CreatureFamiliesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub creature_families: Vec<NameAndId>,
}

// --- Creature Family ---

#[bendpoint(endpoint = "data/wow/creature-family/{id}" url_args = "Id" namespace = "static")]
struct CreatureFamily {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub media: KeyAndId,
}

// --- Creature Types Index ---

#[bendpoint(endpoint = "data/wow/creature-type/index" namespace = "static")]
struct CreatureTypesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub creature_types: Vec<NameAndId>,
}

// --- Creature Type ---

#[bendpoint(endpoint = "data/wow/creature-type/{id}" url_args = "Id" namespace = "static")]
struct CreatureType {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
}

// --- Creature ---

#[bendpoint(endpoint = "data/wow/creature/{id}" url_args = "Id" namespace = "static")]
struct Creature {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    #[serde(alias = "type")]
    pub type_: NameAndId,
    pub family: Option<NameAndId>,
    pub media: KeyAndId,
}

// --- Creature Display Media ---

#[bendpoint(endpoint = "data/wow/media/creature-display/{id}" url_args = "Id" namespace = "static")]
struct CreatureDisplayMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

// --- Creature Search ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureSearchData {
    pub id: u32,
    pub name: NameAndId,
}

pub type CreatureSearchResult = Result<SearchResult<CreatureSearchData>, BattleNetClientError>;
pub type CreatureSearchJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for SearchResult<CreatureSearchData> {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let params = match url_args {
            UrlArgs::Search { params } => params,
            _ => panic!("UrlArgs::Search expected"),
        };
        let endpoint = "data/wow/search/creature";
        let namespace = WowNamespace::Static.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;
        let mut url = format!("{base}/{endpoint}?namespace={namespace}&locale={locale}");
        for (key, value) in params {
            url.push_str(&format!("&{key}={value}"));
        }
        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_creature_families_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "creature_families": [
                {"key": {"href": "https://test"}, "name": "Wolf", "id": 1}
            ]
        }"#;
        let result: CreatureFamiliesIndex = json_to_struct(json).unwrap();
        assert_eq!(result.creature_families.len(), 1);
        assert_eq!(result.creature_families[0].name, "Wolf");
    }

    #[test]
    fn test_creature_family() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "name": "Wolf",
            "media": {"key": {"href": "https://test"}, "id": 1}
        }"#;
        let result: CreatureFamily = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "Wolf");
    }

    #[test]
    fn test_creature_with_optional_family() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 42,
            "name": "Hogger",
            "type": {"key": {"href": "https://test"}, "name": "Humanoid", "id": 7},
            "media": {"key": {"href": "https://test"}, "id": 42}
        }"#;
        let result: Creature = json_to_struct(json).unwrap();
        assert_eq!(result.id, 42);
        assert!(result.family.is_none());
    }

    #[test]
    fn test_creature_display_media() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "assets": [{"key": "icon", "value": "https://test/icon.jpg", "file_data_id": 123}],
            "id": 42
        }"#;
        let result: CreatureDisplayMedia = json_to_struct(json).unwrap();
        assert_eq!(result.id, 42);
        assert_eq!(result.assets.len(), 1);
    }
}
