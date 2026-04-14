use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/collections" url_args = "Player" namespace = "profile")]
struct CharacterCollectionsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub pets: HrefLink,
    pub mounts: HrefLink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterCollectionMount {
    pub mount: NameAndId,
    pub is_useable: Option<bool>,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/collections/mounts" url_args = "Player" namespace = "profile")]
struct CharacterMountsCollectionSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub mounts: Vec<CharacterCollectionMount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterCollectionPet {
    pub species: NameAndId,
    pub level: u32,
    pub quality: TypeAndName,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/collections/pets" url_args = "Player" namespace = "profile")]
struct CharacterPetsCollectionSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub pets: Vec<CharacterCollectionPet>,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/collections/heirlooms" url_args = "Player" namespace = "profile")]
struct CharacterHeirloomsCollectionSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub heirlooms: Vec<HeirloomEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomEntry {
    pub heirloom: NameAndId,
    pub upgrade: HeirloomUpgrade,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeirloomUpgrade {
    pub level: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterCollectionToy {
    pub toy: NameAndId,
}

#[bendpoint(endpoint = "profile/wow/character/{realm_slug}/{name}/collections/toys" url_args = "Player" namespace = "profile")]
struct CharacterToysCollectionSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub toys: Vec<CharacterCollectionToy>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_character_collections_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "pets": {"href": "https://test/pets"},
            "mounts": {"href": "https://test/mounts"}
        }"#;
        let result: CharacterCollectionsIndex = json_to_struct(json).unwrap();
        assert_eq!(result.pets.href, "https://test/pets");
    }

    #[test]
    fn test_character_mounts_collection() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "mounts": [
                {
                    "mount": {"key": {"href": "https://test"}, "name": "Black War Bear", "id": 275},
                    "is_useable": true
                }
            ]
        }"#;
        let result: CharacterMountsCollectionSummary = json_to_struct(json).unwrap();
        assert_eq!(result.mounts.len(), 1);
        assert_eq!(result.mounts[0].mount.name, "Black War Bear");
    }

    #[test]
    fn test_character_pets_collection() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "pets": [
                {
                    "species": {"key": {"href": "https://test"}, "name": "Mechanical Squirrel", "id": 39},
                    "level": 25,
                    "quality": {"type": "RARE", "name": "Rare"}
                }
            ]
        }"#;
        let result: CharacterPetsCollectionSummary = json_to_struct(json).unwrap();
        assert_eq!(result.pets.len(), 1);
        assert_eq!(result.pets[0].species.name, "Mechanical Squirrel");
    }
}
