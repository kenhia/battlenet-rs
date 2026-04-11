use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch pets index
    let pets_result: PetsIndexResult = client.get_data(&UrlArgs::None).await;
    match pets_result {
        Ok(pets) => {
            println!("Pets: {}", pets.pets.len());
            for pet in pets.pets.iter().take(5) {
                println!("  {:?}", pet);
            }
        }
        Err(e) => println!("Error fetching pets: {:?}", e),
    }
}
