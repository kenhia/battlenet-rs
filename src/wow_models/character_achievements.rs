use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/achievements" url_args = "Player" namespace = "profile")]
struct CharacterAchievementsSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub total_quantity: u32,
    pub total_points: u32,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/achievements/statistics" url_args = "Player" namespace = "profile")]
struct CharacterAchievementStatistics {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub categories: Vec<NameAndId>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_achievements_summary() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "total_quantity": 1500,
            "total_points": 25000
        }"#;
        let result: CharacterAchievementsSummary = json_to_struct(json).unwrap();
        assert_eq!(result.total_quantity, 1500);
        assert_eq!(result.total_points, 25000);
    }
}
