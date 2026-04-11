use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// --- Covenant Index ---

#[bendpoint(endpoint = "data/wow/covenant/index" namespace = "static")]
struct CovenantIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub covenants: Vec<NameAndId>,
}

// --- Covenant ---

#[bendpoint(endpoint = "data/wow/covenant/{id}" url_args = "Id" namespace = "static")]
struct Covenant {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub media: KeyAndId,
}

// --- Soulbind Index ---

#[bendpoint(endpoint = "data/wow/covenant/soulbind/index" namespace = "static")]
struct SoulbindIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub soulbinds: Vec<NameAndId>,
}

// --- Soulbind ---

#[bendpoint(endpoint = "data/wow/covenant/soulbind/{id}" url_args = "Id" namespace = "static")]
struct Soulbind {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub covenant: NameAndId,
}

// --- Conduit Index ---

#[bendpoint(endpoint = "data/wow/covenant/conduit/index" namespace = "static")]
struct ConduitIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub conduits: Vec<NameAndId>,
}

// --- Conduit ---

#[bendpoint(endpoint = "data/wow/covenant/conduit/{id}" url_args = "Id" namespace = "static")]
struct Conduit {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub socket_type: Option<TypeAndName>,
}

// --- Covenant Media ---

#[bendpoint(endpoint = "data/wow/media/covenant/{id}" url_args = "Id" namespace = "static")]
struct CovenantMedia {
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
    fn test_covenant_index() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "covenants": [
                {"key": {"href": "https://test"}, "name": "Kyrian", "id": 1}
            ]
        }"#;
        let result: CovenantIndex = json_to_struct(json).unwrap();
        assert_eq!(result.covenants.len(), 1);
        assert_eq!(result.covenants[0].name, "Kyrian");
    }

    #[test]
    fn test_covenant() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "name": "Kyrian",
            "description": "The Kyrian are the guardians of the afterlife.",
            "media": {"key": {"href": "https://test"}, "id": 1}
        }"#;
        let result: Covenant = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "Kyrian");
    }

    #[test]
    fn test_soulbind() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 1,
            "name": "Pelagos",
            "covenant": {"key": {"href": "https://test"}, "name": "Kyrian", "id": 1}
        }"#;
        let result: Soulbind = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "Pelagos");
    }

    #[test]
    fn test_conduit() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 10,
            "name": "Focused Light",
            "socket_type": {"type": "Potency", "name": "Potency"}
        }"#;
        let result: Conduit = json_to_struct(json).unwrap();
        assert_eq!(result.id, 10);
        assert_eq!(result.name, "Focused Light");
    }

    #[test]
    fn test_covenant_media() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "assets": [{"key": "icon", "value": "https://test/icon.jpg", "file_data_id": 123}],
            "id": 1
        }"#;
        let result: CovenantMedia = json_to_struct(json).unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.assets.len(), 1);
    }
}
