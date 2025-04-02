# Anime Schedule CLI Examples

This directory contains example programs demonstrating the various features of the Anime Schedule CLI.

## Running the Examples

To run any of the examples, use the following command:

```bash
cargo run --example <example_name>
```

For example:
```bash
cargo run --example schedule
cargo run --example search
cargo run --example info
cargo run --example top
```

## Available Examples

### 1. Schedule Examples (`schedule.rs`)
- Show today's schedule
- Show schedule for a specific day
- Show schedule with custom timezone
- Show schedule for all days of the week
- Show schedule using short day names

### 2. Search Examples (`search.rs`)
- Search for anime
- Search for manga
- Search with year filter
- Search with season filter
- Search with both year and season
- Search without type specification
- Search with different seasons

### 3. Info Examples (`info.rs`)
- Get basic anime info
- Get basic manga info
- Get anime info with characters
- Get anime info with staff
- Get anime info with both characters and staff
- Get manga info with characters
- Get manga info with staff
- Get info for multiple popular anime

### 4. Top Examples (`top.rs`)
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