use serde::{Deserialize, Serialize};

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::namespace::WowNamespace;
use crate::wow_models::{GenerateUrl, UrlArgs};

use model_macro::bendpoint;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct WowTokenIndex {
//     pub price: i64,
//     pub last_updated_timestamp: i64,
// }

// pub type WowTokenIndexResult = Result<WowTokenIndex, BattleNetClientError>;
// pub type WowTokenIndexJsonResult = Result<String, BattleNetClientError>;

// impl GenerateUrl for WowTokenIndex {
//     fn url(client: &BattleNetClient, _: &UrlArgs) -> String {
//         let endpoint = "data/wow/token/index";
//         let namespace = WowNamespace::Dynamic.to_region_string(&client.region);
//         let base = client.region.base_url();
//         let locale = &client.locale;

//         format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")
//     }
// }

#[bendpoint(endpoint = "data/wow/token/index" namespace = "dynamic")]
pub struct WowTokenIndex {
    pub price: i64,
    pub last_updated_timestamp: i64,
}
