use anyhow::Result;
use async_trait::async_trait;
use chrono::{Datelike, FixedOffset, TimeZone, Utc};
use prettytable::{color, Row};
use serde_json::Value;

use crate::{
    api::AniListClient,
    commands::Command,
    display::{create_table, format_datetime, styled_cell},
    utils::{get_day_timestamps, get_user_timezone, parse_day_of_week},
};

/// Command to show upcoming anime airing schedule
pub struct ScheduleCommand {
    day: Option<String>,
    today: bool,
    timezone: Option<String>,
    client: AniListClient,
}

impl ScheduleCommand {
    /// Create a new schedule command
    pub fn new(day: Option<String>, today: bool, timezone: Option<String>) -> Self {
        Self {
            day,
            today,
            timezone,
            client: AniListClient::new(),
        }
    }

    /// Get the timezone to use for display
    fn get_timezone(&self) -> FixedOffset {
        if let Some(_tz) = &self.timezone {
            // TODO: Parse timezone string properly
            FixedOffset::east_opt(0).unwrap()
        } else {
            get_user_timezone()
        }
    }

    /// Get the day to show schedule for
    fn get_target_day(&self) -> u32 {
        if self.today {
            Utc::now().weekday().num_days_from_monday()
        } else if let Some(day) = &self.day {
            parse_day_of_week(day).unwrap_or(0)
        } else {
            0 // Default to Monday
        }
    }
}

#[async_trait]
impl Command for ScheduleCommand {
    async fn execute(&self) -> Result<()> {
        let timezone = self.get_timezone();
        let target_day = self.get_target_day();
        let (start, end) = get_day_timestamps(target_day);

        // GraphQL query for airing schedule
        let query = r#"
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

        let variables = serde_json::json!({
            "start": start,
            "end": end,
        });

        let response: Value = self.client.query(query, variables).await?;
        let schedules = response["data"]["Page"]["airingSchedules"].as_array().unwrap();

        // Create and populate table
        let mut table = create_table(&["Title", "Episode", "Airs at"]);
        
        for schedule in schedules {
            let title = schedule["media"]["title"]["english"]
                .as_str()
                .or(schedule["media"]["title"]["romaji"].as_str())
                .unwrap_or("Unknown Title");
            
            let episode: i64 = schedule["episode"].as_i64().unwrap_or(0);
            let airing_at: i64 = schedule["airingAt"].as_i64().unwrap_or(0);
            
            let airing_time = Utc.timestamp_opt(airing_at, 0).unwrap();
            let formatted_time = format_datetime(airing_time, timezone);

            table.add_row(Row::new(vec![
                styled_cell(title, color::CYAN),
                styled_cell(&episode.to_string(), color::YELLOW),
                styled_cell(&formatted_time, color::GREEN),
            ]));
        }

        table.printstd();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_schedule_command_today() {
        let command = ScheduleCommand::new(None, true, None);
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_schedule_command_specific_day() {
        let command = ScheduleCommand::new(Some("monday".to_string()), false, None);
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_schedule_command_with_timezone() {
        let command = ScheduleCommand::new(None, true, Some("UTC".to_string()));
        assert!(command.execute().await.is_ok());
    }

    #[test]
    fn test_get_timezone() {
        let command = ScheduleCommand::new(None, false, None);
        let tz = command.get_timezone();
        // The timezone should be valid (either UTC or a specific offset)
        assert!(tz.utc_minus_local() >= -14 * 3600 && tz.utc_minus_local() <= 14 * 3600);
    }

    #[test]
    fn test_get_target_day() {
        let command = ScheduleCommand::new(None, true, None);
        let day = command.get_target_day();
        assert!(day < 7);
    }

    #[test]
    fn test_parse_day_of_week() {
        let command = ScheduleCommand::new(Some("monday".to_string()), false, None);
        assert_eq!(command.get_target_day(), 0);

        let command = ScheduleCommand::new(Some("tuesday".to_string()), false, None);
        assert_eq!(command.get_target_day(), 1);

        let command = ScheduleCommand::new(Some("wednesday".to_string()), false, None);
        assert_eq!(command.get_target_day(), 2);

        let command = ScheduleCommand::new(Some("thursday".to_string()), false, None);
        assert_eq!(command.get_target_day(), 3);

        let command = ScheduleCommand::new(Some("friday".to_string()), false, None);
        assert_eq!(command.get_target_day(), 4);

        let command = ScheduleCommand::new(Some("saturday".to_string()), false, None);
        assert_eq!(command.get_target_day(), 5);

        let command = ScheduleCommand::new(Some("sunday".to_string()), false, None);
        assert_eq!(command.get_target_day(), 6);

        // Test short forms
        let command = ScheduleCommand::new(Some("mon".to_string()), false, None);
        assert_eq!(command.get_target_day(), 0);

        let command = ScheduleCommand::new(Some("tue".to_string()), false, None);
        assert_eq!(command.get_target_day(), 1);

        let command = ScheduleCommand::new(Some("wed".to_string()), false, None);
        assert_eq!(command.get_target_day(), 2);

        let command = ScheduleCommand::new(Some("thu".to_string()), false, None);
        assert_eq!(command.get_target_day(), 3);

        let command = ScheduleCommand::new(Some("fri".to_string()), false, None);
        assert_eq!(command.get_target_day(), 4);

        let command = ScheduleCommand::new(Some("sat".to_string()), false, None);
        assert_eq!(command.get_target_day(), 5);

        let command = ScheduleCommand::new(Some("sun".to_string()), false, None);
        assert_eq!(command.get_target_day(), 6);

        // Test invalid day
        let command = ScheduleCommand::new(Some("invalid".to_string()), false, None);
        assert_eq!(command.get_target_day(), 0); // Should default to Monday
    }
} 