use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Realms Index ---

#[bendpoint(endpoint = "data/wow/realm/index" namespace = "dynamic")]
struct RealmsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub realms: Vec<NameAndId>,
}

// --- Realm ---

#[bendpoint(endpoint = "data/wow/realm/{id}" url_args = "Id" namespace = "dynamic")]
struct RealmData {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub region: NameAndId,
    pub name: String,
    pub category: String,
    pub locale: String,
    pub timezone: String,
    #[serde(alias = "type")]
    pub type_: TypeAndName,
    pub is_tournament: bool,
    pub slug: String,
}

// --- Realm Search ---

#[derive(Debug, Deserialize)]
pub struct RealmSearchData {
    pub id: u32,
    pub name: NameAndId,
    pub timezone: String,
    pub slug: String,
}

pub type RealmSearchResult = Result<SearchResult<RealmSearchData>, BattleNetClientError>;
pub type RealmSearchJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for SearchResult<RealmSearchData> {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let params = match url_args {
            UrlArgs::Search { params } => params,
            _ => panic!("UrlArgs::Search expected"),
        };
        let endpoint = "data/wow/search/realm";
        let namespace = WowNamespace::Dynamic.to_region_string(&client.region);
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
    fn test_realms_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "realms": [
                {"key": {"href": "https://test"}, "name": "Proudmoore", "id": 1}
            ]
        }"#;
        let result: RealmsIndex = json_to_struct(json).unwrap();
        assert_eq!(result.realms.len(), 1);
        assert_eq!(result.realms[0].name, "Proudmoore");
    }

    #[test]
    fn test_realm_data() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "region": {"key": {"href": "https://test"}, "name": "US", "id": 1},
            "name": "Proudmoore",
            "category": "United States",
            "locale": "enUS",
            "timezone": "America/Los_Angeles",
            "type": {"type": "NORMAL", "name": "Normal"},
            "is_tournament": false,
            "slug": "proudmoore"
        }"#;
        let result: RealmData = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "Proudmoore");
        assert!(!result.is_tournament);
    }
}
