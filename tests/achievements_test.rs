#![cfg(feature = "wow")]

use battlenet_rs::wow_models::prelude::*;

mod common;

#[tokio::test]
async fn achievement_categories_index_test() {
    let client = common::setup_client();

    let result: AchievementCategoriesIndexResult = client.get_data(&UrlArgs::None).await;
    match result {
        Ok(result) => {
            // Not a great test, but as this is data coming from static ns
            // the arrays should generally only increase in size.
            assert!(result.categories.len() >= 148);
            assert!(result.root_categories.len() >= 15);
            assert!(result.guild_categories.len() >= 28);
        }
        Err(e) => {
            panic!("result error {:?}", e);
        }
    }
}

#[tokio::test]
async fn achievement_category_test() {
    let client = common::setup_client();

    let result: AchievementCategoryResult = client.get_data(&UrlArgs::Id { id: 15101 }).await;
    match result {
        Ok(result) => {
            // probably won't do the "all locales" thing in other tests, but
            // figured this was a good chance to play with it a bit.
            match client.locale.as_str() {
                "en_US" => {
                    assert_eq!(result.name, "Darkmoon Faire".to_string());
                }
                "es_MX" => {
                    assert_eq!(result.name, "Feria de la Luna Negra".to_string());
                }
                "pt_BR" => {
                    assert_eq!(result.name, "Feira de Negraluna".to_string());
                }
                "de_DE" => {
                    assert_eq!(result.name, "Dunkelmond-Jahrmarkt".to_string());
                }
                "en_GB" => {
                    assert_eq!(result.name, "Darkmoon Faire".to_string());
                }
                "es_ES" => {
                    assert_eq!(result.name, "Feria de la Luna Negra".to_string());
                }
                "fr_FR" => {
                    assert_eq!(result.name, "Foire de Sombrelune".to_string());
                }
                "it_IT" => {
                    assert_eq!(result.name, "Fiera di Lunacupa".to_string());
                }
                "ru_RU" => {
                    assert_eq!(result.name, "Ярмарка Новолуния".to_string());
                }
                "ko_KR" => {
                    assert_eq!(result.name, "다크문 축제".to_string());
                }
                "zh_TW" => {
                    assert_eq!(result.name, "暗月马戏团".to_string());
                }
                "zh_CN" => {
                    assert_eq!(result.name, "暗月马戏团".to_string());
                }
                _ => {
                    panic!("unexpected locale");
                }
            }
            assert_eq!(result.id, 15101);
            let parent_category = result.parent_category.unwrap();
            assert_eq!(parent_category.id, 155);
        }
        Err(e) => {
            panic!("result error {:?}", e);
        }
    }
}

#[tokio::test]
async fn achievement_index_test() {
    let client = common::setup_client();

    let result: AchievementsIndexResult = client.get_data(&UrlArgs::None).await;
    match result {
        Ok(result) => {
            // Not a great test, but as this is data coming from static ns
            // the array should generally only increase in size.
            assert!(result.achievements.len() >= 6927);
            let lonely = result.achievements.iter().find(|a| a.id == 1291).unwrap();
            assert_eq!(lonely.name, "Lonely?".to_string());
            let timerunner = result.achievements.iter().find(|a| a.id == 40223).unwrap();
            assert_eq!(timerunner.name, "Timerunner".to_string());
        }
        Err(e) => {
            panic!("result error {:?}", e);
        }
    }
}

#[tokio::test]
async fn achievement_test() {
    let client = common::setup_client();

    let result: AchievementResult = client.get_data(&UrlArgs::Id { id: 8 }).await;
    match result {
        Ok(result) => {
            assert_eq!(result.id, 8);
            assert_eq!(result.name, "Level 30".to_string());
        }
        Err(e) => {
            panic!("result error {:?}", e);
        }
    }
}

#[tokio::test]
async fn achievement_media_test() {
    let client = common::setup_client();

    let result: AchievementMediaResult = client.get_data(&UrlArgs::Id { id: 40223 }).await;
    match result {
        Ok(result) => {
            assert_eq!(result.id, 40223);
            assert_eq!(result.assets[0].key, "icon".to_string());
            assert_eq!(result.assets[0].file_data_id, Some(4622478));
        }
        Err(e) => {
            panic!("result error {:?}", e);
        }
    }
}
