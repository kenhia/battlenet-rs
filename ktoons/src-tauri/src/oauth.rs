use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use tauri_plugin_oauth::OauthConfig;

/// Static mapping of WoW playable class IDs to names.
fn class_name(id: u64) -> &'static str {
    match id {
        1 => "Warrior",
        2 => "Paladin",
        3 => "Hunter",
        4 => "Rogue",
        5 => "Priest",
        6 => "Death Knight",
        7 => "Shaman",
        8 => "Mage",
        9 => "Warlock",
        10 => "Monk",
        11 => "Druid",
        12 => "Demon Hunter",
        13 => "Evoker",
        _ => "Unknown",
    }
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    scope: String,
}

/// Exchange an authorization code for an access token.
pub async fn exchange_code(
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> Result<crate::state::UserToken, String> {
    let http = Client::new();
    let resp = http
        .post("https://oauth.battle.net/token")
        .basic_auth(client_id, Some(client_secret))
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", redirect_uri),
        ])
        .send()
        .await
        .map_err(|e| format!("Token exchange request failed: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Token exchange failed ({status}): {body}"));
    }

    let token_resp: TokenResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse token response: {e}"))?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    Ok(crate::state::UserToken {
        access_token: token_resp.access_token,
        token_type: token_resp.token_type,
        expires_at: now + token_resp.expires_in as i64,
        scope: token_resp.scope,
        obtained_at: now,
    })
}

/// Start the OAuth local server and return (port, oneshot receiver for the callback URL).
pub fn start_oauth_listener() -> Result<(u16, tokio::sync::oneshot::Receiver<String>), String> {
    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    let tx = std::sync::Mutex::new(Some(tx));

    let config = OauthConfig {
        ports: Some(vec![5055]),
        response: Some(
            "Authentication complete! You can close this tab and return to ktoons.".into(),
        ),
    };

    let port = tauri_plugin_oauth::start_with_config(config, move |url| {
        if let Some(sender) = tx.lock().unwrap().take() {
            let _ = sender.send(url);
        }
    })
    .map_err(|e| format!("Failed to start OAuth listener: {e}"))?;

    Ok((port, rx))
}

/// Build the Blizzard authorization URL.
pub fn build_authorize_url(client_id: &str, redirect_uri: &str, state: &str) -> String {
    format!(
        "https://oauth.battle.net/authorize?client_id={}&redirect_uri={}&response_type=code&scope=wow.profile&state={}",
        client_id,
        urlencoding::encode(redirect_uri),
        state
    )
}

/// Parse the callback URL to extract the authorization code and state.
pub fn parse_callback_url(url: &str) -> Result<(String, String), String> {
    let parsed = url::Url::parse(url).map_err(|e| format!("Invalid callback URL: {e}"))?;
    let params: HashMap<String, String> = parsed.query_pairs().into_owned().collect();

    let code = params
        .get("code")
        .ok_or_else(|| {
            let error = params.get("error").cloned().unwrap_or_default();
            let desc = params.get("error_description").cloned().unwrap_or_default();
            format!("OAuth error: {error} — {desc}")
        })?
        .clone();

    let state = params.get("state").cloned().unwrap_or_default();

    Ok((code, state))
}

/// Map an AccountCharacter to an AccountCharacterEntry with class name resolved.
pub fn map_account_character(
    ac: &battlenet_rs::wow_models::prelude::AccountCharacter,
) -> crate::commands::AccountCharacterEntry {
    crate::commands::AccountCharacterEntry {
        name: ac.name.clone(),
        realm_name: ac.realm.name.clone(),
        realm_slug: ac.realm.slug.clone(),
        level: ac.level,
        class_name: class_name(ac.playable_class.id).to_string(),
        faction: ac.faction.name.clone(),
        id: ac.id,
    }
}
