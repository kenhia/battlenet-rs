use crate::namespace::WowNamespace;
use serde::Deserialize;

use crate::client::BattleNetClient;
use crate::errors::BattleNetClientError;
use crate::wow_models::{core_structs::*, GenerateUrl, UrlArgs};

use model_macro::bendpoint;

#[bendpoint(endpoint = "data/wow/connected-realm/{id}/auctions" url_args = "Id" namespace = "dynamic")]
struct AuctionsIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub connected_realm: HrefLink,
    pub auctions: Vec<AuctionEntry>,
    pub commodities: HrefLink,
}

#[derive(Debug, Deserialize)]
pub struct AuctionEntry {
    pub id: u64,
    pub item: AuctionItemRef,
    pub buyout: Option<u64>,
    pub bid: Option<u64>,
    pub quantity: u64,
    pub time_left: String,
}

#[derive(Debug, Deserialize)]
pub struct AuctionItemRef {
    pub id: u64,
    pub context: Option<u64>,
    pub bonus_lists: Option<Vec<u64>>,
    pub modifiers: Option<Vec<AuctionItemModifier>>,
}

#[derive(Debug, Deserialize)]
pub struct AuctionItemModifier {
    #[serde(alias = "type")]
    pub type_: u64,
    pub value: u64,
}

#[bendpoint(endpoint = "data/wow/auctions/commodities" namespace = "dynamic")]
struct CommoditiesIndex {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub auctions: Vec<CommodityEntry>,
}

#[derive(Debug, Deserialize)]
pub struct CommodityEntry {
    pub id: u64,
    pub item: AuctionItemRef,
    pub quantity: u64,
    pub unit_price: u64,
    pub time_left: String,
}
