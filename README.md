# Anime Schedule CLI

A command-line tool to view anime airing schedules and countdowns from AniList.

## Features

- View upcoming anime airing schedule
- Show past episodes
- Customizable time range
- Timezone support
- Beautiful terminal output with colors

## Installation

```bash
cargo install animesh
```

## Usage

### View Schedule

```bash
# Show schedule for next 1 day (default)
animesh schedule

# Show schedule for next 7 days
animesh schedule --interval 7

# Show schedule in a specific timezone
animesh schedule --timezone "IST"

# Show past episodes from last 3 days
animesh schedule --interval 3 --past
```

## Timezone Support

The tool supports various timezone formats:
- Standard timezone names (e.g., "UTC", "IST", "JST")
- UTC offsets (e.g., "UTC+5:30", "UTC-4:00")
- IANA/Olson timezone database names (from chrono-tz) (e.g., "America/New_York","Europe/London")

If no timezone is specified, the tool will: Try to fallback to your current time zone (not guranteed)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.