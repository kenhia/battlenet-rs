use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Playable Classes Index ---

#[bendpoint(endpoint = "data/wow/playable-class/index" namespace = "static")]
struct PlayableClassesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub classes: Vec<NameAndId>,
}

// --- Playable Class ---

#[bendpoint(endpoint = "data/wow/playable-class/{id}" url_args = "Id" namespace = "static")]
struct PlayableClass {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub gender_name: Option<TypeAndName>,
    pub media: KeyAndId,
}

// --- Playable Class Media ---

#[bendpoint(endpoint = "data/wow/media/playable-class/{id}" url_args = "Id" namespace = "static")]
struct PlayableClassMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

// --- PvP Talent Slots ---

#[derive(Debug, Serialize, Deserialize)]
pub struct PvpTalentSlotEntry {
    pub slot_number: u32,
    pub unlock_player_level: u32,
}

#[bendpoint(endpoint = "data/wow/playable-class/{id}/pvp-talent-slots" url_args = "Id" namespace = "static")]
struct PvpTalentSlots {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub talent_slots: Vec<PvpTalentSlotEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_playable_classes_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "classes": [
                {"key": {"href": "https://test"}, "name": "Warrior", "id": 1}
            ]
        }"#;
        let result: PlayableClassesIndex = json_to_struct(json).unwrap();
        assert_eq!(result.classes.len(), 1);
        assert_eq!(result.classes[0].name, "Warrior");
    }

    #[test]
    fn test_playable_class() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "name": "Warrior",
            "gender_name": {"type": "MALE", "name": "Warrior"},
            "media": {"key": {"href": "https://test"}, "id": 1}
        }"#;
        let result: PlayableClass = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "Warrior");
    }

    #[test]
    fn test_pvp_talent_slots() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "talent_slots": [
                {"slot_number": 1, "unlock_player_level": 20},
                {"slot_number": 2, "unlock_player_level": 30}
            ]
        }"#;
        let result: PvpTalentSlots = json_to_struct(json).unwrap();
        assert_eq!(result.talent_slots.len(), 2);
        assert_eq!(result.talent_slots[0].slot_number, 1);
        assert_eq!(result.talent_slots[0].unlock_player_level, 20);
    }
}
