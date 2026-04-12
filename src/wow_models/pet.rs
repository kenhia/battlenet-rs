use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Pets Index ---

#[bendpoint(endpoint = "data/wow/pet/index" namespace = "static")]
struct PetsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub pets: Vec<NameAndId>,
}

// --- Pet ---

#[bendpoint(endpoint = "data/wow/pet/{id}" url_args = "Id" namespace = "static")]
struct Pet {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub battle_pet_type: TypeAndName,
    pub description: Option<String>,
    pub media: KeyAndId,
}

// --- Pet Media ---

#[bendpoint(endpoint = "data/wow/media/pet/{id}" url_args = "Id" namespace = "static")]
struct PetMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

// --- Pet Abilities Index ---

#[bendpoint(endpoint = "data/wow/pet-ability/index" namespace = "static")]
struct PetAbilitiesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub abilities: Vec<NameAndId>,
}

// --- Pet Ability ---

#[bendpoint(endpoint = "data/wow/pet-ability/{id}" url_args = "Id" namespace = "static")]
struct PetAbility {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub battle_pet_type: TypeAndName,
}

// --- Pet Ability Media ---

#[bendpoint(endpoint = "data/wow/media/pet-ability/{id}" url_args = "Id" namespace = "static")]
struct PetAbilityMedia {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub assets: Vec<Asset>,
    pub id: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_pets_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "pets": [
                {"key": {"href": "https://test"}, "name": "Mechanical Squirrel", "id": 39}
            ]
        }"#;
        let result: PetsIndex = json_to_struct(json).unwrap();
        assert_eq!(result.pets.len(), 1);
        assert_eq!(result.pets[0].name, "Mechanical Squirrel");
    }

    #[test]
    fn test_pet() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 39,
            "name": "Mechanical Squirrel",
            "battle_pet_type": {"type": "MECHANICAL", "name": "Mechanical"},
            "description": "A tiny mechanical squirrel.",
            "media": {"key": {"href": "https://test"}, "id": 39}
        }"#;
        let result: Pet = json_to_struct(json).unwrap();
        assert_eq!(result.id, 39);
        assert_eq!(result.name, "Mechanical Squirrel");
    }

    #[test]
    fn test_pet_ability() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 110,
            "name": "Bite",
            "battle_pet_type": {"type": "BEAST", "name": "Beast"}
        }"#;
        let result: PetAbility = json_to_struct(json).unwrap();
        assert_eq!(result.id, 110);
        assert_eq!(result.name, "Bite");
    }
}
