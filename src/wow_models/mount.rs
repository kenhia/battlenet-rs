use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Mounts Index ---

#[bendpoint(endpoint = "data/wow/mount/index" namespace = "static")]
struct MountsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub mounts: Vec<NameAndId>,
}

// --- Mount ---

#[bendpoint(endpoint = "data/wow/mount/{id}" url_args = "Id" namespace = "static")]
struct Mount {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub source: Option<TypeAndName>,
    pub faction: Option<TypeAndName>,
}

// --- Mount Search ---

#[derive(Debug, Serialize, Deserialize)]
pub struct MountSearchData {
    pub id: u32,
    pub name: NameAndId,
}

pub type MountSearchResult = Result<SearchResult<MountSearchData>, BattleNetClientError>;
pub type MountSearchJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for SearchResult<MountSearchData> {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let params = match url_args {
            UrlArgs::Search { params } => params,
            _ => panic!("UrlArgs::Search expected"),
        };
        let endpoint = "data/wow/search/mount";
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
    fn test_mounts_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "mounts": [
                {"key": {"href": "https://test"}, "name": "Invincible", "id": 363}
            ]
        }"#;
        let result: MountsIndex = json_to_struct(json).unwrap();
        assert_eq!(result.mounts.len(), 1);
        assert_eq!(result.mounts[0].name, "Invincible");
    }

    #[test]
    fn test_mount() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 363,
            "name": "Invincible",
            "description": "Invincible's reins",
            "source": {"type": "DROP", "name": "Drop"},
            "faction": null
        }"#;
        let result: Mount = json_to_struct(json).unwrap();
        assert_eq!(result.id, 363);
        assert_eq!(result.name, "Invincible");
    }
}
