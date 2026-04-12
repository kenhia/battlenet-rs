use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/titles" url_args = "Player" namespace = "profile")]
struct CharacterTitlesSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub character: NameAndId,
    pub active_title: Option<CharacterTitle>,
    pub titles: Vec<NameAndId>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_titles_summary() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "character": {"key": {"href": "https://test"}, "name": "Belarsa", "id": 1001},
            "active_title": {"name": "Professor", "id": 139, "display_string": "Professor {name}"},
            "titles": [
                {"key": {"href": "https://test"}, "name": "Professor", "id": 139}
            ]
        }"#;
        let result: CharacterTitlesSummary = json_to_struct(json).unwrap();
        assert_eq!(result.character.name, "Belarsa");
        assert!(result.active_title.is_some());
        assert_eq!(result.active_title.unwrap().name, "Professor");
        assert_eq!(result.titles.len(), 1);
    }
}
