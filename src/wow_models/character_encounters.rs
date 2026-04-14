use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

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
    pub expansions: Vec<EncounterExpansion>,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/encounters/raids" url_args = "Player" namespace = "profile")]
struct CharacterRaids {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub expansions: Vec<EncounterExpansion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterExpansion {
    pub expansion: NameAndId,
    pub instances: Vec<EncounterInstance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterInstance {
    pub instance: NameAndId,
    pub modes: Vec<EncounterMode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterMode {
    pub difficulty: EncounterDifficulty,
    pub status: TypeAndName,
    pub progress: EncounterProgress,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterDifficulty {
    #[serde(alias = "type", default)]
    pub type_: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterProgress {
    pub completed_count: u32,
    pub total_count: u32,
    pub encounters: Vec<EncounterDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterDetail {
    pub encounter: NameAndId,
    pub completed_count: u32,
    pub last_kill_timestamp: u64,
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
