use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[derive(Debug, Deserialize)]
pub struct CharacterProfessionTier {
    pub skill_points: u32,
    pub max_skill_points: u32,
    pub tier: NameAndId,
}

#[derive(Debug, Deserialize)]
pub struct CharacterProfessionRef {
    pub profession: NameAndId,
    pub tiers: Option<Vec<CharacterProfessionTier>>,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/professions" url_args = "Player" namespace = "profile")]
struct CharacterProfessionsSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub character: NameAndId,
    pub primaries: Option<Vec<CharacterProfessionRef>>,
    pub secondaries: Option<Vec<CharacterProfessionRef>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_professions_summary() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "character": {"key": {"href": "https://test"}, "name": "Belarsa", "id": 1001},
            "primaries": [
                {
                    "profession": {"key": {"href": "https://test"}, "name": "Tailoring", "id": 197},
                    "tiers": [
                        {
                            "skill_points": 100,
                            "max_skill_points": 300,
                            "tier": {"key": {"href": "https://test"}, "name": "Classic", "id": 2539}
                        }
                    ]
                }
            ],
            "secondaries": null
        }"#;
        let result: CharacterProfessionsSummary = json_to_struct(json).unwrap();
        assert_eq!(result.character.name, "Belarsa");
        assert!(result.primaries.is_some());
        assert_eq!(result.primaries.unwrap().len(), 1);
    }
}
