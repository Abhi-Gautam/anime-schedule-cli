use animesh::{commands::top::TopCommand, Command};

#[tokio::main]
async fn main() {
    // Example 1: Show top anime
    println!("Example 1: Top Anime");
    println!("==================");
    let command = TopCommand::new(
        Some("ANIME".to_string()),
        None,
        10,
    );
    command.execute().await.unwrap();
    println!();

    // Example 2: Show top manga
    println!("Example 2: Top Manga");
    println!("==================");
    let command = TopCommand::new(
        Some("MANGA".to_string()),
        None,
        10,
    );
    command.execute().await.unwrap();
    println!();

    // Example 3: Show top anime by genre
    println!("Example 3: Top Action Anime");
    println!("=========================");
    let command = TopCommand::new(
        Some("ANIME".to_string()),
        Some("Action".to_string()),
        10,
    );
    command.execute().await.unwrap();
    println!();

    // Example 4: Show top anime with custom limit
    println!("Example 4: Top 25 Anime");
    println!("=====================");
    let command = TopCommand::new(
        Some("ANIME".to_string()),
        None,
        25,
    );
    command.execute().await.unwrap();
    println!();

    // Example 5: Show top anime by genre with custom limit
    println!("Example 5: Top 15 Romance Anime");
    println!("=============================");
    let command = TopCommand::new(
        Some("ANIME".to_string()),
        Some("Romance".to_string()),
        15,
    );
    command.execute().await.unwrap();
    println!();

    // Example 6: Show top without type specification
    println!("Example 6: Top Overall");
    println!("=====================");
    let command = TopCommand::new(
        None,
        None,
        10,
    );
    command.execute().await.unwrap();
    println!();

    // Example 7: Show top anime by different genres
    println!("Example 7: Top Anime by Different Genres");
    println!("=====================================");
    let genres = ["Action", "Romance", "Comedy", "Drama", "Fantasy"];
    for genre in genres.iter() {
        println!("\nTop {} Anime:", genre);
        println!("{}", "=".repeat(genre.len() + 10));
        let command = TopCommand::new(
            Some("ANIME".to_string()),
            Some(genre.to_string()),
            5,
        );
        command.execute().await.unwrap();
    }

    // Example 8: Show top manga by different genres
    println!("\nExample 8: Top Manga by Different Genres");
    println!("=====================================");
    let genres = ["Action", "Romance", "Comedy", "Drama", "Fantasy"];
    for genre in genres.iter() {
        println!("\nTop {} Manga:", genre);
        println!("{}", "=".repeat(genre.len() + 10));
        let command = TopCommand::new(
            Some("MANGA".to_string()),
            Some(genre.to_string()),
            5,
        );
        command.execute().await.unwrap();
    }
} 