use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Toys Index ---

#[bendpoint(endpoint = "data/wow/toy/index" namespace = "static")]
struct ToysIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub toys: Vec<NameAndId>,
}

// --- Toy ---

#[bendpoint(endpoint = "data/wow/toy/{id}" url_args = "Id" namespace = "static")]
struct Toy {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub item: NameAndId,
    pub source: Option<TypeAndName>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_toys_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "toys": [
                {"key": {"href": "https://test"}, "name": "Foam Sword Rack", "id": 245}
            ]
        }"#;
        let result: ToysIndex = json_to_struct(json).unwrap();
        assert_eq!(result.toys.len(), 1);
        assert_eq!(result.toys[0].name, "Foam Sword Rack");
    }

    #[test]
    fn test_toy() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 245,
            "item": {"key": {"href": "https://test"}, "name": "Foam Sword Rack", "id": 119217},
            "source": {"type": "VENDOR", "name": "Vendor"}
        }"#;
        let result: Toy = json_to_struct(json).unwrap();
        assert_eq!(result.id, 245);
        assert_eq!(result.source.unwrap().name, "Vendor");
    }
}
