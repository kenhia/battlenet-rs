use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch PvP seasons index
    let seasons_result: PvpSeasonsIndexResult = client.get_data(&UrlArgs::None).await;
    match seasons_result {
        Ok(seasons) => {
            println!("PvP seasons: {}", seasons.seasons.len());
            for season in seasons.seasons.iter().take(5) {
                println!("  {:?}", season);
            }
        }
        Err(e) => println!("Error fetching PvP seasons: {:?}", e),
    }

    println!();

    // Fetch a specific PvP season (id 33)
    let season_result: PvpSeasonResult = client.get_data(&UrlArgs::Id { id: 33 }).await;
    match season_result {
        Ok(season) => println!("{season:?}"),
        Err(e) => println!("Error fetching PvP season: {:?}", e),
    }
}
