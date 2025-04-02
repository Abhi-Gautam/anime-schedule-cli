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
#[command(name = "animesh", author = "Abhishek Gautam", version = "0.1.0", about = "Track anime schedules and discover new shows", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// View anime airing schedule
    Schedule {
        /// Day of the week to show schedule for (e.g., monday, tuesday)
        #[arg(short = 'w', long = "day")]
        day: Option<String>,

        /// Number of days to show schedule for
        #[arg(short = 'n', long = "days", default_value = "1")]
        interval: u32,

        /// Timezone to show schedule in (e.g., UTC, IST, JST)
        #[arg(short = 't', long = "timezone")]
        timezone: Option<String>,
    },
    /// Search for anime or manga
    Search {
        /// Search query
        query: String,
        /// Search type (anime or manga)
        #[arg(short, long, value_enum, default_value_t = SearchType::Anime)]
        type_: SearchType,
        /// Filter by year
        #[arg(short = 'y', long)]
        year: Option<i32>,
        /// Filter by season (WINTER, SPRING, SUMMER, FALL)
        #[arg(short = 's', long)]
        season: Option<String>,
    },
    /// Get detailed information about an anime or manga
    Info {
        /// ID of the anime or manga
        id: i32,
        /// Type of media (anime or manga)
        #[arg(short, long, value_enum, default_value_t = SearchType::Anime)]
        type_: SearchType,
        /// Include character information
        #[arg(short = 'c', long)]
        characters: bool,
        /// Include staff information
        #[arg(short = 's', long)]
        staff: bool,
    },
    /// View top anime or manga
    Top {
        /// Type of media to show (anime or manga)
        #[arg(short, long, value_enum, default_value_t = SearchType::Anime)]
        type_: SearchType,
        /// Filter by genre
        #[arg(short = 'g', long)]
        genre: Option<String>,
        /// Number of items to show (default: 10)
        #[arg(short = 'l', long, default_value_t = 10)]
        limit: usize,
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
        Commands::Schedule { day, interval, timezone } => {
            ScheduleCommand::new(day.clone(), *interval, timezone.clone())
                .execute()
                .await
                .expect("Failed to execute schedule command");
        }
        Commands::Search { query, type_, year, season } => {
            SearchCommand::new(
                query.clone(),
                Some(type_.to_string()),
                *year,
                season.clone(),
            )
                .execute()
                .await
                .expect("Failed to execute search command");
        }
        Commands::Info { id, type_, characters, staff } => {
            InfoCommand::new(*id, type_.to_string(), *characters, *staff)
                .execute()
                .await
                .expect("Failed to execute info command");
        }
        Commands::Top { type_, genre, limit } => {
            TopCommand::new(
                Some(type_.to_string()),
                genre.clone(),
                *limit,
            )
                .execute()
                .await
                .expect("Failed to execute top command");
        }
    }
    Ok(())
}
