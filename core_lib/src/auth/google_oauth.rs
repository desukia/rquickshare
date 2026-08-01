use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Serialize, Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,          // User ID único do Google
    pub email: String,
    pub picture: String,
    pub name: String,
}

pub struct GoogleOAuthClient {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    http_client: Client,
}

impl GoogleOAuthClient {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
            http_client: Client::new(),
        }
    }

    pub fn get_auth_url(&self) -> String {
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile",
            self.client_id, self.redirect_uri
        )
    }

    pub async fn exchange_code(&self, code: &str) -> Result<GoogleTokenResponse, Box<dyn std::error::Error>> {
        let response = self.http_client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("code", code),
                ("grant_type", "authorization_code"),
                ("redirect_uri", self.redirect_uri.as_str()),
            ])
            .send()
            .await?;

        let token: GoogleTokenResponse = response.json().await?;
        Ok(token)
    }

    pub async fn get_user_info(&self, access_token: &str) -> Result<GoogleUserInfo, Box<dyn std::error::Error>> {
        let response = self.http_client
            .get("https://openidconnect.googleapis.com/v1/userinfo")
            .bearer_auth(access_token)
            .send()
            .await?;

        let user: GoogleUserInfo = response.json().await?;
        Ok(user)
    }
}
