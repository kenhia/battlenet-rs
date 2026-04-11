use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Modified Crafting Index ---

#[bendpoint(endpoint = "data/wow/modified-crafting/index" namespace = "static")]
struct ModifiedCraftingIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub categories: HrefLink,
    pub slot_types: HrefLink,
}

// --- Modified Crafting Category Index ---

#[bendpoint(endpoint = "data/wow/modified-crafting/category/index" namespace = "static")]
struct ModifiedCraftingCategoryIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub categories: Vec<NameAndId>,
}

// --- Modified Crafting Category ---

#[bendpoint(endpoint = "data/wow/modified-crafting/category/{id}" url_args = "Id" namespace = "static")]
struct ModifiedCraftingCategory {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: Option<String>,
}

// --- Modified Crafting Slot Type Index ---

#[bendpoint(endpoint = "data/wow/modified-crafting/reagent-slot-type/index" namespace = "static")]
struct ModifiedCraftingSlotTypeIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub slot_types: Vec<NameAndId>,
}

// --- Modified Crafting Slot Type ---

#[bendpoint(endpoint = "data/wow/modified-crafting/reagent-slot-type/{id}" url_args = "Id" namespace = "static")]
struct ModifiedCraftingSlotType {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub description: Option<String>,
    pub compatible_categories: Option<Vec<NameAndId>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_modified_crafting_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "categories": {"href": "https://test/categories"},
            "slot_types": {"href": "https://test/slot-types"}
        }"#;
        let result: ModifiedCraftingIndex = json_to_struct(json).unwrap();
        assert_eq!(result.categories.href, "https://test/categories");
    }

    #[test]
    fn test_modified_crafting_category() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "name": "Enchanting"
        }"#;
        let result: ModifiedCraftingCategory = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.name, Some("Enchanting".to_string()));
    }

    #[test]
    fn test_modified_crafting_slot_type() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "description": "Missive",
            "compatible_categories": [
                {"key": {"href": "https://test"}, "name": "Category 1", "id": 1}
            ]
        }"#;
        let result: ModifiedCraftingSlotType = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.description, Some("Missive".to_string()));
    }
}
