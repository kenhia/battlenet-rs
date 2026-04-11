use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch mounts index
    let mounts_index_result: MountsIndexResult = client.get_data(&UrlArgs::None).await;
    match mounts_index_result {
        Ok(mounts_index) => {
            println!("Total mounts: {}", mounts_index.mounts.len());
            for mount in mounts_index.mounts.iter().take(5) {
                println!("  {:?}", mount);
            }
        }
        Err(e) => println!("Error fetching mounts index: {:?}", e),
    }

    println!();

    // Fetch a specific mount (Horse, id 6)
    let mount_result: MountResult = client.get_data(&UrlArgs::Id { id: 6 }).await;
    match mount_result {
        Ok(mount) => println!("{mount:?}"),
        Err(e) => println!("Error fetching mount: {:?}", e),
    }
}
