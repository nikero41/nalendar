use crate::google::auth::AuthToken;

mod auth;
mod calendars;

#[derive(Debug)]
pub struct GoogleClient {
    auth_token: AuthToken,
}

impl GoogleClient {
    pub async fn new() -> Self {
        let auth_token = AuthToken::new().await;
        Self { auth_token }
    }

    pub async fn setup(&mut self) {
        if self.auth_token.should_refresh() {
            self.auth_token.refresh().await;
        }
    }
}
