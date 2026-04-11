use std::env;

use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();
    let token = env::var("USER_TOKEN").expect("USER_TOKEN env var required");

    let player = UrlArgs::Player {
        realm_slug: "trollbane".to_string(),
        name: "belarsa".to_string(),
    };

    // Fetch character equipment
    let result: CharacterEquipmentSummaryResult = client.get_data_with_token(&player, &token).await;
    match result {
        Ok(equipment) => println!("{equipment:?}"),
        Err(e) => println!("Error fetching character equipment: {:?}", e),
    }
}
