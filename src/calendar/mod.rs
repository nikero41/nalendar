use serde::{Deserialize, Serialize};
use std::{fs, io};

pub struct Client {
    credentials: Credentials,
    auth_token: String,
}

impl Client {
    pub async fn new() -> io::Result<Self> {
        let file = fs::File::open("credentials.json")?;
        let credentials: CredentialsFile =
            serde_json::from_reader(&file).expect("Failed to read credentials file");

        Ok(Self {
            credentials: credentials.installed,
            auth_token: String::from(""),
        })
    }

    pub fn get_events(&self) {
        let client = reqwest::Client::new();
    }
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
