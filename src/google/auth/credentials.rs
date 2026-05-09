use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CredentialsFile {
    installed: Credentials,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub client_id: String,
    project_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    pub client_secret: String,
    redirect_uris: Vec<String>,
}

impl Credentials {
    pub fn from_file() -> Self {
        let file = fs::read("credentials.json").expect("Failed to open credentials file");
        let credentials: CredentialsFile =
            serde_json::from_slice(&file).expect("Failed to read credentials file");
        credentials.installed
    }
}
