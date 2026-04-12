use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[derive(Debug, Serialize, Deserialize)]
pub struct HunterPet {
    pub name: String,
    pub level: u32,
    pub creature: NameAndId,
    pub slot: u32,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/hunter-pets" url_args = "Player" namespace = "profile")]
struct CharacterHunterPetsSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub character: NameAndId,
    pub hunter_pets: Vec<HunterPet>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_hunter_pets_summary() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "character": {"key": {"href": "https://test"}, "name": "Belarsa", "id": 1001},
            "hunter_pets": [
                {
                    "name": "Cat",
                    "level": 70,
                    "creature": {"key": {"href": "https://test"}, "name": "Cat", "id": 42},
                    "slot": 0
                }
            ]
        }"#;
        let result: CharacterHunterPetsSummary = json_to_struct(json).unwrap();
        assert_eq!(result.character.name, "Belarsa");
        assert_eq!(result.hunter_pets.len(), 1);
        assert_eq!(result.hunter_pets[0].name, "Cat");
        assert_eq!(result.hunter_pets[0].level, 70);
    }
}
