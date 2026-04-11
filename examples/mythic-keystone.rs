use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch mythic keystone dungeon index
    let dungeons_result: MythicKeystoneDungeonsIndexResult = client.get_data(&UrlArgs::None).await;
    match dungeons_result {
        Ok(dungeons) => {
            println!("Mythic Keystone dungeons: {}", dungeons.dungeons.len());
            for dungeon in dungeons.dungeons.iter().take(5) {
                println!("  {:?}", dungeon);
            }
        }
        Err(e) => println!("Error fetching M+ dungeons: {:?}", e),
    }

    println!();

    // Fetch mythic keystone affixes index
    let affixes_result: MythicKeystoneAffixesIndexResult = client.get_data(&UrlArgs::None).await;
    match affixes_result {
        Ok(affixes) => {
            println!("Mythic Keystone affixes: {}", affixes.affixes.len());
            for affix in &affixes.affixes {
                println!("  {:?}", affix);
            }
        }
        Err(e) => println!("Error fetching M+ affixes: {:?}", e),
    }
}
