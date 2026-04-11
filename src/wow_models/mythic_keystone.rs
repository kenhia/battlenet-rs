use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Mythic Keystone Affixes Index ---

#[bendpoint(endpoint = "data/wow/keystone-affix/index" namespace = "dynamic")]
struct MythicKeystoneAffixesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub affixes: Vec<NameAndId>,
}

// --- Mythic Keystone Affix ---

#[bendpoint(endpoint = "data/wow/keystone-affix/{id}" url_args = "Id" namespace = "dynamic")]
struct MythicKeystoneAffix {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub media: KeyAndId,
}

// --- Mythic Keystone Affix Media ---

#[bendpoint(endpoint = "data/wow/media/keystone-affix/{id}" url_args = "Id" namespace = "dynamic")]
struct MythicKeystoneAffixMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

// --- Mythic Keystone Dungeons Index ---

#[bendpoint(endpoint = "data/wow/mythic-keystone/dungeon/index" namespace = "dynamic")]
struct MythicKeystoneDungeonsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub dungeons: Vec<NameAndId>,
}

// --- Mythic Keystone Dungeon ---

#[bendpoint(endpoint = "data/wow/mythic-keystone/dungeon/{id}" url_args = "Id" namespace = "dynamic")]
struct MythicKeystoneDungeon {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
}

// --- Mythic Keystone Index ---

#[bendpoint(endpoint = "data/wow/mythic-keystone/index" namespace = "dynamic")]
struct MythicKeystoneIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub seasons: HrefLink,
    pub dungeons: HrefLink,
}

// --- Mythic Keystone Periods Index ---

#[bendpoint(endpoint = "data/wow/mythic-keystone/period/index" namespace = "dynamic")]
struct MythicKeystonePeriodsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub periods: Vec<KeyAndId>,
}

// --- Mythic Keystone Period ---

#[bendpoint(endpoint = "data/wow/mythic-keystone/period/{id}" url_args = "Id" namespace = "dynamic")]
struct MythicKeystonePeriod {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
}

// --- Mythic Keystone Seasons Index ---

#[bendpoint(endpoint = "data/wow/mythic-keystone/season/index" namespace = "dynamic")]
struct MythicKeystoneSeasonsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub seasons: Vec<KeyAndId>,
}

// --- Mythic Keystone Season ---

#[bendpoint(endpoint = "data/wow/mythic-keystone/season/{id}" url_args = "Id" namespace = "dynamic")]
struct MythicKeystoneSeason {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_mythic_keystone_affixes_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "affixes": [
                {"key": {"href": "https://test"}, "name": "Fortified", "id": 10}
            ]
        }"#;
        let result: MythicKeystoneAffixesIndex = json_to_struct(json).unwrap();
        assert_eq!(result.affixes.len(), 1);
        assert_eq!(result.affixes[0].name, "Fortified");
    }

    #[test]
    fn test_mythic_keystone_affix() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 10,
            "name": "Fortified",
            "description": "Non-boss enemies have 20% more health.",
            "media": {"key": {"href": "https://test"}, "id": 10}
        }"#;
        let result: MythicKeystoneAffix = json_to_struct(json).unwrap();
        assert_eq!(result.id, 10);
        assert_eq!(result.name, "Fortified");
        assert!(result.description.is_some());
    }

    #[test]
    fn test_mythic_keystone_dungeon() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 375,
            "name": "Mists of Tirna Scithe"
        }"#;
        let result: MythicKeystoneDungeon = json_to_struct(json).unwrap();
        assert_eq!(result.id, 375);
    }

    #[test]
    fn test_mythic_keystone_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "seasons": {"href": "https://us.api.blizzard.com/data/wow/mythic-keystone/season/index"},
            "dungeons": {"href": "https://us.api.blizzard.com/data/wow/mythic-keystone/dungeon/index"}
        }"#;
        let result: MythicKeystoneIndex = json_to_struct(json).unwrap();
        assert!(result.seasons.href.contains("season"));
        assert!(result.dungeons.href.contains("dungeon"));
    }

    #[test]
    fn test_mythic_keystone_period() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 900,
            "start_timestamp": 1700000000000,
            "end_timestamp": 1700604800000
        }"#;
        let result: MythicKeystonePeriod = json_to_struct(json).unwrap();
        assert_eq!(result.id, 900);
    }

    #[test]
    fn test_mythic_keystone_season() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 12,
            "start_timestamp": 1700000000000,
            "end_timestamp": 1700604800000
        }"#;
        let result: MythicKeystoneSeason = json_to_struct(json).unwrap();
        assert_eq!(result.id, 12);
    }
}
