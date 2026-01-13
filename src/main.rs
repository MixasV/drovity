mod cli;
mod config;
mod oauth;
mod proxy;
mod daemon;
mod factory;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "drovity")]
#[command(about = "Google Gemini API proxy for Factory Droid", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive menu
    Menu,
    /// Start proxy server in foreground
    Start,
    /// Stop background proxy server
    Stop,
    /// Check proxy server status
    Status,
    /// Run proxy in background (daemon mode)
    Hide,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Menu) | None => {
            // Default: show interactive menu
            cli::menu::show_main_menu().await?;
        }
        Some(Commands::Start) => {
            // Start proxy in foreground
            daemon::start_foreground().await?;
        }
        Some(Commands::Stop) => {
            // Stop background proxy
            daemon::stop().await?;
        }
        Some(Commands::Status) => {
            // Show status
            daemon::status().await?;
        }
        Some(Commands::Hide) => {
            // Start in background
            daemon::start_background().await?;
        }
    }

    Ok(())
}
