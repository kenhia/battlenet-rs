use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/character-media" url_args = "Player" namespace = "profile")]
struct CharacterMediaSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub character: NameAndId,
    pub assets: Option<Vec<Asset>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_media_summary() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "character": {"key": {"href": "https://test"}, "name": "Belarsa", "id": 1001},
            "assets": [
                {
                    "key": "avatar",
                    "value": "https://render.worldofwarcraft.com/avatar.jpg",
                    "file_data_id": 123456
                }
            ]
        }"#;
        let result: CharacterMediaSummary = json_to_struct(json).unwrap();
        assert_eq!(result.character.name, "Belarsa");
        assert!(result.assets.is_some());
        assert_eq!(result.assets.unwrap().len(), 1);
    }
}
