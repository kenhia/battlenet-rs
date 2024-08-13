use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;
use battlenet_rs::wow_models::UrlArgs;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

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

    let character_profile_json_result: CharacterProfileJsonResult =
        client.get_json::<CharacterProfile>(&belarsa).await;
    match character_profile_json_result {
        Ok(character_profile_json) => {
            println!("{character_profile_json}");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    let character_profile_result: CharacterProfileResult = client.get_data(&belarsa).await;
    match character_profile_result {
        Ok(character_profile) => {
            println!("{character_profile:?}");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
