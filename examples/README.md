# Animesh Examples

This directory contains example programs demonstrating the usage of the Animesh library.

## Schedule Example

The `schedule.rs` example demonstrates how to use the schedule command to view anime airing schedules:

```rust
use animesh::commands::{Command, ScheduleCommand};

#[tokio::main]
async fn main() {
    // Show schedule for next 2 days (default)
    ScheduleCommand::new(2, None, false)
        .execute()
        .await
        .expect("Failed to execute schedule command");

    // Show schedule for next 7 days in UTC
    ScheduleCommand::new(7, Some("UTC".to_string()), false)
        .execute()
        .await
        .expect("Failed to execute schedule command");

    // Show past episodes from last 3 days
    ScheduleCommand::new(3, None, true)
        .execute()
        .await
        .expect("Failed to execute schedule command");
}
```

Run this example using:
```bash
cargo run --example schedule
```

## Countdown Example

The `countdown.rs` example demonstrates how to use the countdown command to show countdown for a specific anime:

```rust
use animesh::commands::{Command, CountdownCommand};

#[tokio::main]
async fn main() {
    // Show countdown for One Piece
    CountdownCommand::new("One Piece".to_string())
        .execute()
        .await
        .expect("Failed to execute countdown command");
}
```

Run this example using:
```bash
cargo run --example countdown
```

## Available Examples

### 1. Search Examples (`search.rs`)
- Search for anime
- Search for manga
- Search with year filter
- Search with season filter
- Search with both year and season
- Search without type specification
- Search with different seasons

### 2. Info Examples (`info.rs`)
- Get basic anime info
- Get basic manga info
- Get anime info with characters
- Get anime info with staff
- Get anime info with both characters and staff
- Get manga info with characters
- Get manga info with staff
- Get info for multiple popular anime

### 3. Top Examples (`top.rs`)
- Show top anime
- Show top manga
- Show top anime by genre
- Show top anime with custom limit
- Show top anime by genre with custom limit
- Show top without type specification
- Show top anime by different genres
- Show top manga by different genres

## Example Output

Each example will print its results to the console in a formatted table. The output includes:
- Colored text for better readability
- Formatted tables with headers
- Clear section separators
- Descriptive titles for each example

## Note

These examples use real API calls to AniList, so they require an internet connection to work. Some examples might take a few seconds to complete due to API rate limiting. 