use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch achievement categories index
    let categories_result: AchievementCategoriesIndexResult = client.get_data(&UrlArgs::None).await;
    match categories_result {
        Ok(categories) => {
            println!("Achievement categories: {}", categories.categories.len());
            for cat in categories.categories.iter().take(5) {
                println!("  {:?}", cat);
            }
        }
        Err(e) => println!("Error fetching achievement categories: {:?}", e),
    }

    println!();

    // Fetch a specific achievement (id 6)
    let achievement_result: AchievementResult = client.get_data(&UrlArgs::Id { id: 6 }).await;
    match achievement_result {
        Ok(achievement) => println!("{achievement:?}"),
        Err(e) => println!("Error fetching achievement: {:?}", e),
    }
}
