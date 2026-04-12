use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Guild Crest Components Index (Static) ---

#[bendpoint(endpoint = "data/wow/guild-crest/index" namespace = "static")]
struct GuildCrestComponentsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub emblems: Vec<GuildCrestEmblem>,
    pub borders: Vec<GuildCrestBorder>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestEmblem {
    pub id: u64,
    pub media: KeyAndId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildCrestBorder {
    pub id: u64,
    pub media: KeyAndId,
}

// --- Guild Crest Border Media (Static) ---

#[bendpoint(endpoint = "data/wow/media/guild-crest/border/{id}" url_args = "Id" namespace = "static")]
struct GuildCrestBorderMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

// --- Guild Crest Emblem Media (Static) ---

#[bendpoint(endpoint = "data/wow/media/guild-crest/emblem/{id}" url_args = "Id" namespace = "static")]
struct GuildCrestEmblemMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

// --- Guild (Dynamic) ---

#[bendpoint(endpoint = "data/wow/guild/{realm_slug}/{name_slug}" url_args = "Guild" namespace = "dynamic")]
struct Guild {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u64,
    pub name: String,
    pub faction: TypeAndName,
    pub achievement_points: u64,
    pub member_count: u32,
    pub realm: Realm,
    pub created_timestamp: u64,
}

// --- Guild Activity (Dynamic) ---

#[bendpoint(endpoint = "data/wow/guild/{realm_slug}/{name_slug}/activity" url_args = "Guild" namespace = "dynamic")]
struct GuildActivity {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub guild: NameAndId,
    pub activities: Vec<GuildActivityEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildActivityEntry {
    pub activity: TypeAndName,
    pub timestamp: u64,
}

// --- Guild Achievements (Dynamic) ---

#[bendpoint(endpoint = "data/wow/guild/{realm_slug}/{name_slug}/achievements" url_args = "Guild" namespace = "dynamic")]
struct GuildAchievements {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub guild: NameAndId,
    pub total_quantity: u32,
    pub total_points: u32,
}

// --- Guild Roster (Dynamic) ---

#[bendpoint(endpoint = "data/wow/guild/{realm_slug}/{name_slug}/roster" url_args = "Guild" namespace = "dynamic")]
struct GuildRoster {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub guild: NameAndId,
    pub members: Vec<GuildMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildMember {
    pub character: GuildMemberCharacter,
    pub rank: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildMemberCharacter {
    pub name: String,
    pub id: u64,
    pub realm: Realm,
    pub level: u32,
    pub playable_class: KeyAndId,
    pub playable_race: KeyAndId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_guild_crest_components_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "emblems": [{"id": 1, "media": {"key": {"href": "https://test"}, "id": 1}}],
            "borders": [{"id": 2, "media": {"key": {"href": "https://test"}, "id": 2}}]
        }"#;
        let result: GuildCrestComponentsIndex = json_to_struct(json).unwrap();
        assert_eq!(result.emblems.len(), 1);
        assert_eq!(result.borders.len(), 1);
    }

    #[test]
    fn test_guild() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 123,
            "name": "Test Guild",
            "faction": {"type": "ALLIANCE", "name": "Alliance"},
            "achievement_points": 5000,
            "member_count": 50,
            "realm": {"name": "Proudmoore", "id": 1, "slug": "proudmoore"},
            "created_timestamp": 1234567890
        }"#;
        let result: Guild = json_to_struct(json).unwrap();
        assert_eq!(result.id, 123);
        assert_eq!(result.name, "Test Guild");
        assert_eq!(result.member_count, 50);
    }

    #[test]
    fn test_guild_roster() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "guild": {"key": {"href": "https://test"}, "name": "Test Guild", "id": 1},
            "members": [{
                "character": {
                    "name": "Testchar",
                    "id": 42,
                    "realm": {"name": "Proudmoore", "id": 1, "slug": "proudmoore"},
                    "level": 70,
                    "playable_class": {"key": {"href": "https://test"}, "id": 1},
                    "playable_race": {"key": {"href": "https://test"}, "id": 1}
                },
                "rank": 0
            }]
        }"#;
        let result: GuildRoster = json_to_struct(json).unwrap();
        assert_eq!(result.members.len(), 1);
        assert_eq!(result.members[0].character.name, "Testchar");
        assert_eq!(result.members[0].rank, 0);
    }

    #[test]
    fn test_guild_activity() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "guild": {"key": {"href": "https://test"}, "name": "Test Guild", "id": 1},
            "activities": [{
                "activity": {"type": "ENCOUNTER", "name": "Boss Kill"},
                "timestamp": 1234567890
            }]
        }"#;
        let result: GuildActivity = json_to_struct(json).unwrap();
        assert_eq!(result.activities.len(), 1);
    }
}
