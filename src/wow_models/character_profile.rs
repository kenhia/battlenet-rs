use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattlenetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{GenerateUrl, UrlArgs};


#[derive(Debug, Deserialize)]
pub struct CharacterProfileStatus {
    pub id: u64,
    pub is_valid: bool,
}

pub type WowCharacterProfileStatusResult = Result<CharacterProfileStatus, BattlenetClientError>;
pub type WowCharacterProfileStatusJsonResult = Result<String, BattlenetClientError>;

impl GenerateUrl for CharacterProfileStatus {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let (realm_slug, name) = match url_args {
            UrlArgs::Player { realm_slug, name } => (realm_slug, name),
            _ => panic!("UrlArgs::Player expected"),
        };

        let endpoint = format!("profile/wow/character/{realm_slug}/{name}/status");
        let namespace = WowNamespace::Profile.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;

        format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")
    }
}