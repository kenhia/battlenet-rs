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

    // Fetch character mounts collection
    let result: CharacterMountsCollectionSummaryResult =
        client.get_data_with_token(&player, &token).await;
    match result {
        Ok(collection) => {
            println!("Mounts collected: {}", collection.mounts.len());
            for mount in collection.mounts.iter().take(5) {
                println!("  {:?}", mount);
            }
        }
        Err(e) => println!("Error fetching character collections: {:?}", e),
    }
}
