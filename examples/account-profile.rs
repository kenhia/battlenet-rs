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
        Err(e) => eprintln!("Error fetching account profile: {e}\n\nIf you see an EOF/empty-body error, your user token is likely expired. Re-authenticate via bnauth."),
    }
}
