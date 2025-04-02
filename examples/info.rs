use animesh::{commands::info::InfoCommand, Command};

#[tokio::main]
async fn main() {
    // Example 1: Get basic anime info
    println!("Example 1: Basic Anime Info");
    println!("==========================");
    let command = InfoCommand::new(1, "ANIME".to_string(), false, false);
    command.execute().await.unwrap();
    println!();

    // Example 2: Get basic manga info
    println!("Example 2: Basic Manga Info");
    println!("==========================");
    let command = InfoCommand::new(1, "MANGA".to_string(), false, false);
    command.execute().await.unwrap();
    println!();

    // Example 3: Get anime info with characters
    println!("Example 3: Anime Info with Characters");
    println!("==================================");
    let command = InfoCommand::new(1, "ANIME".to_string(), true, false);
    command.execute().await.unwrap();
    println!();

    // Example 4: Get anime info with staff
    println!("Example 4: Anime Info with Staff");
    println!("==============================");
    let command = InfoCommand::new(1, "ANIME".to_string(), false, true);
    command.execute().await.unwrap();
    println!();

    // Example 5: Get anime info with both characters and staff
    println!("Example 5: Anime Info with Characters and Staff");
    println!("============================================");
    let command = InfoCommand::new(1, "ANIME".to_string(), true, true);
    command.execute().await.unwrap();
    println!();

    // Example 6: Get manga info with characters
    println!("Example 6: Manga Info with Characters");
    println!("==================================");
    let command = InfoCommand::new(1, "MANGA".to_string(), true, false);
    command.execute().await.unwrap();
    println!();

    // Example 7: Get manga info with staff
    println!("Example 7: Manga Info with Staff");
    println!("==============================");
    let command = InfoCommand::new(1, "MANGA".to_string(), false, true);
    command.execute().await.unwrap();
    println!();

    // Example 8: Get info for multiple popular anime
    println!("Example 8: Info for Multiple Popular Anime");
    println!("=======================================");
    let anime_ids = [1, 16498, 9253, 16498, 16498]; // Example IDs for popular anime
    for id in anime_ids.iter() {
        println!("\nGetting info for anime ID: {}", id);
        println!("{}", "=".repeat(25));
        let command = InfoCommand::new(*id, "ANIME".to_string(), true, true);
        command.execute().await.unwrap();
    }
} 