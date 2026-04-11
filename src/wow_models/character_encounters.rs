use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/encounters" url_args = "Player" namespace = "profile")]
struct CharacterEncountersSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub dungeons: HrefLink,
    pub raids: HrefLink,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/encounters/dungeons" url_args = "Player" namespace = "profile")]
struct CharacterDungeons {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub expansions: Vec<NameAndId>,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/encounters/raids" url_args = "Player" namespace = "profile")]
struct CharacterRaids {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub expansions: Vec<NameAndId>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_encounters_summary() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "dungeons": {"href": "https://test/dungeons"},
            "raids": {"href": "https://test/raids"}
        }"#;
        let result: CharacterEncountersSummary = json_to_struct(json).unwrap();
        assert_eq!(result.dungeons.href, "https://test/dungeons");
        assert_eq!(result.raids.href, "https://test/raids");
    }
}
