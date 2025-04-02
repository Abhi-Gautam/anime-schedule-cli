use animesh::{commands::schedule::ScheduleCommand, Command};

#[tokio::main]
async fn main() {
    // Example 1: Show today's schedule
    println!("Example 1: Today's Schedule");
    println!("==========================");
    let command = ScheduleCommand::new(None, true, None);
    command.execute().await.unwrap();
    println!();

    // Example 2: Show schedule for a specific day
    println!("Example 2: Monday's Schedule");
    println!("==========================");
    let command = ScheduleCommand::new(Some("monday".to_string()), false, None);
    command.execute().await.unwrap();
    println!();

    // Example 3: Show schedule with custom timezone
    println!("Example 3: Schedule in UTC");
    println!("==========================");
    let command = ScheduleCommand::new(None, true, Some("UTC".to_string()));
    command.execute().await.unwrap();
    println!();

    // Example 4: Show schedule for a specific day with custom timezone
    println!("Example 4: Friday's Schedule in UTC");
    println!("================================");
    let command = ScheduleCommand::new(Some("friday".to_string()), false, Some("UTC".to_string()));
    command.execute().await.unwrap();
    println!();

    // Example 5: Show schedule using short day names
    println!("Example 5: Schedule using short day names");
    println!("=====================================");
    let command = ScheduleCommand::new(Some("mon".to_string()), false, None);
    command.execute().await.unwrap();
    println!();

    // Example 6: Show schedule for all days of the week
    println!("Example 6: Schedule for all days of the week");
    println!("========================================");
    let days = ["monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday"];
    for day in days.iter() {
        println!("\n{}'s Schedule:", day.to_uppercase());
        println!("{}", "=".repeat(day.len() + 10));
        let command = ScheduleCommand::new(Some(day.to_string()), false, None);
        command.execute().await.unwrap();
    }
} 