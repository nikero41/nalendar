use std::io;

use url::Url;

use crate::google::auth::AuthToken;

pub mod auth;

pub struct GoogleClient {
    auth_token: AuthToken,
}

impl GoogleClient {
    pub async fn new() -> io::Result<Self> {
        let auth_token = AuthToken::new().await;
        Ok(Self { auth_token })
    }

    pub async fn get_events(&self) {
        let client = reqwest::Client::new();

        let url =
            Url::parse("https://www.googleapis.com/calendar/v3/users/me/calendarList").unwrap();

        let response = client
            .get(url)
            .bearer_auth(&self.auth_token.access_token)
            .send()
            .await
            .unwrap();
        println!("{} {}: {:?}", "🪚", "response", response);
        let result = response.json::<serde_json::Value>().await.unwrap();
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    }
}
