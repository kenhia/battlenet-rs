use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattlenetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{GenerateUrl, UrlArgs};

#[derive(Debug, Deserialize)]
pub struct WowTokenIndex {
    pub price: i64,
    pub last_updated_timestamp: i64,
}

pub type WowTokenIndexResult = Result<WowTokenIndex, BattlenetClientError>;
pub type WowTokenIndexJsonResult = Result<String, BattlenetClientError>;

impl GenerateUrl for WowTokenIndex {
    fn url(client: &BattleNetClient, _: &UrlArgs) -> String {
        let endpoint = "wow/token/index";
        let namespace = WowNamespace::Dynamic.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;

        format!("{base}/data/{endpoint}?namespace={namespace}&locale={locale}")
    }
}
