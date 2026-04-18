mod commands;
mod oauth;
mod state;

use battlenet_rs::cache::cached_client::CachedClient;
use battlenet_rs::cache::sqlite::SqliteCacheStore;
use battlenet_rs::client::BattleNetClient;
use state::AppState;
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_oauth::init())
        .setup(|app| {
            use tauri::Manager;

            let client = BattleNetClient::new_from_environment();

            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");

            let db_path = app_data_dir.join("cache.db");
            let db_url = format!("sqlite:{}", db_path.display());

            let rt = tokio::runtime::Handle::current();
            let store = rt
                .block_on(SqliteCacheStore::new(&db_url))
                .expect("failed to create SQLite cache store");
            let cached_client = rt
                .block_on(CachedClient::new(client, store))
                .expect("failed to create cached client");

            app.manage(AppState {
                client: cached_client,
                user_token: Mutex::new(None),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_realms,
            commands::lookup_character,
            commands::login,
            commands::get_character,
            commands::refresh_character
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
