use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Heirloom Index ---

#[bendpoint(endpoint = "data/wow/heirloom/index" namespace = "static")]
struct HeirloomIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub heirlooms: Vec<NameAndId>,
}

// --- Heirloom ---

#[bendpoint(endpoint = "data/wow/heirloom/{id}" url_args = "Id" namespace = "static")]
struct Heirloom {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub media: KeyAndId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_heirloom_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "heirlooms": [
                {"key": {"href": "https://test"}, "name": "Burnished Essence of the All-Flame", "id": 1}
            ]
        }"#;
        let result: HeirloomIndex = json_to_struct(json).unwrap();
        assert_eq!(result.heirlooms.len(), 1);
    }

    #[test]
    fn test_heirloom() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "name": "Burnished Essence of the All-Flame",
            "media": {"key": {"href": "https://test"}, "id": 1}
        }"#;
        let result: Heirloom = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "Burnished Essence of the All-Flame");
    }
}
