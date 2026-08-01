use tauri::State;
use core_lib::auth::google_oauth::GoogleOAuthClient;

#[tauri::command]
pub async fn google_login_url(
    client: State<'_, GoogleOAuthClient>,
) -> Result<String, String> {
    Ok(client.get_auth_url())
}

#[tauri::command]
pub async fn google_exchange_code(
    code: String,
    client: State<'_, GoogleOAuthClient>,
) -> Result<String, String> {
    let token = client
        .exchange_code(&code)
        .await
        .map_err(|e| e.to_string())?;
    Ok(token.access_token)
}

#[tauri::command]
pub async fn get_user_profile(
    access_token: String,
    client: State<'_, GoogleOAuthClient>,
) -> Result<serde_json::Value, String> {
    let user = client
        .get_user_info(&access_token)
        .await
        .map_err(|e| e.to_string())?;
    Ok(serde_json::to_value(user).unwrap())
}
