use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch professions index
    let professions_result: ProfessionsIndexResult = client.get_data(&UrlArgs::None).await;
    match professions_result {
        Ok(professions) => {
            println!("Professions: {}", professions.professions.len());
            for prof in &professions.professions {
                println!("  {:?}", prof);
            }
        }
        Err(e) => println!("Error fetching professions: {:?}", e),
    }

    println!();

    // Fetch a specific profession (Blacksmithing, id 164)
    let prof_result: ProfessionResult = client.get_data(&UrlArgs::Id { id: 164 }).await;
    match prof_result {
        Ok(prof) => println!("{prof:?}"),
        Err(e) => println!("Error fetching profession: {:?}", e),
    }
}
