use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterReputationStanding {
    pub raw: i32,
    pub value: i32,
    pub max: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterReputation {
    pub faction: NameAndId,
    pub standing: CharacterReputationStanding,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/reputations" url_args = "Player" namespace = "profile")]
struct CharacterReputationsSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub character: NameAndId,
    pub reputations: Vec<CharacterReputation>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_reputations_summary() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "character": {"key": {"href": "https://test"}, "name": "Belarsa", "id": 1001},
            "reputations": [
                {
                    "faction": {"key": {"href": "https://test"}, "name": "Stormwind", "id": 72},
                    "standing": {"raw": 42999, "value": 999, "max": 1000, "name": "Exalted"}
                }
            ]
        }"#;
        let result: CharacterReputationsSummary = json_to_struct(json).unwrap();
        assert_eq!(result.character.name, "Belarsa");
        assert_eq!(result.reputations.len(), 1);
        assert_eq!(result.reputations[0].standing.name, "Exalted");
    }
}
