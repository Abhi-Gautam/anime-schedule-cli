# Animesh - Anime Schedule & Information CLI

A powerful command-line interface tool for anime fans to track their favorite shows, discover new anime, and get detailed information about anime and manga.

## Features

- 📅 View anime airing schedules by day
- 🔍 Search for anime and manga
- ℹ️ Get detailed information about anime/manga
- 🏆 Browse top-rated anime and manga
- 🌐 Timezone support for accurate airing times
- 🎨 Beautiful colored output in tables
- 📊 Comprehensive filtering options

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/animesh.git
cd animesh

# Build the project
cargo build --release

# Run the binary
./target/release/animesh
```

## Usage

### Schedule Command
View anime airing schedules:
```bash
# View today's schedule
animesh schedule

# View schedule for a specific day
animesh schedule monday

# View schedule with custom timezone
animesh schedule --timezone UTC
```

### Search Command
Search for anime or manga:
```bash
# Search for anime
animesh search "Attack on Titan" --type anime

# Search with filters
animesh search "Demon Slayer" --year 2023 --season WINTER

# Search for manga
animesh search "One Piece" --type manga
```

### Info Command
Get detailed information about anime/manga:
```bash
# Get basic anime info
animesh info 1 --type anime

# Get info with characters and staff
animesh info 1 --type anime --characters --staff

# Get manga info
animesh info 1 --type manga
```

### Top Command
View top-rated anime/manga:
```bash
# View top anime
animesh top --type anime

# View top anime in a specific genre
animesh top --type anime --genre "Action"

# View top manga with custom limit
animesh top --type manga --limit 25
```

## Examples

The project includes comprehensive examples demonstrating all features. Run them using:

```bash
cargo run --example <example_name>
```

Available examples:
- `schedule`: Demonstrates schedule viewing features
- `search`: Shows various search capabilities
- `info`: Examples of getting detailed information
- `top`: Demonstrates top-rated content features

See the [examples README](examples/README.md) for detailed information about each example.

## Project Structure

```
animesh/
├── src/
│   ├── api/          # API client and GraphQL queries
│   │   └── mod.rs
│   ├── commands/     # Command implementations
│   │   ├── schedule.rs
│   │   ├── search.rs
│   │   ├── info.rs
│   │   ├── top.rs
│   │   └── mod.rs
│   ├── display/      # Output formatting and tables
│   ├── utils/        # Utility functions
│   ├── lib.rs        # Library exports
│   └── main.rs       # CLI entry point
├── examples/         # Usage examples
│   ├── schedule.rs
│   ├── search.rs
│   ├── info.rs
│   ├── top.rs
│   └── README.md
└── README.md
```

## Dependencies

- `tokio`: Async runtime
- `clap`: Command line argument parsing
- `reqwest`: HTTP client
- `serde`: JSON serialization/deserialization
- `prettytable-rs`: Table formatting
- `colored`: Terminal colors
- `chrono`: Date and time handling

## Development

```bash
# Run tests
cargo test

# Run specific example
cargo run --example schedule

# Build documentation
cargo doc --open
```

## Recent Changes

- Added comprehensive example programs
- Implemented colored output for better readability
- Added timezone support for schedule command
- Created library interface for better code organization
- Added detailed test cases for all commands
- Improved error handling and user feedback
- Added support for manga-related operations

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [AniList](https://anilist.co/) for providing the API
- All contributors and users of the project