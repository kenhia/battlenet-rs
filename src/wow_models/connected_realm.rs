use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

#[derive(Debug, Deserialize)]
pub struct ConnectedRealmsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub connected_realms: Vec<HrefLink>,
}

pub type ConnectedRealmsIndexResult = Result<ConnectedRealmsIndex, BattleNetClientError>;
pub type ConnectedRealmsIndexJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for ConnectedRealmsIndex {
    fn url(client: &BattleNetClient, _: &UrlArgs) -> String {
        let endpoint = "data/wow/connected-realm/index";
        let namespace = WowNamespace::Dynamic.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;

        format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")
    }
}

#[derive(Debug, Deserialize)]
pub struct ConnectedRealm {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub has_queue: bool,
    pub status: TypeAndName,
    pub population: TypeAndName,
    pub realms: Vec<RealmLong>,
}

pub type ConnectedRealmResult = Result<ConnectedRealm, BattleNetClientError>;
pub type ConnectedRealmJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for ConnectedRealm {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let id = match url_args {
            UrlArgs::Id { id } => id,
            _ => panic!("UrlArgs::Id expected"),
        };

        let endpoint = format!("data/wow/connected-realm/{id}");
        let namespace = WowNamespace::Dynamic.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;

        format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")
    }
}
