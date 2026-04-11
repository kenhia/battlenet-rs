use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch a spell (Arcane Intellect, id 1459)
    let spell_result: SpellResult = client.get_data(&UrlArgs::Id { id: 1459 }).await;
    match spell_result {
        Ok(spell) => println!("{spell:?}"),
        Err(e) => println!("Error fetching spell: {:?}", e),
    }
}
