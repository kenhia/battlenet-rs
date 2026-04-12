use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Azerite Essence Index ---

#[bendpoint(endpoint = "data/wow/azerite-essence/index" namespace = "static")]
struct AzeriteEssencesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub azerite_essences: Vec<NameAndId>,
}

// --- Azerite Essence ---

#[bendpoint(endpoint = "data/wow/azerite-essence/{id}" url_args = "Id" namespace = "static")]
struct AzeriteEssence {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub allowed_specializations: Option<Vec<NameAndId>>,
    pub media: KeyAndId,
}

// --- Azerite Essence Media ---

#[bendpoint(endpoint = "data/wow/media/azerite-essence/{id}" url_args = "Id" namespace = "static")]
struct AzeriteEssenceMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

// --- Azerite Essence Search ---

#[derive(Debug, Serialize, Deserialize)]
pub struct AzeriteEssenceSearchData {
    pub allowed_specializations: Option<Vec<NameAndId>>,
    pub id: u32,
    pub name: NameAndId,
}

pub type AzeriteEssenceSearchResult =
    Result<SearchResult<AzeriteEssenceSearchData>, BattleNetClientError>;
pub type AzeriteEssenceSearchJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for SearchResult<AzeriteEssenceSearchData> {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let params = match url_args {
            UrlArgs::Search { params } => params,
            _ => panic!("UrlArgs::Search expected"),
        };
        let endpoint = "data/wow/search/azerite-essence";
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
    fn test_azerite_essences_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://us.api.blizzard.com/data/wow/azerite-essence/index"}},
            "azerite_essences": [
                {"key": {"href": "https://test"}, "name": "Azeroth's Undying Gift", "id": 2}
            ]
        }"#;
        let result: AzeriteEssencesIndex = json_to_struct(json).unwrap();
        assert_eq!(result.azerite_essences.len(), 1);
        assert_eq!(result.azerite_essences[0].id, 2);
    }

    #[test]
    fn test_azerite_essence() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 2,
            "name": "Azeroth's Undying Gift",
            "allowed_specializations": [
                {"key": {"href": "https://test"}, "name": "Blood", "id": 250}
            ],
            "media": {"key": {"href": "https://test"}, "id": 2}
        }"#;
        let result: AzeriteEssence = json_to_struct(json).unwrap();
        assert_eq!(result.id, 2);
        assert_eq!(result.name, "Azeroth's Undying Gift");
    }
}
