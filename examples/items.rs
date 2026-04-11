use battlenet_rs::client::BattleNetClient;
use battlenet_rs::wow_models::prelude::*;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_filename(".env");
    let client = BattleNetClient::new_from_environment();

    // Fetch item classes index
    let classes_result: ItemClassesIndexResult = client.get_data(&UrlArgs::None).await;
    match classes_result {
        Ok(classes) => {
            println!("Item classes: {}", classes.item_classes.len());
            for class in &classes.item_classes {
                println!("  {:?}", class);
            }
        }
        Err(e) => println!("Error fetching item classes: {:?}", e),
    }

    println!();

    // Fetch a specific item (Thunderfury, id 19019)
    let item_result: ItemResult = client.get_data(&UrlArgs::Id { id: 19019 }).await;
    match item_result {
        Ok(item) => println!("{item:?}"),
        Err(e) => println!("Error fetching item: {:?}", e),
    }
}
