use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Playable Specializations Index ---

#[bendpoint(endpoint = "data/wow/playable-specialization/index" namespace = "static")]
struct PlayableSpecializationsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub character_specializations: Vec<NameAndId>,
}

// --- Playable Specialization ---

#[bendpoint(endpoint = "data/wow/playable-specialization/{id}" url_args = "Id" namespace = "static")]
struct PlayableSpecialization {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub playable_class: NameAndId,
    pub name: String,
    pub description: Option<String>,
    pub media: KeyAndId,
    pub role: TypeAndName,
}

// --- Playable Specialization Media ---

#[bendpoint(endpoint = "data/wow/media/playable-specialization/{id}" url_args = "Id" namespace = "static")]
struct PlayableSpecializationMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_playable_specializations_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "character_specializations": [
                {"key": {"href": "https://test"}, "name": "Arms", "id": 71}
            ]
        }"#;
        let result: PlayableSpecializationsIndex = json_to_struct(json).unwrap();
        assert_eq!(result.character_specializations.len(), 1);
        assert_eq!(result.character_specializations[0].name, "Arms");
    }

    #[test]
    fn test_playable_specialization() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 71,
            "playable_class": {"key": {"href": "https://test"}, "name": "Warrior", "id": 1},
            "name": "Arms",
            "description": "A battle-hardened master of weapons.",
            "media": {"key": {"href": "https://test"}, "id": 71},
            "role": {"type": "DAMAGE", "name": "Damage"}
        }"#;
        let result: PlayableSpecialization = json_to_struct(json).unwrap();
        assert_eq!(result.id, 71);
        assert_eq!(result.name, "Arms");
    }
}
