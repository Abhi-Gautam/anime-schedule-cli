use animesh::{commands::search::SearchCommand, Command};

#[tokio::main]
async fn main() {
    // Example 1: Search for anime
    println!("Example 1: Search for Anime");
    println!("==========================");
    let command = SearchCommand::new(
        "Naruto".to_string(),
        Some("ANIME".to_string()),
        None,
        None,
    );
    command.execute().await.unwrap();
    println!();

    // Example 2: Search for manga
    println!("Example 2: Search for Manga");
    println!("==========================");
    let command = SearchCommand::new(
        "One Piece".to_string(),
        Some("MANGA".to_string()),
        None,
        None,
    );
    command.execute().await.unwrap();
    println!();

    // Example 3: Search with year filter
    println!("Example 3: Search with Year Filter");
    println!("================================");
    let command = SearchCommand::new(
        "Attack on Titan".to_string(),
        Some("ANIME".to_string()),
        Some(2023),
        None,
    );
    command.execute().await.unwrap();
    println!();

    // Example 4: Search with season filter
    println!("Example 4: Search with Season Filter");
    println!("=================================");
    let command = SearchCommand::new(
        "Demon Slayer".to_string(),
        Some("ANIME".to_string()),
        None,
        Some("WINTER".to_string()),
    );
    command.execute().await.unwrap();
    println!();

    // Example 5: Search with both year and season
    println!("Example 5: Search with Year and Season");
    println!("====================================");
    let command = SearchCommand::new(
        "Jujutsu Kaisen".to_string(),
        Some("ANIME".to_string()),
        Some(2023),
        Some("FALL".to_string()),
    );
    command.execute().await.unwrap();
    println!();

    // Example 6: Search without type specification
    println!("Example 6: Search without Type");
    println!("=============================");
    let command = SearchCommand::new(
        "Bleach".to_string(),
        None,
        None,
        None,
    );
    command.execute().await.unwrap();
    println!();

    // Example 7: Search with different seasons
    println!("Example 7: Search with Different Seasons");
    println!("======================================");
    let seasons = ["WINTER", "SPRING", "SUMMER", "FALL"];
    for season in seasons.iter() {
        println!("\nSearching for {} 2023 anime:", season);
        println!("{}", "=".repeat(season.len() + 20));
        let command = SearchCommand::new(
            "".to_string(),
            Some("ANIME".to_string()),
            Some(2023),
            Some(season.to_string()),
        );
        command.execute().await.unwrap();
    }
} 