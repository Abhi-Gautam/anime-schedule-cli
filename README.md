Anime Schedule CLI
==================

**Anime Schedule CLI** is a command-line tool built in Rust that allows users to fetch airing schedules and release dates of their favorite anime. This tool fetches data from AniList (or any other API you integrate with) and displays the information in your terminal.

Features
--------

-   Fetch all animes airing today.
-   Get the next release date for a specific anime by name.

Installation
------------

To use the Anime Schedule CLI, follow the steps below to install it on your machine.

### Prerequisites

-   Rust installed on your machine. You can check if you have Rust installed by running the following command:

    `rustc --version`

### Build and Install

1.  **Clone the repository:**

    Clone this repository to your local machine:

    `git clone https://github.com/yourusername/anime-schedule-cli.git` \
    `cd anime-schedule-cli`

2.  **Install the CLI tool:**

    You can install the CLI tool locally using `cargo install`. This will compile the project and place the binary in your Cargo bin directory, which is usually in your system's PATH.

    `cargo install --path .`

4.  **Verify the installation:**

    Once installed, you can check if the CLI tool is available by typing:

    `anime-schedule --help`

    This should display the help message for the tool.

Usage
-----

You can now use the Anime Schedule CLI tool from your terminal. Below are the available commands:

### Fetch Today's Airing Anime

To get all the anime that are airing today, use the following command:

`anime-schedule today`

This will print a list of all the anime airing today, including the episode number and the airing time in both UTC and Indian Standard Time (IST).

`Yao Shen Ji 8 - Episode 27 airs at 2024-10-29 07:30:00 IST` \
`A Terrified Teacher at Ghoul School! - Episode 4 airs at 2024-10-29 19:30:00 IST`\
`I’ll Become a Villainess Who Goes Down in History - Episode 5 airs at 2024-10-29 20:00:00 IST`\
`Hibike! Euphonium 3: Extra Episodes - Episode 5 airs at 2024-10-29 20:30:00 IST`\
`Tying the Knot with an Amagami Sister - Episode 5 airs at 2024-10-29 20:30:00 IST`\
`Pochars - Episode 4 airs at 2024-10-29 21:55:00 IST`
`Tasuketsu -Fate of the Majority- - Episode 16 airs at 2024-10-29 22:29:00 IST`\
`Chibi Godzilla Raids Again Season 2 - Episode 31 airs at 2024-10-30 03:37:00 IST`

### Get Release Dates for a Specific Anime

To get the release dates for a specific anime, use the `release` command followed by the anime's name:

`anime-schedule release "Naruto"`

This will print the next episode's airing time and total episodes for the specified anime.

Uninstall
---------

If you want to uninstall the Anime Schedule CLI tool, you can use Cargo's `uninstall` command:

bash

Copy code

`cargo uninstall anime-schedule`

This will remove the CLI tool from your system.