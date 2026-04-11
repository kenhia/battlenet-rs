use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch realms index
    let realms_result: RealmsIndexResult = client.get_data(&UrlArgs::None).await;
    match realms_result {
        Ok(realms) => {
            println!("Realms: {}", realms.realms.len());
            for realm in realms.realms.iter().take(5) {
                println!("  {:?}", realm);
            }
        }
        Err(e) => println!("Error fetching realms: {:?}", e),
    }

    println!();

    // Fetch a specific realm (Proudmoore, id 1)
    let realm_result: RealmDataResult = client.get_data(&UrlArgs::Id { id: 1 }).await;
    match realm_result {
        Ok(realm) => println!("{realm:?}"),
        Err(e) => println!("Error fetching realm: {:?}", e),
    }
}
