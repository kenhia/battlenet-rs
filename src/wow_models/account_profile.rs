use crate::namespace::WowNamespace;
use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCharacter {
    pub character: HrefLink,
    pub protected_character: Option<HrefLink>,
    pub name: String,
    pub id: u64,
    pub realm: Realm,
    pub playable_class: KeyAndId,
    pub playable_race: KeyAndId,
    pub gender: TypeAndName,
    pub faction: TypeAndName,
    pub level: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WowAccount {
    pub id: u64,
    pub characters: Vec<AccountCharacter>,
}

#[bendpoint(endpoint = "profile/user/wow" namespace = "profile")]
struct AccountProfileSummary {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u64,
    pub wow_accounts: Option<Vec<WowAccount>>,
}

#[bendpoint(endpoint = "profile/user/wow/protected-character/{id1}-{id2}" url_args = "TwoIds" namespace = "profile")]
struct ProtectedCharacterProfile {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u64,
    pub money: u64,
}

#[bendpoint(endpoint = "profile/user/wow/collections" namespace = "profile")]
struct AccountCollectionsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub pets: HrefLink,
    pub mounts: HrefLink,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::json_to_struct;

    #[test]
    fn test_account_profile_summary() {
        let json = r#"{
            "_links": {"self": {"href": "https://test"}},
            "id": 12345,
            "wow_accounts": [
                {
                    "id": 99,
                    "characters": [
                        {
                            "character": {"href": "https://test"},
                            "name": "Belarsa",
                            "id": 1001,
                            "realm": {"name": "Proudmoore", "id": 1, "slug": "proudmoore"},
                            "playable_class": {"key": {"href": "https://test"}, "id": 5},
                            "playable_race": {"key": {"href": "https://test"}, "id": 7},
                            "gender": {"type": "FEMALE", "name": "Female"},
                            "faction": {"type": "ALLIANCE", "name": "Alliance"},
                            "level": 70
                        }
                    ]
                }
            ]
        }"#;
        let result: AccountProfileSummary = json_to_struct(json).unwrap();
        assert_eq!(result.id, 12345);
        let accounts = result.wow_accounts.unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].characters[0].name, "Belarsa");
    }
}
