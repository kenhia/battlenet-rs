use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[derive(Debug, Deserialize)]
pub struct EquippedItemLevel {
    pub value: u32,
    pub display_string: String,
}

#[derive(Debug, Deserialize)]
pub struct EquippedItem {
    pub item: KeyAndId,
    pub slot: TypeAndName,
    pub quantity: u32,
    pub quality: TypeAndName,
    pub name: String,
    pub level: EquippedItemLevel,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/equipment" url_args = "Player" namespace = "profile")]
struct CharacterEquipmentSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub character: NameAndId,
    pub equipped_items: Vec<EquippedItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_equipment_summary() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "character": {"key": {"href": "https://test"}, "name": "Belarsa", "id": 1001},
            "equipped_items": [
                {
                    "item": {"key": {"href": "https://test"}, "id": 178692},
                    "slot": {"type": "HEAD", "name": "Head"},
                    "quantity": 1,
                    "quality": {"type": "EPIC", "name": "Epic"},
                    "name": "Cowl of the Devoted General",
                    "level": {"value": 226, "display_string": "Item Level 226"}
                }
            ]
        }"#;
        let result: CharacterEquipmentSummary = json_to_struct(json).unwrap();
        assert_eq!(result.character.name, "Belarsa");
        assert_eq!(result.equipped_items.len(), 1);
        assert_eq!(result.equipped_items[0].name, "Cowl of the Devoted General");
        assert_eq!(result.equipped_items[0].level.value, 226);
    }
}
