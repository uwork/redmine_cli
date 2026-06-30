mod cli;
mod client;
mod commands;
mod config;
mod models;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Config(cmd) => commands::config::run(cmd).await,
        Commands::Issues(cmd) => commands::issues::run(cmd).await,
        Commands::Projects(cmd) => commands::projects::run(cmd).await,
        Commands::Users(cmd) => commands::users::run(cmd).await,
    }
}
