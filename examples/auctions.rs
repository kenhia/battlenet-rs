use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch auctions for connected realm 1175
    let auctions_result: AuctionsIndexResult = client.get_data(&UrlArgs::Id { id: 1175 }).await;
    match auctions_result {
        Ok(auctions) => {
            println!("Auctions: {}", auctions.auctions.len());
            for auction in auctions.auctions.iter().take(5) {
                println!("  {:?}", auction);
            }
        }
        Err(e) => println!("Error fetching auctions: {:?}", e),
    }
}
