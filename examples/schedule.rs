use animesh::{commands::schedule::ScheduleCommand, Command};

#[tokio::main]
async fn main() {
    // Show next 2 days schedule in your timezone
    let command = ScheduleCommand::new(None, 2, None, false);
    command.execute().await.unwrap();

    // Show next 2 days schedule in IST
    let command = ScheduleCommand::new(None, 2, Some("IST".to_string()), false);
    command.execute().await.unwrap();

    // Show next 2 days schedule for Monday
    let command = ScheduleCommand::new(Some("monday".to_string()), 2, None, false);
    command.execute().await.unwrap();

    // Show next 5 days schedule
    let command = ScheduleCommand::new(None, 5, None, false);
    command.execute().await.unwrap();

    // Show next 7 days schedule for Wednesday in PST
    let command = ScheduleCommand::new(
        Some("wednesday".to_string()),
        7,
        Some("PST".to_string()),
        false,
    );
    command.execute().await.unwrap();

    // Show next 14 days schedule in EST
    let command = ScheduleCommand::new(None, 14, Some("EST".to_string()), false);
    command.execute().await.unwrap();

    // Show past 2 days schedule
    let command = ScheduleCommand::new(None, 2, None, true);
    command.execute().await.unwrap();
} 