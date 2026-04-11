use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Item Classes Index ---

#[bendpoint(endpoint = "data/wow/item-class/index" namespace = "static")]
struct ItemClassesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub item_classes: Vec<NameAndId>,
}

// --- Item Class ---

#[bendpoint(endpoint = "data/wow/item-class/{id}" url_args = "Id" namespace = "static")]
struct ItemClass {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub class_id: u32,
    pub name: String,
    pub item_subclasses: Vec<NameAndId>,
}

// --- Item Subclass ---

#[bendpoint(endpoint = "data/wow/item-class/{id1}/item-subclass/{id2}" url_args = "TwoIds" namespace = "static")]
struct ItemSubclass {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub class_id: u32,
    pub subclass_id: u32,
    pub display_name: String,
}

// --- Item Sets Index ---

#[bendpoint(endpoint = "data/wow/item-set/index" namespace = "static")]
struct ItemSetsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub item_sets: Vec<NameAndId>,
}

// --- Item Set ---

#[bendpoint(endpoint = "data/wow/item-set/{id}" url_args = "Id" namespace = "static")]
struct ItemSet {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub items: Option<Vec<NameAndId>>,
}

// --- Item ---

#[bendpoint(endpoint = "data/wow/item/{id}" url_args = "Id" namespace = "static")]
struct Item {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub quality: TypeAndName,
    pub level: u32,
    pub required_level: u32,
    pub media: KeyAndId,
    pub item_class: NameAndId,
    pub item_subclass: NameAndId,
}

// --- Item Media ---

#[bendpoint(endpoint = "data/wow/media/item/{id}" url_args = "Id" namespace = "static")]
struct ItemMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

// --- Item Search ---

#[derive(Debug, Deserialize)]
pub struct ItemSearchData {
    pub id: u32,
    pub name: NameAndId,
    pub quality: TypeAndName,
    pub level: u32,
}

pub type ItemSearchResult = Result<SearchResult<ItemSearchData>, BattleNetClientError>;
pub type ItemSearchJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for SearchResult<ItemSearchData> {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let params = match url_args {
            UrlArgs::Search { params } => params,
            _ => panic!("UrlArgs::Search expected"),
        };
        let endpoint = "data/wow/search/item";
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
    fn test_item_classes_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "item_classes": [
                {"key": {"href": "https://test"}, "name": "Consumable", "id": 0}
            ]
        }"#;
        let result: ItemClassesIndex = json_to_struct(json).unwrap();
        assert_eq!(result.item_classes.len(), 1);
    }

    #[test]
    fn test_item_class() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "class_id": 2,
            "name": "Weapon",
            "item_subclasses": [
                {"key": {"href": "https://test"}, "name": "Axe", "id": 0}
            ]
        }"#;
        let result: ItemClass = json_to_struct(json).unwrap();
        assert_eq!(result.class_id, 2);
        assert_eq!(result.name, "Weapon");
    }

    #[test]
    fn test_item_subclass() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "class_id": 2,
            "subclass_id": 0,
            "display_name": "One-Handed Axes"
        }"#;
        let result: ItemSubclass = json_to_struct(json).unwrap();
        assert_eq!(result.class_id, 2);
        assert_eq!(result.subclass_id, 0);
    }

    #[test]
    fn test_item() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 19019,
            "name": "Thunderfury, Blessed Blade of the Windseeker",
            "quality": {"type": "LEGENDARY", "name": "Legendary"},
            "level": 80,
            "required_level": 60,
            "media": {"key": {"href": "https://test"}, "id": 19019},
            "item_class": {"key": {"href": "https://test"}, "name": "Weapon", "id": 2},
            "item_subclass": {"key": {"href": "https://test"}, "name": "Sword", "id": 7}
        }"#;
        let result: Item = json_to_struct(json).unwrap();
        assert_eq!(result.id, 19019);
        assert!(result.name.contains("Thunderfury"));
    }

    #[test]
    fn test_item_set_optional_items() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "name": "Tier 1"
        }"#;
        let result: ItemSet = json_to_struct(json).unwrap();
        assert!(result.items.is_none());
    }
}
