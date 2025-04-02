use chrono::FixedOffset;
use std::env;

/// Get the user's timezone from environment variables
pub fn get_user_timezone() -> FixedOffset {
    if let Ok(tz) = env::var("TZ") {
        // Try to parse the timezone string
        if let Ok(offset) = tz.parse::<i32>() {
            return FixedOffset::east_opt(offset * 3600)
                .unwrap_or(FixedOffset::east_opt(0).unwrap());
        }
    }

    // Default to UTC if no timezone is set
    FixedOffset::east_opt(0).unwrap()
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
