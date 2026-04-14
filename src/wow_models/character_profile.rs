use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct CharacterProfileStatus {
//     pub id: u64,
//     pub is_valid: bool,
// }

// pub type CharacterProfileStatusResult = Result<CharacterProfileStatus, BattleNetClientError>;
// pub type CharacterProfileStatusJsonResult = Result<String, BattleNetClientError>;

// impl GenerateUrl for CharacterProfileStatus {
//     fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
//         let (realm_slug, name) = match url_args {
//             UrlArgs::Player { realm_slug, name } => (realm_slug, name),
//             _ => panic!("UrlArgs::Player expected"),
//         };

//         let endpoint = format!("profile/wow/character/{realm_slug}/{name}/status");
//         let namespace = WowNamespace::Profile.to_region_string(&client.region);
//         let base = client.region.base_url();
//         let locale = &client.locale;

//         format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")
//     }
// }

/// status and a unique ID for a character.
///
/// A client should delete information about a character from their application
/// if any of the following conditions occur:
/// - an HTTP 404 Not Found error is returned
/// - the is_valid value is false
/// - the returned character ID doesn't match the previously recorded value for the character
///
/// The following example illustrates how to use this endpoint:
/// A client requests and stores information about a character, including its
/// unique character ID and the timestamp of the request. After 30 days, the
/// client makes a request to the status endpoint to verify if the character
/// information is still valid. If character cannot be found, is not valid, or
/// the characters IDs do not match, the client removes the information from
/// their application.
///
/// If the character is valid and the character IDs match, the client retains
/// the data for another 30 days.
#[bendpoint(
    endpoint = "profile/wow/character/{realm_slug}/{name}/status"
    url_args = "Player"
    namespace = "profile"
)]
struct CharacterProfileStatus {
    id: u64,
    is_valid: bool,
}

/// profile summary for a character.
#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfile {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u64,
    pub name: String,
    pub gender: TypeAndName,
    pub faction: TypeAndName,
    pub race: NameAndId,
    pub character_class: NameAndId,
    pub active_spec: NameAndId,
    pub realm: Realm,
    pub guild: Option<CharacterGuild>,
    pub level: u64,
    pub experience: u64,
    pub achievement_points: u64,
    pub last_login_timestamp: u64,
    pub average_item_level: u64,
    pub equipped_item_level: u64,
    pub active_title: Option<CharacterTitle>,
    pub achievements: HrefLink,
    pub titles: HrefLink,
    pub pvp_summary: HrefLink,
    pub encounters: HrefLink,
    pub media: HrefLink,
    pub specializations: HrefLink,
    pub statistics: HrefLink,
    pub mythic_keystone_profile: HrefLink,
    pub equipment: HrefLink,
    pub appearance: HrefLink,
    pub collections: HrefLink,
    pub reputations: HrefLink,
    pub quests: HrefLink,
    pub achievements_statistics: HrefLink,
    pub professions: HrefLink,
}

pub type CharacterProfileResult = Result<CharacterProfile, BattleNetClientError>;
pub type CharacterProfileJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for CharacterProfile {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let (realm_slug, name) = match url_args {
            UrlArgs::Player { realm_slug, name } => (realm_slug, name),
            _ => panic!("UrlArgs::Player expected"),
        };

        let endpoint = format!("profile/wow/character/{realm_slug}/{name}");
        let namespace = WowNamespace::Profile.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;

        format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")
    }
}
