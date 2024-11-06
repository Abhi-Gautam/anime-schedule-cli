// src/main.rs
use chrono::{FixedOffset, TimeZone, Utc};
use clap::{Parser, Subcommand};
use serde::Deserialize;
#[macro_use] extern crate prettytable;
use prettytable::{color, format, Attr, Cell, Row, Table};

// Command-line interface setup using Clap
#[derive(Parser)]
#[command(name = "Anime Schedule CLI")]
#[command(about = "Fetch anime schedules and release dates")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get all animes airing today
    Today,
    /// Get release dates of a specific anime
    Release {
        /// Name of the anime
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Today => {
            fetch_today_anime();
        }
        Commands::Release { name } => {
            fetch_anime_release_dates(name);
        }
    }
}

// GraphQL Queries
const TODAY_QUERY: &str = r#"
query ($start: Int, $end: Int) {
  Page(perPage: 50) {
    airingSchedules(airingAt_greater: $start, airingAt_lesser: $end) {
      airingAt
      episode
      media {
        title {
          romaji
          english
        }
      }
    }
  }
}
"#;

const RELEASE_QUERY: &str = r#"
query ($search: String) {
  Media(search: $search, type: ANIME) {
    title {
      romaji
      english
    }
    episodes
    startDate {
      year
      month
      day
    }
    endDate {
      year
      month
      day
    }
    nextAiringEpisode {
      airingAt
      episode
    }
  }
}
"#;

// Data Structures for 'Today' Command
#[derive(Deserialize)]
struct AiringScheduleResponse {
    data: AiringData,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct AiringData {
    Page: AiringPage,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct AiringPage {
    airingSchedules: Vec<AiringSchedule>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct AiringSchedule {
    airingAt: i64,
    episode: i32,
    media: AiringMedia,
}

#[derive(Deserialize)]
struct AiringMedia {
    title: Title,
}

#[derive(Deserialize)]
struct Title {
    romaji: Option<String>,
    english: Option<String>,
}

// Data Structures for 'Release' Command
#[derive(Deserialize)]
struct ReleaseResponse {
    data: ReleaseData,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct ReleaseData {
    Media: MediaInfo,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct MediaInfo {
    title: Title,
    episodes: Option<i32>,
    startDate: Option<Date>,
    endDate: Option<Date>,
    nextAiringEpisode: Option<NextAiringEpisode>,
}

#[derive(Deserialize)]
struct Date {
    year: Option<i32>,
    month: Option<i32>,
    day: Option<i32>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct NextAiringEpisode {
    airingAt: i64,
    episode: i32,
}

// Function to fetch today's airing anime
fn fetch_today_anime() {
    // Calculate start and end of the day in UNIX timestamp
    let now = Utc::now();
    let start_of_day = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .expect("Invalid start of day time")
        .and_utc()
        .timestamp();
    let end_of_day = now
        .date_naive()
        .and_hms_opt(23, 59, 59)
        .expect("Invalid end of day time")
        .and_utc()
        .timestamp();

    // Variables for the GraphQL query
    let variables = serde_json::json!({
        "start": start_of_day,
        "end": end_of_day,
    });

    // Build the request payload
    let payload = serde_json::json!({
        "query": TODAY_QUERY,
        "variables": variables,
    });

    // Send the request
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://graphql.anilist.co")
        .json(&payload)
        .send();

    let res = match res {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Network error: {}", e);
            return;
        }
    };

    // Parse the response
    let response: Result<AiringScheduleResponse, _> = res.json();

    let response = match response {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing response: {}", e);
            return;
        }
    };

    // Display the results
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    // Add a header row
    table.set_titles(row![
        b->"Title",
        b->"Episode",
        b->"Airs at (IST)"
    ]);
    for schedule in response.data.Page.airingSchedules {
        let title = schedule
            .media
            .title
            .english
            .clone()
            .or(schedule.media.title.romaji.clone())
            .unwrap_or_else(|| "Unknown Title".to_string());
        let title_cell = Cell::new(&title)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::CYAN));
        let episode_cell = Cell::new(&schedule.episode.to_string())
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::YELLOW));
        let airing_time = Utc.timestamp_opt(schedule.airingAt, 0);

        match airing_time {
            chrono::LocalResult::Single(utc_time) => {
                // Convert the UTC time to IST (UTC+5:30)
                let ist_offset = FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap(); // IST is UTC+5:30
                let ist_time = utc_time.with_timezone(&ist_offset);

                // println!(
                //     "{} - Episode {} airs at {} IST",
                //     title,
                //     schedule.episode,
                //     ist_time.format("%Y-%m-%d %H:%M:%S")
                // );
                let time_cell = Cell::new(&ist_time.format("%Y-%m-%d %H:%M:%S").to_string())
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::GREEN));
                table.add_row(Row::new(vec![title_cell, episode_cell, time_cell]));
            }
            chrono::LocalResult::None => {
                println!("Invalid airing time for Episode {}", schedule.episode);
            }
            chrono::LocalResult::Ambiguous(_, _) => {
                println!("Ambiguous airing time for Episode {}", schedule.episode);
            }
        }
    }
    table.printstd();
}

// Function to fetch release dates of a specific anime
fn fetch_anime_release_dates(name: &str) {
    let variables = serde_json::json!({ "search": name });

    let payload = serde_json::json!({
        "query": RELEASE_QUERY,
        "variables": variables,
    });

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://graphql.anilist.co")
        .json(&payload)
        .send();

    let res = match res {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Network error: {}", e);
            return;
        }
    };

    let response: Result<ReleaseResponse, _> = res.json();

    let response = match response {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing response: {}", e);
            return;
        }
    };

    let media = response.data.Media;
    let title = media
        .title
        .english
        .clone()
        .or(media.title.romaji.clone())
        .unwrap_or_else(|| "Unknown Title".to_string());

    println!("Title: {}", title);

    if let Some(episodes) = media.episodes {
        println!("Total Episodes: {}", episodes);
    }

    if let Some(start_date) = media.startDate {
        println!(
            "Start Date: {}-{:02}-{:02}",
            start_date.year.unwrap_or(0),
            start_date.month.unwrap_or(0),
            start_date.day.unwrap_or(0)
        );
    }

    if let Some(end_date) = media.endDate {
        println!(
            "End Date: {}-{:02}-{:02}",
            end_date.year.unwrap_or(0),
            end_date.month.unwrap_or(0),
            end_date.day.unwrap_or(0)
        );
    }

    if let Some(next_ep) = media.nextAiringEpisode {
        let airing_time = Utc.timestamp_opt(next_ep.airingAt, 0);
        match airing_time {
            chrono::LocalResult::Single(utc_time) => {
                let ist_offset = FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap(); // IST is UTC+5:30
                let ist_time = utc_time.with_timezone(&ist_offset);
                println!(
                    "Next Episode {} airs at {} IST",
                    next_ep.episode,
                    ist_time.format("%Y-%m-%d %H:%M:%S")
                );
            }
            chrono::LocalResult::None => {
                println!("Invalid airing time for Episode");
            }
            chrono::LocalResult::Ambiguous(_, _) => {
                println!("Ambiguous airing time for Episode");
            }
        }
    }
}
