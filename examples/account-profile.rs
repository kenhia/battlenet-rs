use std::env;

use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();
    let token = env::var("USER_TOKEN").expect("USER_TOKEN env var required");

    // Fetch account profile summary
    let result: AccountProfileSummaryResult =
        client.get_data_with_token(&UrlArgs::None, &token).await;
    match result {
        Ok(profile) => println!("{profile:?}"),
        Err(e) => println!("Error fetching account profile: {:?}", e),
    }
}
