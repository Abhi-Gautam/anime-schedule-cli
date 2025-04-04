use animesh::commands::{Command, ScheduleCommand};

#[tokio::main]
async fn main() {
    // Show schedule for next 2 days (default)
    println!("Showing schedule for next 2 days:");
    ScheduleCommand::new(2, None, false)
        .execute()
        .await
        .expect("Failed to execute schedule command");

    // Show schedule for next 7 days in UTC
    println!("\nShowing schedule for next 7 days in UTC:");
    ScheduleCommand::new(7, Some("UTC".to_string()), false)
        .execute()
        .await
        .expect("Failed to execute schedule command");

    // Show past episodes from last 3 days
    println!("\nShowing past episodes from last 3 days:");
    ScheduleCommand::new(3, None, true)
        .execute()
        .await
        .expect("Failed to execute schedule command");
}
