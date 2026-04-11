use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[derive(Debug, Deserialize)]
pub struct MatchStatistics {
    pub played: u32,
    pub won: u32,
    pub lost: u32,
}

#[derive(Debug, Deserialize)]
pub struct PvpMapStatistic {
    pub world_map: NameAndId,
    pub match_statistics: MatchStatistics,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/pvp-summary" url_args = "Player" namespace = "profile")]
struct CharacterPvpSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub honor_level: u32,
    pub pvp_map_statistics: Option<Vec<PvpMapStatistic>>,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/pvp-bracket/{extra}" url_args = "PlayerExtra" namespace = "profile")]
struct CharacterPvpBracketStatistics {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub bracket: TypeAndName,
    pub rating: u32,
    pub season: KeyAndId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_pvp_summary() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "honor_level": 50,
            "pvp_map_statistics": [
                {
                    "world_map": {"key": {"href": "https://test"}, "name": "Warsong Gulch", "id": 1},
                    "match_statistics": {"played": 100, "won": 60, "lost": 40}
                }
            ]
        }"#;
        let result: CharacterPvpSummary = json_to_struct(json).unwrap();
        assert_eq!(result.honor_level, 50);
        assert!(result.pvp_map_statistics.is_some());
        let stats = result.pvp_map_statistics.unwrap();
        assert_eq!(stats[0].match_statistics.won, 60);
    }
}
