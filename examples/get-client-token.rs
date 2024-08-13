use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;
use chrono::{DateTime, Local, Utc};
use num_format::{Locale, ToFormattedString};

// https://us.api.blizzard.com/data/wow/token/index?namespace=static-us&locale=en_US
// https://us.api.blizzard.com/data/wow/token/index?namespace=dynamic-us&locale=en_US&access_token=<token>

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    let wow_token_index_json_result: WowTokenIndexJsonResult =
        client.get_json::<WowTokenIndex>(&UrlArgs::None).await;
    match wow_token_index_json_result {
        Ok(wow_token_index_json) => {
            println!("{wow_token_index_json}");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    println!();
    let wow_token_index_result: WowTokenIndexResult = client.get_data(&UrlArgs::None).await;
    match wow_token_index_result {
        Ok(wow_token_index) => {
            let dt: DateTime<Utc> =
                DateTime::from_timestamp_millis(wow_token_index.last_updated_timestamp).unwrap();
            let local_dt: DateTime<Local> = dt.into();

            let price_in_gold = wow_token_index.price / 10000;
            let price_formatted = price_in_gold.to_formatted_string(&Locale::en);
            println!("{price_formatted} gold at {local_dt}");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    println!();
    let belarsa = UrlArgs::Player {
        realm_slug: "trollbane".to_string(),
        name: "belarsa".to_string(),
    };
    let character_profile_status_json_result: CharacterProfileStatusJsonResult =
        client.get_json::<CharacterProfileStatus>(&belarsa).await;
    match character_profile_status_json_result {
        Ok(character_profile_status_json) => {
            println!("{character_profile_status_json}");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    println!();
    let character_profile_status_result: CharacterProfileStatusResult =
        client.get_data(&belarsa).await;
    match character_profile_status_result {
        Ok(character_profile_status) => {
            println!("{character_profile_status:?}");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
