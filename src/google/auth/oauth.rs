use std::io::{self, Write};

use axum::{Router, extract::Query, routing::get};
use crossterm::{ExecutableCommand, cursor, event, style::Stylize, terminal};
use serde::Deserialize;
use tokio::sync::broadcast;
use url::Url;

use crate::google::auth::{AuthToken, Credentials};

const REDIRECT_URI: &str = "http://localhost:8080";

pub async fn oauth_prompt(credentials: &Credentials) -> AuthToken {
    let url = {
        let mut url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap();
        url.query_pairs_mut()
            .append_pair("client_id", &credentials.client_id)
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
    let mut stdout = io::stdout();
    // let _ = stdout.execute(cursor::MoveTo(0, 0));
    let _ = stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown));
    let _ = stdout.flush();

    let server_handler = tokio::spawn(listen_for_code());
    let open_browser_handler = tokio::spawn(async move {
        loop {
            if let Ok(event::Event::Key(key)) = event::read()
                && key.code == event::KeyCode::Enter
            {
                open::that(url.to_string()).unwrap();
            }
        }
    });

    match server_handler.await {
        Ok(code) => {
            
            open_browser_handler.abort();
            exchange_code(credentials, &code).await.unwrap()
        }
        Err(_) => panic!("Failed to listen for code"),
    }
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
            async move |Query(params): Query<CallbackParams>| -> String {
                let _ = tx.send(params.code);
                "You can go back to your terminal now. :)".to_string()
            }
        }),
    );

    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();

    let code_future = rx.recv();
    axum::serve(listener, server)
        .with_graceful_shutdown({
            let mut shutdown_rx = tx.subscribe();
            async move {
                let _ = shutdown_rx.recv().await;
            }
        })
        .await
        .unwrap();

    code_future.await.unwrap()
}

async fn exchange_code(credentials: &Credentials, code: &str) -> Result<AuthToken, reqwest::Error> {
    let params = [
        ("code", code),
        ("client_id", &credentials.client_id),
        ("client_secret", &credentials.client_secret),
        ("redirect_uri", REDIRECT_URI),
        ("grant_type", "authorization_code"),
    ];

    let client = reqwest::Client::new();
    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await?;

    Ok::<AuthToken, reqwest::Error>(response.json::<AuthToken>().await.unwrap())
}
