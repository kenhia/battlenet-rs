use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/quests" url_args = "Player" namespace = "profile")]
struct CharacterQuests {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub character: NameAndId,
    pub in_progress: Option<Vec<NameAndId>>,
    pub completed: HrefLink,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/quests/completed" url_args = "Player" namespace = "profile")]
struct CharacterCompletedQuests {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub character: NameAndId,
    pub quests: Vec<NameAndId>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_quests() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "character": {"key": {"href": "https://test"}, "name": "Belarsa", "id": 1001},
            "in_progress": [
                {"key": {"href": "https://test"}, "name": "A Noble Deed", "id": 100}
            ],
            "completed": {"href": "https://test/completed"}
        }"#;
        let result: CharacterQuests = json_to_struct(json).unwrap();
        assert_eq!(result.character.name, "Belarsa");
        assert!(result.in_progress.is_some());
        assert_eq!(result.completed.href, "https://test/completed");
    }
}
