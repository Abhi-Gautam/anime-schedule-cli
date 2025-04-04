use chrono::{FixedOffset, Local, TimeZone, Utc};
use chrono_tz::TZ_VARIANTS;
use std::env;

/// Match timezone string to FixedOffset
pub fn match_timezone(tz: &str) -> Option<FixedOffset> {
    match tz.to_uppercase().as_str() {
        "UTC" => Some(FixedOffset::east_opt(0).unwrap()),
        "IST" => Some(FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap()), // UTC+5:30
        "JST" => Some(FixedOffset::east_opt(9 * 3600).unwrap()),           // UTC+9
        "PST" => Some(FixedOffset::west_opt(8 * 3600).unwrap()),           // UTC-8
        "EST" => Some(FixedOffset::west_opt(5 * 3600).unwrap()),           // UTC-5
        _ => {
            // Try to parse as offset (e.g., "+05:30")
            if let Ok(offset) = tz.parse::<i32>() {
                Some(
                    FixedOffset::east_opt(offset * 3600)
                        .unwrap_or(FixedOffset::east_opt(0).unwrap()),
                )
            } else if tz.starts_with('+') || tz.starts_with('-') {
                // Try to parse offsets like "+05:30" or "-08:00"
                let hours_mins: Vec<&str> = tz[1..].split(':').collect();
                if hours_mins.len() == 2 {
                    if let (Ok(hours), Ok(mins)) =
                        (hours_mins[0].parse::<i32>(), hours_mins[1].parse::<i32>())
                    {
                        let total_secs = hours * 3600 + mins * 60;
                        if tz.starts_with('+') {
                            Some(FixedOffset::east_opt(total_secs).unwrap())
                        } else {
                            Some(FixedOffset::west_opt(total_secs).unwrap())
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                // Try to find in chrono-tz database
                for variant in TZ_VARIANTS.iter() {
                    if variant.name() == tz {
                        // Get current time
                        let now = Utc::now();
                        // Convert to the timezone
                        let tz_time = variant.from_utc_datetime(&now.naive_utc());
                        // Calculate the offset by comparing with UTC
                        let utc_timestamp = now.timestamp();
                        let tz_timestamp = tz_time.timestamp();
                        let offset_secs = (tz_timestamp - utc_timestamp) as i32;

                        if offset_secs >= 0 {
                            return Some(FixedOffset::east_opt(offset_secs).unwrap());
                        } else {
                            return Some(FixedOffset::west_opt(-offset_secs).unwrap());
                        }
                    }
                }
                None
            }
        }
    }
}

/// Get the user's timezone from the system in a cross-platform way
pub fn get_user_timezone() -> FixedOffset {
    // Method 1: Try to get timezone from TZ environment variable
    if let Ok(tz) = env::var("TZ") {
        if let Some(offset) = match_timezone(&tz) {
            return offset;
        }
    }

    // Method 2: Use chrono's Local to get the current local timezone offset
    let local_now = Local::now();
    let offset_secs = local_now.offset().local_minus_utc();

    if offset_secs >= 0 {
        FixedOffset::east_opt(offset_secs).unwrap_or_else(|| FixedOffset::east_opt(0).unwrap())
    } else {
        FixedOffset::west_opt(-offset_secs).unwrap_or_else(|| FixedOffset::east_opt(0).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, TimeZone, Utc};

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
