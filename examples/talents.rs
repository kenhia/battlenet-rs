use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch talent tree index
    let talent_result: TalentTreeIndexResult = client.get_data(&UrlArgs::None).await;
    match talent_result {
        Ok(talent_index) => {
            println!("Talent trees: {}", talent_index.spec_talent_trees.len());
            for tree in talent_index.spec_talent_trees.iter().take(5) {
                println!("  {:?}", tree);
            }
        }
        Err(e) => println!("Error fetching talent trees: {:?}", e),
    }
}
