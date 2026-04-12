use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Journal Expansions Index ---

#[bendpoint(endpoint = "data/wow/journal-expansion/index" namespace = "static")]
struct JournalExpansionsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub tiers: Vec<NameAndId>,
}

// --- Journal Expansion ---

#[bendpoint(endpoint = "data/wow/journal-expansion/{id}" url_args = "Id" namespace = "static")]
struct JournalExpansion {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub dungeons: Option<Vec<NameAndId>>,
    pub raids: Option<Vec<NameAndId>>,
}

// --- Journal Encounters Index ---

#[bendpoint(endpoint = "data/wow/journal-encounter/index" namespace = "static")]
struct JournalEncountersIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub encounters: Vec<NameAndId>,
}

// --- Journal Encounter ---

#[bendpoint(endpoint = "data/wow/journal-encounter/{id}" url_args = "Id" namespace = "static")]
struct JournalEncounter {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub instance: NameAndId,
}

// --- Journal Instances Index ---

#[bendpoint(endpoint = "data/wow/journal-instance/index" namespace = "static")]
struct JournalInstancesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub instances: Vec<NameAndId>,
}

// --- Journal Instance ---

#[bendpoint(endpoint = "data/wow/journal-instance/{id}" url_args = "Id" namespace = "static")]
struct JournalInstance {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub encounters: Option<Vec<NameAndId>>,
    pub expansion: NameAndId,
    pub media: KeyAndId,
}

// --- Journal Instance Media ---

#[bendpoint(endpoint = "data/wow/media/journal-instance/{id}" url_args = "Id" namespace = "static")]
struct JournalInstanceMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

// --- Journal Encounter Search ---

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalEncounterSearchData {
    pub id: u32,
    pub name: NameAndId,
}

pub type JournalEncounterSearchResult =
    Result<SearchResult<JournalEncounterSearchData>, BattleNetClientError>;
pub type JournalEncounterSearchJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for SearchResult<JournalEncounterSearchData> {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let params = match url_args {
            UrlArgs::Search { params } => params,
            _ => panic!("UrlArgs::Search expected"),
        };
        let endpoint = "data/wow/search/journal-encounter";
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
    fn test_journal_expansions_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "tiers": [
                {"key": {"href": "https://test"}, "name": "Classic", "id": 68}
            ]
        }"#;
        let result: JournalExpansionsIndex = json_to_struct(json).unwrap();
        assert_eq!(result.tiers.len(), 1);
    }

    #[test]
    fn test_journal_expansion() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 68,
            "name": "Classic",
            "dungeons": [
                {"key": {"href": "https://test"}, "name": "Deadmines", "id": 63}
            ]
        }"#;
        let result: JournalExpansion = json_to_struct(json).unwrap();
        assert_eq!(result.id, 68);
        assert!(result.dungeons.is_some());
        assert!(result.raids.is_none());
    }

    #[test]
    fn test_journal_encounter() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 89,
            "name": "Edwin VanCleef",
            "description": "The leader of the Defias Brotherhood.",
            "instance": {"key": {"href": "https://test"}, "name": "Deadmines", "id": 63}
        }"#;
        let result: JournalEncounter = json_to_struct(json).unwrap();
        assert_eq!(result.id, 89);
        assert_eq!(result.name, "Edwin VanCleef");
    }

    #[test]
    fn test_journal_instance() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 63,
            "name": "Deadmines",
            "description": "It is said the weights of Edwin VanCleef...",
            "encounters": [
                {"key": {"href": "https://test"}, "name": "Edwin VanCleef", "id": 89}
            ],
            "expansion": {"key": {"href": "https://test"}, "name": "Classic", "id": 68},
            "media": {"key": {"href": "https://test"}, "id": 63}
        }"#;
        let result: JournalInstance = json_to_struct(json).unwrap();
        assert_eq!(result.id, 63);
        assert!(result.encounters.is_some());
    }
}
