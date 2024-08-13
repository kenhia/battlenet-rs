use serde::Deserialize;

use super::prelude::{HrefLink, LinksRef, TypeAndName, TypeAndValue};

#[derive(Debug, Deserialize)]
pub struct PetItem {
    pub id: u64,
    pub modifiers: Option<Vec<TypeAndName>>,
    pub pet_breed_id: u64,
    pub pet_level: u64,
    pub pet_quality_id: u64,
    pub pet_species_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AuctionItem {
    Pet {
        id: u64,
        modifiers: Option<Vec<TypeAndValue>>,
        pet_breed_id: u64,
        pet_level: u64,
        pet_quality_id: u64,
        pet_species_id: u64,
    },
    Item {
        id: u64,
        context: Option<u64>,
        bonus_lists: Option<Vec<u64>>,
        modifiers: Option<Vec<TypeAndValue>>,
    },
}

#[derive(Debug, Deserialize)]
pub struct Auction {
    pub id: u64,
    pub item: AuctionItem,
    pub buyout: u64,
    pub quantity: u64,
    pub time_left: String,
}

#[derive(Debug, Deserialize)]
pub struct Auctions {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub connected_realm_id: HrefLink,
    pub auctions: Vec<Auction>,
    pub commodities: HrefLink,
}
