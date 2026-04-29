use std::{
    fs::{self, File},
    io::Write,
};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::google::auth::oauth::oauth_prompt;

mod oauth;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokenRaw {
    access_token: String,
    expires_in: u64,
    refresh_token: String,
    scope: String,
    token_type: String,
    refresh_token_expires_in: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    pub access_token: String,
    expires_at: DateTime<Utc>,
    refresh_token: String,
    scope: String,
    token_type: String,
    refresh_token_expires_in: Option<u64>,
}

impl From<AuthTokenRaw> for AuthToken {
    fn from(raw: AuthTokenRaw) -> Self {
        let expires_in = Duration::new(raw.expires_in as i64, 0).unwrap();
        Self {
            access_token: raw.access_token,
            expires_at: Utc::now().checked_add_signed(expires_in).unwrap(),
            refresh_token: raw.refresh_token,
            scope: raw.scope,
            token_type: raw.token_type,
            refresh_token_expires_in: raw.refresh_token_expires_in,
        }
    }
}

impl AuthToken {
    pub async fn new() -> Self {
        let credentials = get_credentials();

        match File::open("token.json") {
            Ok(file) => match serde_json::from_reader::<_, AuthToken>(&file) {
                Ok(mut auth_token) => {
                    if auth_token.should_refresh() {
                        auth_token.refresh().await;
                    }
                    auth_token
                }
                Err(_) => {
                    let auth_token = oauth_prompt(&credentials).await;
                    auth_token.save();
                    auth_token
                }
            },
            Err(_) => {
                let auth_token = oauth_prompt(&credentials).await;
                auth_token.save();
                auth_token
            }
        }
    }

    fn save(&self) {
        let mut file = fs::File::create("token.json").expect("Failed to save token file");
        file.write_all(serde_json::to_string(&self).unwrap().as_bytes())
            .expect("Failed to write token file");
    }

    pub fn should_refresh(&self) -> bool {
        self.expires_at
            .signed_duration_since(Utc::now())
            .num_seconds()
            <= 0
    }

    pub async fn refresh(&mut self) {
        let credentials = get_credentials();

        let params = [
            ("grant_type", "refresh_token"),
            ("client_id", &credentials.client_id),
            ("client_secret", &credentials.client_secret),
        ];

        let client = reqwest::Client::new();
        let response = client
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await
            .expect("Failed to exchange auth code");

        match response.json::<AuthTokenRaw>().await {
            Ok(auth_token) => {
                let auth_token = AuthToken::from(auth_token);
                self.access_token = auth_token.access_token;
                self.expires_at = auth_token.expires_at;
                self.refresh_token = auth_token.refresh_token;
                self.scope = auth_token.scope;
                self.token_type = auth_token.token_type;
                self.refresh_token_expires_in = auth_token.refresh_token_expires_in;
                self.save();
            }
            Err(error) => panic!("🪚 error: {:?}", error),
        };
    }
}

fn get_credentials() -> Credentials {
    let file = fs::File::open("credentials.json").expect("Failed to open credentials file");
    let credentials: CredentialsFile =
        serde_json::from_reader(&file).expect("Failed to read credentials file");
    credentials.installed
}

#[derive(Debug, Serialize, Deserialize)]
struct CredentialsFile {
    installed: Credentials,
}

#[derive(Debug, Serialize, Deserialize)]
struct Credentials {
    client_id: String,
    project_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_secret: String,
    redirect_uris: Vec<String>,
}
