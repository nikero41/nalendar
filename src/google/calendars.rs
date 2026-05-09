use color_eyre::eyre::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CalendarListResponse {
    items: Vec<Calendar>,
}

#[derive(Debug, Deserialize)]
pub struct Calendar {
    id: String,
    summary: String,
    background_color: String,
}

impl super::GoogleClient {
    pub async fn get_calendars(&mut self) -> Result<Vec<Calendar>> {
        let client = reqwest::Client::new();

        let response = client
            .get("https://www.googleapis.com/calendar/v3/users/me/calendarList")
            .bearer_auth(self.auth_token.access_token().await)
            .send()
            .await?;

        let result: CalendarListResponse = response
            .json()
            .await
            .context("Failed to parse calendar response")?;

        Ok(result.items)
    }
}
