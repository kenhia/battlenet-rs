use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSpecRef {
    pub specialization: NameAndId,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/specializations" url_args = "Player" namespace = "profile")]
struct CharacterSpecializationsSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub specializations: Vec<CharacterSpecRef>,
    pub active_specialization: NameAndId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_specializations_summary() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "specializations": [
                {
                    "specialization": {"key": {"href": "https://test"}, "name": "Discipline", "id": 256}
                }
            ],
            "active_specialization": {"key": {"href": "https://test"}, "name": "Discipline", "id": 256}
        }"#;
        let result: CharacterSpecializationsSummary = json_to_struct(json).unwrap();
        assert_eq!(result.active_specialization.name, "Discipline");
        assert_eq!(result.specializations.len(), 1);
    }
}
