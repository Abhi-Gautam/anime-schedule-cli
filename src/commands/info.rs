use anyhow::Result;
use async_trait::async_trait;
use colored::*;
use prettytable::{color, Row};
use serde_json::Value;

use crate::{
    api::AniListClient,
    commands::Command,
    display::{create_table, styled_cell},
};

/// Command to show detailed information about an anime/manga
pub struct InfoCommand {
    id: i32,
    media_type: String,
    show_characters: bool,
    show_staff: bool,
    client: AniListClient,
}

impl InfoCommand {
    /// Create a new info command
    pub fn new(id: i32, media_type: String, show_characters: bool, show_staff: bool) -> Self {
        Self {
            id,
            media_type,
            show_characters,
            show_staff,
            client: AniListClient::new(),
        }
    }
}

#[async_trait]
impl Command for InfoCommand {
    async fn execute(&self) -> Result<()> {
        let query = r#"
            query ($id: Int, $type: MediaType) {
                Media(id: $id, type: $type) {
                    id
                    title {
                        romaji
                        english
                        native
                    }
                    type
                    format
                    status
                    description
                    episodes
                    chapters
                    volumes
                    duration
                    averageScore
                    popularity
                    startDate {
                        year
                        month
                        day
                    }
                    endDate {
                        year
                        month
                        day
                    }
                    nextAiringEpisode {
                        episode
                        airingAt
                    }
                    characters(sort: ROLE) {
                        nodes {
                            role
                            character {
                                name {
                                    full
                                }
                                image {
                                    medium
                                }
                            }
                        }
                    }
                    staff {
                        nodes {
                            role
                            person {
                                name {
                                    full
                                }
                                image {
                                    medium
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "id": self.id,
            "type": self.media_type,
        });

        let response: Value = self.client.query(query, variables).await?;
        let media = &response["data"]["Media"];

        // Print basic information
        let title = media["title"]["english"]
            .as_str()
            .or(media["title"]["romaji"].as_str())
            .unwrap_or("Unknown Title");
        
        println!("{}", title.bold());
        println!("{}", "=".repeat(title.len()));
        println!();

        // Create and populate table
        let mut table = create_table(&["Property", "Value"]);

        let episodes = media["episodes"].as_i64().map(|e| e.to_string()).unwrap_or_else(|| "?".to_string());
        let chapters = media["chapters"].as_i64().map(|c| c.to_string()).unwrap_or_else(|| "?".to_string());
        let volumes = media["volumes"].as_i64().map(|v| v.to_string()).unwrap_or_else(|| "?".to_string());
        let duration = media["duration"].as_i64().map(|d| format!("{} min", d)).unwrap_or_else(|| "?".to_string());
        let score = format!("{:.1}", media["averageScore"].as_f64().unwrap_or(0.0));
        let popularity = media["popularity"].as_i64().unwrap_or(0).to_string();

        let properties = vec![
            ("Type", media["type"].as_str().unwrap_or("Unknown")),
            ("Format", media["format"].as_str().unwrap_or("Unknown")),
            ("Status", media["status"].as_str().unwrap_or("Unknown")),
            ("Episodes", &episodes),
            ("Chapters", &chapters),
            ("Volumes", &volumes),
            ("Duration", &duration),
            ("Score", &score),
            ("Popularity", &popularity),
        ];

        for (key, value) in properties {
            table.add_row(Row::new(vec![
                styled_cell(key, color::CYAN),
                styled_cell(value, color::WHITE),
            ]));
        }

        table.printstd();
        println!();

        // Print description
        if let Some(desc) = media["description"].as_str() {
            println!("{}", "Description".bold());
            println!("{}", "=".repeat("Description".len()));
            println!("{}", desc);
            println!();
        }

        // Print dates
        let start_date = format_date(&media["startDate"]);
        let end_date = format_date(&media["endDate"]);
        println!("{}", "Dates".bold());
        println!("{}", "=".repeat("Dates".len()));
        println!("Start: {}", start_date);
        println!("End: {}", end_date);
        println!();

        // Print next airing episode if available
        if let Some(next) = media["nextAiringEpisode"].as_object() {
            let episode = next["episode"].as_i64().unwrap_or(0);
            let airing_at = next["airingAt"].as_i64().unwrap_or(0);
            let date = chrono::DateTime::from_timestamp(airing_at, 0)
                .unwrap()
                .format("%Y-%m-%d %H:%M")
                .to_string();
            println!("{}", "Next Episode".bold());
            println!("{}", "=".repeat("Next Episode".len()));
            println!("Episode {} airs on {}", episode, date);
            println!();
        }

        // Print characters if requested
        if self.show_characters {
            if let Some(characters) = media["characters"]["nodes"].as_array() {
                println!("{}", "Main Characters".bold());
                println!("{}", "=".repeat("Main Characters".len()));
                for character in characters.iter().take(5) {
                    let role = character["role"].as_str().unwrap_or("Unknown");
                    let name = character["character"]["name"]["full"].as_str().unwrap_or("Unknown");
                    println!("{}: {}", role, name);
                }
                println!();
            }
        }

        // Print staff if requested
        if self.show_staff {
            if let Some(staff) = media["staff"]["nodes"].as_array() {
                println!("{}", "Key Staff".bold());
                println!("{}", "=".repeat("Key Staff".len()));
                for member in staff.iter().take(5) {
                    let role = member["role"].as_str().unwrap_or("Unknown");
                    let name = member["person"]["name"]["full"].as_str().unwrap_or("Unknown");
                    println!("{}: {}", role, name);
                }
            }
        }

        Ok(())
    }
}

fn format_date(date: &Value) -> String {
    let year = date["year"].as_i64().unwrap_or(0);
    let month = date["month"].as_i64().unwrap_or(0);
    let day = date["day"].as_i64().unwrap_or(0);
    
    if year == 0 && month == 0 && day == 0 {
        "Unknown".to_string()
    } else {
        format!("{:04}-{:02}-{:02}", year, month, day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_info_command_anime() {
        let command = InfoCommand::new(1, "ANIME".to_string(), false, false);
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_info_command_manga() {
        let command = InfoCommand::new(1, "MANGA".to_string(), false, false);
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_info_command_with_characters() {
        let command = InfoCommand::new(1, "ANIME".to_string(), true, false);
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_info_command_with_staff() {
        let command = InfoCommand::new(1, "ANIME".to_string(), false, true);
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_info_command_with_characters_and_staff() {
        let command = InfoCommand::new(1, "ANIME".to_string(), true, true);
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_info_command_manga_with_characters() {
        let command = InfoCommand::new(1, "MANGA".to_string(), true, false);
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_info_command_manga_with_staff() {
        let command = InfoCommand::new(1, "MANGA".to_string(), false, true);
        assert!(command.execute().await.is_ok());
    }
} 