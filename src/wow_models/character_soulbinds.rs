use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/soulbinds" url_args = "Player" namespace = "profile")]
struct CharacterSoulbindsSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub character: NameAndId,
    pub chosen_covenant: Option<NameAndId>,
    pub renown_level: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_soulbinds_summary() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "character": {"key": {"href": "https://test"}, "name": "Belarsa", "id": 1001},
            "chosen_covenant": {"key": {"href": "https://test"}, "name": "Kyrian", "id": 1},
            "renown_level": 80
        }"#;
        let result: CharacterSoulbindsSummary = json_to_struct(json).unwrap();
        assert_eq!(result.character.name, "Belarsa");
        assert!(result.chosen_covenant.is_some());
        assert_eq!(result.chosen_covenant.unwrap().name, "Kyrian");
        assert_eq!(result.renown_level, Some(80));
    }
}
