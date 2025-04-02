use anyhow::Result;
use async_trait::async_trait;
use prettytable::{color, Row};
use serde_json::Value;

use crate::{
    api::AniListClient,
    commands::Command,
    display::{create_table, styled_cell},
};

/// Command to show top-ranked anime or manga
pub struct TopCommand {
    media_type: Option<String>,
    genre: Option<String>,
    limit: usize,
    client: AniListClient,
}

impl TopCommand {
    /// Create a new top command
    pub fn new(
        media_type: Option<String>,
        genre: Option<String>,
        limit: usize,
    ) -> Self {
        Self {
            media_type,
            genre,
            limit,
            client: AniListClient::new(),
        }
    }
}

#[async_trait]
impl Command for TopCommand {
    async fn execute(&self) -> Result<()> {
        let query = r#"
            query ($type: MediaType, $genre: String, $perPage: Int) {
                Page(perPage: $perPage) {
                    media(
                        type: $type,
                        genre: $genre,
                        sort: SCORE_DESC
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
                        genres
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
            "type": self.media_type,
            "genre": self.genre,
            "perPage": self.limit,
        });

        let response: Value = self.client.query(query, variables).await?;
        let media = response["data"]["Page"]["media"].as_array().unwrap();

        // Create and populate table
        let mut table = create_table(&[
            "Title",
            "Type",
            "Format",
            "Status",
            "Episodes/Chapters",
            "Score",
            "Popularity",
            "Genres",
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
            let popularity = item["popularity"].as_i64().unwrap_or(0);
            let genres = item["genres"]
                .as_array()
                .unwrap()
                .iter()
                .map(|g| g.as_str().unwrap_or("Unknown"))
                .collect::<Vec<_>>()
                .join(", ");
            
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
                styled_cell(&popularity.to_string(), color::WHITE),
                styled_cell(&genres, color::GREEN),
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
    async fn test_top_command_anime() {
        let command = TopCommand::new(
            Some("ANIME".to_string()),
            None,
            10,
        );
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_top_command_manga() {
        let command = TopCommand::new(
            Some("MANGA".to_string()),
            None,
            10,
        );
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_top_command_with_genre() {
        let command = TopCommand::new(
            Some("ANIME".to_string()),
            Some("Action".to_string()),
            10,
        );
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_top_command_with_custom_limit() {
        let command = TopCommand::new(
            Some("ANIME".to_string()),
            None,
            25,
        );
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_top_command_with_genre_and_limit() {
        let command = TopCommand::new(
            Some("ANIME".to_string()),
            Some("Romance".to_string()),
            15,
        );
        assert!(command.execute().await.is_ok());
    }

    #[tokio::test]
    async fn test_top_command_no_type() {
        let command = TopCommand::new(
            None,
            None,
            10,
        );
        assert!(command.execute().await.is_ok());
    }
} 