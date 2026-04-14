use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/statistics" url_args = "Player" namespace = "profile")]
struct CharacterStatisticsSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub health: u64,
    pub power: u64,
    pub power_type: NameAndId,
    pub strength: StatValue,
    pub agility: StatValue,
    pub intellect: StatValue,
    pub stamina: StatValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatValue {
    pub base: u64,
    pub effective: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_statistics_summary() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "health": 120000,
            "power": 50000,
            "power_type": {"key": {"href": "https://test"}, "name": "Mana", "id": 0},
            "strength": {"base": 100, "effective": 100},
            "agility": {"base": 200, "effective": 200},
            "intellect": {"base": 1500, "effective": 1500},
            "stamina": {"base": 800, "effective": 800}
        }"#;
        let result: CharacterStatisticsSummary = json_to_struct(json).unwrap();
        assert_eq!(result.health, 120000);
        assert_eq!(result.intellect.effective, 1500);
        assert_eq!(result.power_type.name, "Mana");
    }
}
