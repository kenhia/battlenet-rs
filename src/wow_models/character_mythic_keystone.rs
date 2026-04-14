use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/mythic-keystone-profile" url_args = "Player" namespace = "profile")]
struct CharacterMythicKeystoneProfileIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub current_period: Option<CurrentMythicKeystonePeriod>,
    pub seasons: Option<Vec<KeyAndId>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentMythicKeystonePeriod {
    pub period: KeyAndId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MythicKeystoneRun {
    pub completed_timestamp: u64,
    pub duration: u64,
    pub keystone_level: u32,
    pub dungeon: NameAndId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMythicKeystoneSeason {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub season: KeyAndId,
    pub best_runs: Option<Vec<MythicKeystoneRun>>,
}

pub type CharacterMythicKeystoneSeasonResult =
    Result<CharacterMythicKeystoneSeason, BattleNetClientError>;
pub type CharacterMythicKeystoneSeasonJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for CharacterMythicKeystoneSeason {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let (realm_slug, name, extra) = match url_args {
            UrlArgs::PlayerExtra {
                realm_slug,
                name,
                extra,
            } => (realm_slug, name, extra),
            _ => panic!("UrlArgs::PlayerExtra expected"),
        };

        let endpoint = format!(
            "profile/wow/character/{realm_slug}/{name}/mythic-keystone-profile/season/{extra}"
        );
        let namespace = WowNamespace::Profile.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;

        format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_mythic_keystone_profile_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "current_period": {"period": {"key": {"href": "https://test/period"}, "id": 900}},
            "seasons": [
                {"key": {"href": "https://test/season/1"}, "id": 1}
            ]
        }"#;
        let result: CharacterMythicKeystoneProfileIndex = json_to_struct(json).unwrap();
        assert!(result.current_period.is_some());
        assert_eq!(result.current_period.unwrap().period.id, 900);
        assert_eq!(result.seasons.unwrap().len(), 1);
    }
}
