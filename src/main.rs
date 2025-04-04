// src/main.rs
mod api;
mod commands;
mod display;
mod utils;

use clap::{Parser, Subcommand};
use commands::{
    Command, ScheduleCommand, CountdownCommand,
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
        #[arg(short = 'd', long = "day")]
        day: Option<String>,

        /// Number of days to show schedule for
        #[arg(short = 'n', long = "days", default_value = "1")]
        interval: u32,

        /// Timezone to show schedule in (e.g., UTC, IST, JST)
        #[arg(short = 't', long = "timezone")]
        timezone: Option<String>,

        /// Show past episodes instead of upcoming ones
        #[arg(short = 'p', long = "past")]
        past: bool,
    },

    /// Show countdown for next airing episode
    Countdown {
        /// Timezone to show countdown in (e.g., UTC, IST, JST)
        #[arg(short = 't', long = "timezone")]
        timezone: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Schedule { day, interval, timezone, past } => {
            ScheduleCommand::new(day.clone(), *interval, timezone.clone(), *past)
                .execute()
                .await
                .expect("Failed to execute schedule command");
        }
        Commands::Countdown { timezone } => {
            CountdownCommand::new(timezone.clone())
                .execute()
                .await
                .expect("Failed to execute countdown command");
        }
    }
    Ok(())
}
