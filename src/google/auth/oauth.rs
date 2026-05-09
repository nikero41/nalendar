use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use axum::{Router, extract::Query, routing::get};
use crossterm::{event, style::Stylize};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use url::Url;

const REDIRECT_URI: &str = "http://localhost:8080";

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
    pub refresh_token_expires_in: Option<u64>,
}

pub async fn oauth_prompt(client_id: &str, client_secret: &str) -> AuthTokenResponse {
    let url = {
        let mut url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth")
            .expect("Failed to parse oauth url");
        url.query_pairs_mut()
            .append_pair("client_id", client_id)
            .append_pair("redirect_uri", REDIRECT_URI)
            .append_pair("scope", "https://www.googleapis.com/auth/calendar")
            .append_pair("access_type", "offline")
            .append_pair("response_type", "code");
        url
    };

    println!(
        "Login to your {G}{o}{o2}{g}{l}{e} account",
        G = "G".blue(),
        o = "o".red(),
        o2 = "o".yellow(),
        g = "g".blue(),
        l = "l".green(),
        e = "e".red(),
    );
    println!("Open the following URL in your browser:");
    println!("\n{}\n", url.to_string().blue().underline_blue());
    println!(
        "Or press {} to open the URL in your browser",
        "Enter".green()
    );

    let server_handler = tokio::spawn(listen_for_code());

    let flag = Arc::new(AtomicBool::new(false));
    let flag_check = Arc::clone(&flag);
    tokio::task::spawn_blocking(move || {
        while !flag_check.load(Ordering::Relaxed) {
            if event::poll(Duration::from_millis(100)).is_ok_and(|x| x)
                && let Ok(event::Event::Key(key)) = event::read()
                && key.code == event::KeyCode::Enter
            {
                match open::that(url.to_string()) {
                    Ok(_) => println!("Opened url"),
                    Err(err) => {
                        tracing::error!("Failed to open url: {}", err);
                        println!("Failed to open url")
                    },
                }
            }
        }
    });

    let code = server_handler.await.expect("Failed to listen for code");
    flag.store(true, Ordering::Relaxed);
    exchange_code(client_id, client_secret, &code)
        .await
        .unwrap()
}

#[derive(Debug, Deserialize)]
struct CallbackParams {
    code: String,
}

async fn listen_for_code() -> String {
    let (tx, mut rx) = broadcast::channel(1);

    let server = Router::new().route(
        "/",
        get({
            let tx = tx.clone();
            move |Query(params): Query<CallbackParams>| async move {
                let _ = tx.send(params.code);
                "You can go back to your terminal now. :)"
            }
        }),
    );

    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();

    axum::serve(listener, server)
        .with_graceful_shutdown({
            let mut shutdown_rx = tx.subscribe();
            async move {
                let _ = shutdown_rx.recv().await;
            }
        })
        .await
        .unwrap();

    rx.recv().await.unwrap()
}

async fn exchange_code(
    client_id: &str,
    client_secret: &str,
    code: &str,
) -> Result<AuthTokenResponse, reqwest::Error> {
    let params = [
        ("code", code),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("redirect_uri", REDIRECT_URI),
        ("grant_type", "authorization_code"),
    ];

    let client = reqwest::Client::new();
    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await?;

    response.json().await
}
