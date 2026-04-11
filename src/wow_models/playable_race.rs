use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Playable Races Index ---

#[bendpoint(endpoint = "data/wow/playable-race/index" namespace = "static")]
struct PlayableRacesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub races: Vec<NameAndId>,
}

// --- Playable Race ---

#[bendpoint(endpoint = "data/wow/playable-race/{id}" url_args = "Id" namespace = "static")]
struct PlayableRace {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub faction: TypeAndName,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_playable_races_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "races": [
                {"key": {"href": "https://test"}, "name": "Human", "id": 1}
            ]
        }"#;
        let result: PlayableRacesIndex = json_to_struct(json).unwrap();
        assert_eq!(result.races.len(), 1);
        assert_eq!(result.races[0].name, "Human");
    }

    #[test]
    fn test_playable_race() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "name": "Human",
            "faction": {"type": "ALLIANCE", "name": "Alliance"}
        }"#;
        let result: PlayableRace = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "Human");
    }
}
