use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::full_character::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    let realm = "trollbane";
    let name = "belarsa";

    println!("Fetching full character data for {name}@{realm}...");
    match full_character(&client, realm, name, None).await {
        Ok(fc) => {
            println!("\n{} ({})", fc.character_name, fc.realm_slug);
            if let Some(ref profile) = fc.profile {
                println!(
                    "  Level {} {} {}",
                    profile.level, profile.race.name, profile.active_spec.name
                );
            }
            println!("  Fetched at: {}", fc.fetched_at);

            // Count populated fields
            let fields = [
                ("profile", fc.profile.is_some()),
                ("achievements", fc.achievements.is_some()),
                (
                    "achievement_statistics",
                    fc.achievement_statistics.is_some(),
                ),
                ("appearance", fc.appearance.is_some()),
                ("collections", fc.collections.is_some()),
                ("mounts_collection", fc.mounts_collection.is_some()),
                ("pets_collection", fc.pets_collection.is_some()),
                ("heirlooms_collection", fc.heirlooms_collection.is_some()),
                ("toys_collection", fc.toys_collection.is_some()),
                ("encounters", fc.encounters.is_some()),
                ("dungeons", fc.dungeons.is_some()),
                ("raids", fc.raids.is_some()),
                ("equipment", fc.equipment.is_some()),
                ("hunter_pets", fc.hunter_pets.is_some()),
                ("media", fc.media.is_some()),
                (
                    "mythic_keystone_profile",
                    fc.mythic_keystone_profile.is_some(),
                ),
                (
                    "mythic_keystone_season",
                    fc.mythic_keystone_season.is_some(),
                ),
                ("professions", fc.professions.is_some()),
                ("pvp_summary", fc.pvp_summary.is_some()),
                ("pvp_2v2", fc.pvp_2v2.is_some()),
                ("pvp_3v3", fc.pvp_3v3.is_some()),
                ("pvp_rbg", fc.pvp_rbg.is_some()),
                ("quests", fc.quests.is_some()),
                ("completed_quests", fc.completed_quests.is_some()),
                ("reputations", fc.reputations.is_some()),
                ("soulbinds", fc.soulbinds.is_some()),
                ("specializations", fc.specializations.is_some()),
                ("statistics", fc.statistics.is_some()),
                ("titles", fc.titles.is_some()),
            ];
            let populated = fields.iter().filter(|(_, v)| *v).count();
            let total = fields.len();
            println!("\n  Populated: {populated}/{total} endpoints");
            if !fc.errors.is_empty() {
                println!("  Errors ({}):", fc.errors.len());
                for e in &fc.errors {
                    println!("    - {}: {}", e.endpoint, e.message);
                }
            }

            // JSON excerpt
            println!("\n--- JSON excerpt (first 500 chars) ---");
            match full_character_json(&client, realm, name, None).await {
                Ok(json) => {
                    let len = json.len().min(500);
                    println!("{}", &json[..len]);
                    if json.len() > 500 {
                        println!("...(truncated, total {} bytes)", json.len());
                    }
                }
                Err(e) => println!("JSON error: {e}"),
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
