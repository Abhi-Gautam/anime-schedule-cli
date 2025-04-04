use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Duration, FixedOffset, TimeZone, Utc};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
    style::{Color, Print, ResetColor, SetForegroundColor},
    cursor,
};
use std::{
    io::{stdout, Write},
    time::Duration as StdDuration,
};

use crate::{
    api::AniListClient,
    commands::Command,
    utils::{get_user_timezone, match_timezone},
};

/// Command to show countdown for upcoming episodes
pub struct CountdownCommand {
    timezone: Option<String>,
    client: AniListClient,
}

impl CountdownCommand {
    /// Create a new countdown command
    pub fn new(timezone: Option<String>) -> Self {
        Self {
            timezone,
            client: AniListClient::new(),
        }
    }

    /// Get the timezone to use for display
    fn get_timezone(&self) -> FixedOffset {
        if let Some(tz) = &self.timezone {
            match_timezone(&tz).unwrap_or_else(|| {
                eprintln!("Invalid timezone: {}. Using default timezone.", tz);
                get_user_timezone()
            })
        } else {
            get_user_timezone()
        }
    }

    /// Format duration as countdown string
    fn format_countdown(duration: Duration) -> String {
        let days = duration.num_days();
        let hours = duration.num_hours() % 24;
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;

        if days > 0 {
            format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
        } else if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }

    /// Get next airing episode
    async fn get_next_episode(&self) -> Result<Option<(String, DateTime<FixedOffset>)>> {
        let timezone = self.get_timezone();
        let now = Utc::now().timestamp();

        // GraphQL query for next airing episode
        let query = r#"
            query ($now: Int) {
                Page(perPage: 1) {
                    airingSchedules(airingAt_greater: $now, sort: AIRING_AT) {
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

        let variables = serde_json::json!({
            "now": now,
        });

        let response: serde_json::Value = self.client.query(query, variables).await?;
        let schedules = response["data"]["Page"]["airingSchedules"].as_array().unwrap();

        if let Some(schedule) = schedules.first() {
            let title = schedule["media"]["title"]["english"]
                .as_str()
                .or(schedule["media"]["title"]["romaji"].as_str())
                .unwrap_or("Unknown Title");
            
            let episode: i64 = schedule["episode"].as_i64().unwrap_or(0);
            let airing_at: i64 = schedule["airingAt"].as_i64().unwrap_or(0);
            
            let airing_time = Utc.timestamp_opt(airing_at, 0).unwrap().with_timezone(&timezone);
            
            Ok(Some((format!("{} Episode {}", title, episode), airing_time)))
        } else {
            Ok(None)
        }
    }

    /// Run the countdown display
    async fn run_countdown(&self, title: String, target_time: DateTime<FixedOffset>) -> Result<()> {
        // Set up terminal
        terminal::enable_raw_mode()?;
        let mut stdout = stdout();

        // Clear screen and hide cursor
        execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide)?;

        let mut last_update = std::time::Instant::now();
        let update_interval = StdDuration::from_millis(100); // Update every 100ms

        loop {
            // Check for 'q' key press
            if event::poll(StdDuration::from_millis(0))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }

            // Update display if enough time has passed
            if last_update.elapsed() >= update_interval {
                let now = Utc::now().with_timezone(&target_time.timezone());
                let duration = target_time - now;
                
                // Clear previous line
                execute!(stdout, cursor::MoveTo(0, 0), terminal::Clear(ClearType::CurrentLine))?;
                
                // Print title
                execute!(
                    stdout,
                    SetForegroundColor(Color::Cyan),
                    Print(&title),
                    ResetColor,
                    Print("\n"),
                )?;
                
                // Print countdown
                execute!(
                    stdout,
                    SetForegroundColor(Color::Yellow),
                    Print(Self::format_countdown(duration)),
                    ResetColor,
                    Print("\n"),
                )?;
                
                // Print target time
                execute!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    Print(format!("Airing at: {}", target_time.format("%Y-%m-%d %H:%M:%S %Z"))),
                    ResetColor,
                    Print("\n\nPress 'q' to quit"),
                )?;
                
                stdout.flush()?;
                last_update = std::time::Instant::now();
            }
        }

        // Restore terminal
        execute!(stdout, cursor::Show)?;
        terminal::disable_raw_mode()?;
        
        Ok(())
    }
}

#[async_trait]
impl Command for CountdownCommand {
    async fn execute(&self) -> Result<()> {
        if let Some((title, target_time)) = self.get_next_episode().await? {
            self.run_countdown(title, target_time).await?;
        } else {
            println!("No upcoming episodes found.");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_countdown() {
        assert_eq!(
            CountdownCommand::format_countdown(Duration::days(2) + Duration::hours(3) + Duration::minutes(45) + Duration::seconds(30)),
            "2d 3h 45m 30s"
        );
        assert_eq!(
            CountdownCommand::format_countdown(Duration::hours(5) + Duration::minutes(30) + Duration::seconds(15)),
            "5h 30m 15s"
        );
        assert_eq!(
            CountdownCommand::format_countdown(Duration::minutes(45) + Duration::seconds(30)),
            "45m 30s"
        );
        assert_eq!(
            CountdownCommand::format_countdown(Duration::seconds(30)),
            "30s"
        );
    }
} 