use anyhow::Result;
use async_trait::async_trait;
use prettytable::{color, Row};
use serde_json::Value;

use crate::{
    api::AniListClient,
    commands::Command,
    display::{create_table, styled_cell},
};

/// Command to search for anime or manga
pub struct SearchCommand {
    query: String,
    media_type: Option<String>,
    year: Option<i32>,
    season: Option<String>,
    client: AniListClient,
}

impl SearchCommand {
    /// Create a new search command
    pub fn new(
        query: String,
        media_type: Option<String>,
        year: Option<i32>,
        season: Option<String>,
    ) -> Self {
        Self {
            query,
            media_type,
            year,
            season,
            client: AniListClient::new(),
        }
    }
}

#[async_trait]
impl Command for SearchCommand {
    async fn execute(&self) -> Result<()> {
        let query = r#"
            query ($search: String, $type: MediaType, $year: Int, $season: MediaSeason) {
                Page(perPage: 20) {
                    media(
                        search: $search,
                        type: $type,
                        seasonYear: $year,
                        season: $season
                    ) {
                        id
                        title {
                            romaji
                            english
                        }
                        type
                        format
                        status
                        episodes
                        chapters
                        volumes
                        averageScore
                        popularity
                        startDate {
                            year
                            month
                            day
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "search": self.query,
            "type": self.media_type,
            "year": self.year,
            "season": self.season,
        });

        println!("Search Query: {}", self.query);
        println!("Variables: {}", serde_json::to_string_pretty(&variables)?);
        
        let response: Value = self.client.query(query, variables).await?;
        println!("Raw API Response: {}", serde_json::to_string_pretty(&response)?);
        
        let media = response["data"]["Page"]["media"].as_array().unwrap();
        println!("Number of results: {}", media.len());

        // Create and populate table
        let mut table = create_table(&[
            "Title",
            "Type",
            "Format",
            "Status",
            "Episodes/Chapters",
            "Score",
            "Start Date",
        ]);

        for item in media {
            let title = item["title"]["english"]
                .as_str()
                .or(item["title"]["romaji"].as_str())
                .unwrap_or("Unknown Title");
            
            let media_type = item["type"].as_str().unwrap_or("Unknown");
            let format = item["format"].as_str().unwrap_or("Unknown");
            let status = item["status"].as_str().unwrap_or("Unknown");
            
            let episodes = if media_type == "ANIME" {
                item["episodes"].as_i64().map(|e| e.to_string())
            } else {
                item["chapters"].as_i64().map(|c| c.to_string())
            }.unwrap_or_else(|| "?".to_string());
            
            let score = item["averageScore"].as_f64().unwrap_or(0.0);
            let start_date = if let Some(date) = item["startDate"].as_object() {
                format!(
                    "{}-{:02}-{:02}",
                    date["year"].as_i64().unwrap_or(0),
                    date["month"].as_i64().unwrap_or(0),
                    date["day"].as_i64().unwrap_or(0)
                )
            } else {
                "Unknown".to_string()
            };

            table.add_row(Row::new(vec![
                styled_cell(title, color::CYAN),
                styled_cell(media_type, color::YELLOW),
                styled_cell(format, color::GREEN),
                styled_cell(status, color::MAGENTA),
                styled_cell(&episodes, color::BLUE),
                styled_cell(&format!("{:.1}", score), color::RED),
                styled_cell(&start_date, color::WHITE),
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
    async fn test_search_command_anime() {
        let command = SearchCommand::new(
            "Naruto".to_string(),
            Some("ANIME".to_string()),
            None,
            None,
        );
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_search_command_manga() {
        let command = SearchCommand::new(
            "One Piece".to_string(),
            Some("MANGA".to_string()),
            None,
            None,
        );
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_search_command_with_year() {
        let command = SearchCommand::new(
            "Attack on Titan".to_string(),
            Some("ANIME".to_string()),
            Some(2023),
            None,
        );
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_search_command_with_season() {
        let command = SearchCommand::new(
            "Demon Slayer".to_string(),
            Some("ANIME".to_string()),
            None,
            Some("WINTER".to_string()),
        );
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_search_command_with_year_and_season() {
        let command = SearchCommand::new(
            "Jujutsu Kaisen".to_string(),
            Some("ANIME".to_string()),
            Some(2023),
            Some("FALL".to_string()),
        );
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_search_command_no_type() {
        let command = SearchCommand::new(
            "Bleach".to_string(),
            None,
            None,
            None,
        );
        assert!(command.execute().await.is_ok());
    }
} 