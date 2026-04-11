use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Spell ---

#[bendpoint(endpoint = "data/wow/spell/{id}" url_args = "Id" namespace = "static")]
struct Spell {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub media: KeyAndId,
}

// --- Spell Media ---

#[bendpoint(endpoint = "data/wow/media/spell/{id}" url_args = "Id" namespace = "static")]
struct SpellMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

// --- Spell Search ---

#[derive(Debug, Deserialize)]
pub struct SpellSearchData {
    pub id: u32,
    pub name: NameAndId,
}

pub type SpellSearchResult = Result<SearchResult<SpellSearchData>, BattleNetClientError>;
pub type SpellSearchJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for SearchResult<SpellSearchData> {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let params = match url_args {
            UrlArgs::Search { params } => params,
            _ => panic!("UrlArgs::Search expected"),
        };
        let endpoint = "data/wow/search/spell";
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
    fn test_spell() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1459,
            "name": "Arcane Intellect",
            "description": "Infuses the target with brilliance.",
            "media": {"key": {"href": "https://test"}, "id": 1459}
        }"#;
        let result: Spell = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1459);
        assert_eq!(result.name, "Arcane Intellect");
        assert!(result.description.is_some());
    }

    #[test]
    fn test_spell_no_description() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 99999,
            "name": "Unknown Spell",
            "media": {"key": {"href": "https://test"}, "id": 99999}
        }"#;
        let result: Spell = json_to_struct(json).unwrap();
        assert!(result.description.is_none());
    }
}
