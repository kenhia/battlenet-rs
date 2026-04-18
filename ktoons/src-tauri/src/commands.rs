use serde::Serialize;
use tauri::State;

use battlenet_rs::wow_models::full_character::{full_character, full_character_force};
use battlenet_rs::wow_models::prelude::*;

use crate::oauth;
use crate::state::AppState;

/// Realm entry returned to the frontend for the dropdown.
#[derive(Debug, Serialize)]
pub struct RealmEntry {
    pub name: String,
    pub slug: String,
}

/// Account character entry returned by the login command.
#[derive(Debug, Serialize)]
pub struct AccountCharacterEntry {
    pub name: String,
    pub realm_name: String,
    pub realm_slug: String,
    pub level: u32,
    pub class_name: String,
    pub faction: String,
    pub id: u64,
}

/// Fetch the list of available realms for the configured region.
#[tauri::command]
pub async fn get_realms(state: State<'_, AppState>) -> Result<Vec<RealmEntry>, String> {
    let result: SearchResult<RealmSearchData> = state
        .client
        .get_data(&UrlArgs::Search {
            params: vec![
                ("orderby".to_string(), "name".to_string()),
                ("_pageSize".to_string(), "1000".to_string()),
            ],
        })
        .await
        .map_err(|e| format!("Failed to fetch realms: {e}"))?;

    let realms: Vec<RealmEntry> = result
        .results
        .into_iter()
        .map(|entry| RealmEntry {
            name: entry.data.name.name,
            slug: entry.data.slug,
        })
        .collect();

    Ok(realms)
}

/// Look up a character by name and realm using client token only (no OAuth).
#[tauri::command]
pub async fn lookup_character(
    state: State<'_, AppState>,
    realm_slug: String,
    character_name: String,
) -> Result<serde_json::Value, String> {
    let fc = full_character(
        &state.client,
        &realm_slug,
        &character_name.to_lowercase(),
        None,
    )
    .await
    .map_err(|e| format!("Failed to fetch character: {e}"))?;

    serde_json::to_value(&fc).map_err(|e| format!("Failed to serialize character: {e}"))
}

/// Start OAuth flow: open browser, wait for callback, exchange code, fetch account characters.
#[tauri::command]
pub async fn login(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<Vec<AccountCharacterEntry>, String> {
    let client_id = std::env::var("BATTLE_NET_CLIENT_ID")
        .map_err(|_| "BATTLE_NET_CLIENT_ID not set".to_string())?;
    let client_secret = std::env::var("BATTLE_NET_CLIENT_SECRET")
        .map_err(|_| "BATTLE_NET_CLIENT_SECRET not set".to_string())?;

    // Start local OAuth listener
    let (port, rx) = oauth::start_oauth_listener()?;
    let redirect_uri = format!("http://127.0.0.1:{port}");

    // Generate CSRF state
    let csrf_state = format!("{:x}", rand::random::<u64>());
    let authorize_url = oauth::build_authorize_url(&client_id, &redirect_uri, &csrf_state);

    // Open browser
    use tauri_plugin_shell::ShellExt;
    #[allow(deprecated)]
    app.shell()
        .open(&authorize_url, None)
        .map_err(|e| format!("Failed to open browser: {e}"))?;

    // Wait for callback
    let callback_url = rx
        .await
        .map_err(|_| "OAuth callback was not received".to_string())?;

    // Parse callback
    let (code, _returned_state) = oauth::parse_callback_url(&callback_url)?;

    // Exchange code for token
    let user_token = oauth::exchange_code(&client_id, &client_secret, &code, &redirect_uri).await?;

    // Store token
    let token_str = user_token.access_token.clone();
    {
        let mut guard = state.user_token.lock().await;
        *guard = Some(user_token);
    }

    // Fetch account profile
    let profile: AccountProfileSummary = state
        .client
        .get_data_with_token::<AccountProfileSummary>(&UrlArgs::None, &token_str)
        .await
        .map_err(|e| format!("Failed to fetch account profile: {e}"))?;

    // Map characters
    let mut entries: Vec<AccountCharacterEntry> = Vec::new();
    if let Some(wow_accounts) = profile.wow_accounts {
        for account in wow_accounts {
            for character in &account.characters {
                entries.push(oauth::map_account_character(character));
            }
        }
    }

    // Sort by realm, then name
    entries.sort_by(|a, b| a.realm_name.cmp(&b.realm_name).then(a.name.cmp(&b.name)));

    Ok(entries)
}

/// Get a character using user token if available (for user-scoped data).
#[tauri::command]
pub async fn get_character(
    state: State<'_, AppState>,
    realm_slug: String,
    character_name: String,
) -> Result<serde_json::Value, String> {
    let token = {
        let guard = state.user_token.lock().await;
        guard.as_ref().map(|t| t.access_token.clone())
    };

    let fc = full_character(
        &state.client,
        &realm_slug,
        &character_name.to_lowercase(),
        token.as_deref(),
    )
    .await
    .map_err(|e| format!("Failed to fetch character: {e}"))?;

    serde_json::to_value(&fc).map_err(|e| format!("Failed to serialize character: {e}"))
}

/// Force re-fetch character data from API, bypassing cache.
#[tauri::command]
pub async fn refresh_character(
    state: State<'_, AppState>,
    realm_slug: String,
    character_name: String,
) -> Result<serde_json::Value, String> {
    let token = {
        let guard = state.user_token.lock().await;
        guard.as_ref().map(|t| t.access_token.clone())
    };

    let fc = full_character_force(
        &state.client,
        &realm_slug,
        &character_name.to_lowercase(),
        token.as_deref(),
    )
    .await
    .map_err(|e| format!("Failed to refresh character: {e}"))?;

    serde_json::to_value(&fc).map_err(|e| format!("Failed to serialize character: {e}"))
}
