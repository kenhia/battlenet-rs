use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[bendpoint(endpoint = "data/wow/connected-realm/index" namespace = "dynamic")]
struct ConnectedRealmsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub connected_realms: Vec<HrefLink>,
}

#[bendpoint(endpoint = "data/wow/connected-realm/{id}" url_args = "Id" namespace = "dynamic")]
struct ConnectedRealm {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub has_queue: bool,
    pub status: TypeAndName,
    pub population: TypeAndName,
    pub realms: Vec<RealmLong>,
}

// --- Connected Realm Search ---

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectedRealmSearchData {
    pub id: u32,
    pub has_queue: bool,
    pub population: TypeAndName,
    pub realms: Vec<RealmLong>,
}

pub type ConnectedRealmSearchResult =
    Result<SearchResult<ConnectedRealmSearchData>, BattleNetClientError>;
pub type ConnectedRealmSearchJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for SearchResult<ConnectedRealmSearchData> {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let params = match url_args {
            UrlArgs::Search { params } => params,
            _ => panic!("UrlArgs::Search expected"),
        };
        let endpoint = "data/wow/search/connected-realm";
        let namespace = WowNamespace::Dynamic.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;
        let mut url = format!("{base}/{endpoint}?namespace={namespace}&locale={locale}");
        for (key, value) in params {
            url.push_str(&format!("&{key}={value}"));
        }
        url
    }
}
