use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch creature families index
    let families_result: CreatureFamiliesIndexResult = client.get_data(&UrlArgs::None).await;
    match families_result {
        Ok(families) => {
            println!("Creature families: {}", families.creature_families.len());
            for family in families.creature_families.iter().take(5) {
                println!("  {:?}", family);
            }
        }
        Err(e) => println!("Error fetching creature families: {:?}", e),
    }

    println!();

    // Fetch creature types index
    let types_result: CreatureTypesIndexResult = client.get_data(&UrlArgs::None).await;
    match types_result {
        Ok(types) => {
            println!("Creature types: {}", types.creature_types.len());
            for t in &types.creature_types {
                println!("  {:?}", t);
            }
        }
        Err(e) => println!("Error fetching creature types: {:?}", e),
    }
}
