use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Regions Index ---

#[bendpoint(endpoint = "data/wow/region/index" namespace = "dynamic")]
struct RegionsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub regions: Vec<HrefLink>,
}

// --- Region ---

#[bendpoint(endpoint = "data/wow/region/{id}" url_args = "Id" namespace = "dynamic")]
struct RegionData {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub tag: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_regions_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "regions": [
                {"href": "https://us.api.blizzard.com/data/wow/region/1"}
            ]
        }"#;
        let result: RegionsIndex = json_to_struct(json).unwrap();
        assert_eq!(result.regions.len(), 1);
    }

    #[test]
    fn test_region_data() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "name": "North America",
            "tag": "US"
        }"#;
        let result: RegionData = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.tag, "US");
    }
}
