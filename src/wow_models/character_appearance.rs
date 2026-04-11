use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/appearance" url_args = "Player" namespace = "profile")]
struct CharacterAppearanceSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub character: NameAndId,
    pub playable_race: NameAndId,
    pub playable_class: NameAndId,
    pub active_spec: NameAndId,
    pub gender: TypeAndName,
    pub faction: TypeAndName,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_appearance_summary() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "character": {"key": {"href": "https://test"}, "name": "Belarsa", "id": 1001},
            "playable_race": {"key": {"href": "https://test"}, "name": "Gnome", "id": 7},
            "playable_class": {"key": {"href": "https://test"}, "name": "Priest", "id": 5},
            "active_spec": {"key": {"href": "https://test"}, "name": "Discipline", "id": 256},
            "gender": {"type": "FEMALE", "name": "Female"},
            "faction": {"type": "ALLIANCE", "name": "Alliance"}
        }"#;
        let result: CharacterAppearanceSummary = json_to_struct(json).unwrap();
        assert_eq!(result.character.name, "Belarsa");
        assert_eq!(result.playable_race.name, "Gnome");
    }
}
