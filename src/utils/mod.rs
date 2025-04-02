use chrono::FixedOffset;
use std::env;

/// Get the user's timezone from environment variables
pub fn get_user_timezone() -> FixedOffset {
    // Try to get timezone from TZ environment variable
    if let Ok(tz) = env::var("TZ") {
        println!("Found TZ environment variable: {}", tz);
        match tz.to_uppercase().as_str() {
            "UTC" => return FixedOffset::east_opt(0).unwrap(),
            "IST" => return FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap(), // UTC+5:30
            "JST" => return FixedOffset::east_opt(9 * 3600).unwrap(), // UTC+9
            "PST" => return FixedOffset::west_opt(8 * 3600).unwrap(), // UTC-8
            "EST" => return FixedOffset::west_opt(5 * 3600).unwrap(), // UTC-5
            _ => {
                // Try to parse as offset (e.g., "+05:30")
                if let Ok(offset) = tz.parse::<i32>() {
                    return FixedOffset::east_opt(offset * 3600)
                        .unwrap_or(FixedOffset::east_opt(0).unwrap());
                }
            }
        }
    }

    // Try to get timezone from system
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("systemsetup")
            .arg("-gettimezone")
            .output() {
            if let Ok(tz) = String::from_utf8(output.stdout) {
                match tz.trim().to_uppercase().as_str() {
                    "UTC" => return FixedOffset::east_opt(0).unwrap(),
                    "IST" => return FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap(),
                    "JST" => return FixedOffset::east_opt(9 * 3600).unwrap(),
                    "PST" => return FixedOffset::west_opt(8 * 3600).unwrap(),
                    "EST" => return FixedOffset::west_opt(5 * 3600).unwrap(),
                    _ => {}
                }
            }
        }
    }

    // Default to IST (UTC+5:30) for Indian users
    FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap()
}

/// Parse a day of the week string into a number (0-6, where 0 is Monday)
pub fn parse_day_of_week(day: &str) -> Option<u32> {
    match day.to_lowercase().as_str() {
        "mon" | "monday" => Some(0),
        "tue" | "tuesday" => Some(1),
        "wed" | "wednesday" => Some(2),
        "thu" | "thursday" => Some(3),
        "fri" | "friday" => Some(4),
        "sat" | "saturday" => Some(5),
        "sun" | "sunday" => Some(6),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc, Datelike};

    use super::*;

    /// Get the start and end timestamps for a given day
    pub fn get_day_timestamps(day: u32) -> (i64, i64) {
        let now = Utc::now();
        let current_day = now.weekday().num_days_from_monday();
        let days_ahead = (day + 7 - current_day) % 7;

        let target_date = now.date_naive() + chrono::Days::new(days_ahead as u64);
        let start = target_date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp();
        let end = target_date
            .and_hms_opt(23, 59, 59)
            .unwrap()
            .and_utc()
            .timestamp();

        (start, end)
    }

    #[test]
    fn test_get_user_timezone() {
        let tz = get_user_timezone();
        println!("User timezone offset: {} seconds", tz.utc_minus_local());
        assert!(tz.utc_minus_local() >= -14 * 3600 && tz.utc_minus_local() <= 14 * 3600);
    }

    #[test]
    fn test_parse_day_of_week() {
        assert_eq!(parse_day_of_week("mon"), Some(0));
        assert_eq!(parse_day_of_week("MONDAY"), Some(0));
        assert_eq!(parse_day_of_week("invalid"), None);
    }

    #[test]
    fn test_get_day_timestamps() {
        let (start, end) = get_day_timestamps(0);
        assert!(start < end);
        assert_eq!(
            Utc.timestamp_opt(start, 0)
                .unwrap()
                .weekday()
                .num_days_from_monday(),
            0
        );
    }
}
