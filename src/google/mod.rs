use std::io;

use crate::google::auth::AuthToken;

pub mod auth;

#[derive(Debug)]
pub struct GoogleClient {
    auth_token: AuthToken,
}

impl GoogleClient {
    pub async fn new() -> io::Result<Self> {
        let auth_token = AuthToken::new().await;
        Ok(Self { auth_token })
    }

    pub async fn get_calendars(&self) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();

        let response = client
            .get("https://www.googleapis.com/calendar/v3/users/me/calendarList")
            .bearer_auth(&self.auth_token.access_token)
            .send()
            .await?;

        let result = response.json::<serde_json::Value>().await.unwrap();
        println!("{}", serde_json::to_string_pretty(&result).unwrap());

        Ok(())
    }
}
