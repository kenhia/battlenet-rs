use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Titles Index ---

#[bendpoint(endpoint = "data/wow/title/index" namespace = "static")]
struct TitlesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub titles: Vec<NameAndId>,
}

// --- Title ---

#[bendpoint(endpoint = "data/wow/title/{id}" url_args = "Id" namespace = "static")]
struct Title {
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
    fn test_titles_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "titles": [
                {"key": {"href": "https://test"}, "name": "Private", "id": 1}
            ]
        }"#;
        let result: TitlesIndex = json_to_struct(json).unwrap();
        assert_eq!(result.titles.len(), 1);
        assert_eq!(result.titles[0].name, "Private");
    }

    #[test]
    fn test_title() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "name": "Private"
        }"#;
        let result: Title = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "Private");
    }
}
