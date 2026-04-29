use clap::{Parser, Subcommand};
use nalendar::{google::GoogleClient, tui::App};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Auth,
    Calendar,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Auth) => {
            let _google_client = GoogleClient::new().await;
        }
        Some(Commands::Calendar) => {
            let google_client = GoogleClient::new().await.unwrap();
            let _ = google_client.get_calendars().await;
        }
        None => {
            color_eyre::install()?;
            ratatui::run(|terminal| App::default().run(terminal))?;
            let hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |panic_info| {
                // let _ = restore(); // ignore any errors as we are already failing
                hook(panic_info);
            }));
        }
    }
    Ok(())
}
