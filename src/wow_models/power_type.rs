use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Power Types Index ---

#[bendpoint(endpoint = "data/wow/power-type/index" namespace = "static")]
struct PowerTypesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub power_types: Vec<NameAndId>,
}

// --- Power Type ---

#[bendpoint(endpoint = "data/wow/power-type/{id}" url_args = "Id" namespace = "static")]
struct PowerType {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_power_types_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "power_types": [
                {"key": {"href": "https://test"}, "name": "Mana", "id": 0}
            ]
        }"#;
        let result: PowerTypesIndex = json_to_struct(json).unwrap();
        assert_eq!(result.power_types.len(), 1);
        assert_eq!(result.power_types[0].name, "Mana");
    }

    #[test]
    fn test_power_type() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 0,
            "name": "Mana"
        }"#;
        let result: PowerType = json_to_struct(json).unwrap();
        assert_eq!(result.id, 0);
        assert_eq!(result.name, "Mana");
    }
}
