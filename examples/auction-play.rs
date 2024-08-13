use serde::Deserialize;

use battlenet_rs::wow_models::prelude::*;

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
    pub connected_realm: HrefLink,
    pub auctions: Vec<Auction>,
    pub commodities: HrefLink,
}

/*
Pet and Item are the two types of items that can be auctioned in World of Warcraft.
Examples of each are shown below. Several of the fields may be omitted.
        {
            "id": 327853898,
            "item": {
                "id": 82800,
                "modifiers": [
                    {
                        "type": 6,
                        "value": 106659
                    }
                ],
                "pet_breed_id": 17,
                "pet_level": 1,
                "pet_quality_id": 3,
                "pet_species_id": 3319
            },
            "buyout": 19990000,
            "quantity": 1,
            "time_left": "SHORT"
        },
        {
            "id": 327854618,
            "item": {
                "id": 8184,
                "bonus_lists": [
                    6655
                ],
                "modifiers": [
                    {
                        "type": 9,
                        "value": 27
                    },
                    {
                        "type": 28,
                        "value": 18
                    }
                ]
            },
            "buyout": 500000,
            "quantity": 1,
            "time_left": "SHORT"
        },

*/


fn main() {
    let file = std::fs::File::open("./data/auctions-small.json").unwrap();
    let data = serde_json::from_reader::<_, Auctions>(file).unwrap();
    let mut pet_count = 0;
    let mut item_count = 0;
    for auction in data.auctions {
        match auction.item {
            AuctionItem::Pet {
                id: _,
                modifiers: _,
                pet_breed_id: _,
                pet_level: _,
                pet_quality_id: _,
                pet_species_id: _,
            } => {
                pet_count += 1;
            }
            AuctionItem::Item {
                id: _,
                context: _,
                bonus_lists: _,
                modifiers: _,
            } => {
                item_count += 1;
            }
        }
    }
    println!("Pet count: {}", pet_count);
    println!("Item count: {}", item_count);
    // for auction in data.auctions {
    //     match auction.item {
    //         AuctionItem::Pet {
    //             id,
    //             modifiers: _,
    //             pet_breed_id,
    //             pet_level,
    //             pet_quality_id,
    //             pet_species_id,
    //         } => {
    //             println!(
    //                 "Pet: id: {}, breed: {}, level: {}, quality: {}, species: {}",
    //                 id, pet_breed_id, pet_level, pet_quality_id, pet_species_id
    //             );
    //         }
    //         AuctionItem::Item {
    //             id,
    //             context: _,
    //             bonus_lists: _,
    //             modifiers: _,
    //         } => {
    //             println!("Item: id: {}", id);
    //         }
    //     }
    // }
}
