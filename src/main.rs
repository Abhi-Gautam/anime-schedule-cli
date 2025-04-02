// src/main.rs
mod api;
mod commands;
mod display;
mod utils;

use clap::{Parser, Subcommand};
use commands::{
    Command, ScheduleCommand, SearchCommand, InfoCommand, TopCommand,
};
use anyhow::Result;
use tokio;

/// A powerful CLI tool for anime fans to track their favorite shows
#[derive(Parser)]
#[command(name = "animesh", author = "Your Name", version = "0.1.0", about = "Track anime schedules and discover new shows", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// View anime airing schedule
    Schedule {
        /// Number of days to show (default: 7)
        #[arg(short, long, default_value_t = 7)]
        days: u32,
    },
    /// Search for anime or manga
    Search {
        /// Search query
        query: String,
        /// Search type (anime or manga)
        #[arg(short, long, value_enum, default_value_t = SearchType::Anime)]
        type_: SearchType,
    },
    /// Get detailed information about an anime or manga
    Info {
        /// ID of the anime or manga
        id: i32,
    },
    /// View top anime or manga
    Top {
        /// Type of media to show (anime or manga)
        #[arg(short, long, value_enum, default_value_t = SearchType::Anime)]
        type_: SearchType,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum SearchType {
    Anime,
    Manga,
}

impl std::fmt::Display for SearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchType::Anime => write!(f, "ANIME"),
            SearchType::Manga => write!(f, "MANGA"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Schedule { days: _ } => {
            ScheduleCommand::new(None, true, None)
                .execute()
                .await
                .expect("Failed to execute schedule command");
        }
        Commands::Search { query, type_ } => {
            SearchCommand::new(
                query.clone(),
                Some(type_.to_string()),
                None,
                None,
            )
                .execute()
                .await
                .expect("Failed to execute search command");
        }
        Commands::Info { id } => {
            InfoCommand::new(*id, "ANIME".to_string(), false, false)
                .execute()
                .await
                .expect("Failed to execute info command");
        }
        Commands::Top { type_ } => {
            TopCommand::new(
                Some(type_.to_string()),
                None,
                10,
            )
                .execute()
                .await
                .expect("Failed to execute top command");
        }
    }
    Ok(())
}
