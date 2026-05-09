use std::{fs, io::Write};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

mod credentials;
mod oauth;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    access_token: String,
    expires_at: DateTime<Utc>,
    refresh_token: String,
    scope: String,
    token_type: String,
    refresh_token_expires_in: Option<u64>,
}

impl From<oauth::AuthTokenResponse> for AuthToken {
    fn from(raw: oauth::AuthTokenResponse) -> Self {
        Self {
            access_token: raw.access_token,
            expires_at: calculate_expires_at(raw.expires_in),
            refresh_token: raw.refresh_token,
            scope: raw.scope,
            token_type: raw.token_type,
            refresh_token_expires_in: raw.refresh_token_expires_in,
        }
    }
}

impl AuthToken {
    pub async fn new() -> Self {
        let auth_token = std::fs::read("token.json")
            .ok()
            .and_then(|x| serde_json::from_slice::<AuthToken>(&x).ok());

        if let Some(auth_token) = auth_token {
            return auth_token;
        }

        let auth_token: AuthToken = oauth::oauth_prompt("", "").await.into();
        auth_token.save();
        auth_token
    }

    fn save(&self) {
        let mut file = fs::File::create("token.json").expect("Failed to save token file");
        file.write_all(serde_json::to_string(&self).unwrap().as_bytes())
            .expect("Failed to write token file");
    }

    pub async fn access_token(&mut self) -> &str {
        if self.should_refresh() {
            self.refresh().await;
        }

        &self.access_token
    }

    pub fn should_refresh(&self) -> bool {
        self.expires_at
            .signed_duration_since(Utc::now())
            .num_seconds()
            <= 0
    }

    pub async fn refresh(&mut self) {
        let credentials = credentials::Credentials::from_file();

        let params = [
            ("grant_type", "refresh_token"),
            ("client_id", &credentials.client_id),
            ("client_secret", &credentials.client_secret),
            ("refresh_token", &self.refresh_token),
        ];

        let client = reqwest::Client::new();
        let response = client
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await
            .expect("Failed to exchange auth code");

        match response.json::<RefreshTokenResponse>().await {
            Ok(response) => {
                self.access_token = response.access_token;
                self.expires_at = calculate_expires_at(response.expires_in);
                self.scope = response.scope;
                self.token_type = response.token_type;
                self.save();
            }
            Err(error) => panic!("🪚 error: {:?}", error),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshTokenResponse {
    access_token: String,
    expires_in: u64,
    scope: String,
    token_type: String,
}

fn calculate_expires_at(expires_in: u64) -> DateTime<Utc> {
    let expires_in = Duration::new(expires_in as i64, 0).unwrap();
    Utc::now().checked_add_signed(expires_in).unwrap()
}
