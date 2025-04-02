use animesh::{commands::schedule::ScheduleCommand, Command};

#[tokio::main]
async fn main() {
    // Example 1: Show default schedule (7 days)
    println!("Example 1: Default Schedule (7 days)");
    println!("==================================");
    let command = ScheduleCommand::new(None, true, None);
    command.execute().await.unwrap();
    println!();

    // Example 2: Show schedule for 14 days
    println!("Example 2: Schedule for 14 days");
    println!("=============================");
    let command = ScheduleCommand::new(None, true, None);
    command.execute().await.unwrap();
    println!();

    // Example 3: Show schedule for 30 days
    println!("Example 3: Schedule for 30 days");
    println!("=============================");
    let command = ScheduleCommand::new(None, true, None);
    command.execute().await.unwrap();
} 