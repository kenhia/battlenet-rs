use battlenet_rs::wow_models::prelude::*;

mod common;

#[tokio::test]
async fn connected_realm_index_test() {
    let client = common::setup_client();

    let result: ConnectedRealmsIndexResult = client.get_data(&UrlArgs::None).await;
    match result {
        Ok(result) => {
            // Not a great test, but as this is data coming from static ns
            // the arrays should generally only increase in size.
            assert!(result.connected_realms.len() >= 83);
        }
        Err(e) => {
            panic!("result error {:?}", e);
        }
    }
}

// My primary connected realm is id:1175 including
// slugs: trollbane, grizzly-hills, malfurion, lothar, kaelthas, gnomeregan,
//        moonrunner, and ghostlands...so I'll use these for the test.
#[tokio::test]
async fn connected_realm_test() {
    let client = common::setup_client();

    let result: ConnectedRealmResult = client.get_data(&UrlArgs::Id { id: 1175 }).await;
    match result {
        Ok(result) => {
            let trollbane = result.realms.iter().find(|realm| realm.id == 1175).unwrap();
            let malfurion = result
                .realms
                .iter()
                .find(|realm| realm.slug == "malfurion")
                .unwrap();
            let grizzly_hills = result
                .realms
                .iter()
                .find(|realm| realm.name == "Grizzly Hills")
                .unwrap();
            assert_eq!(trollbane.name, "Trollbane");
            assert_eq!(trollbane.slug, "trollbane");
            assert_eq!(trollbane.locale, "enUS".to_string());
            assert_eq!(trollbane.timezone, "America/New_York".to_string());
            assert_eq!(malfurion.id, 1132);
            assert_eq!(grizzly_hills.id, 1579);
        }
        Err(e) => {
            panic!("result error {:?}", e);
        }
    }
}
