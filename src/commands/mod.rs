use async_trait::async_trait;
use anyhow::Result;

mod schedule;
mod countdown;

pub use schedule::ScheduleCommand;
pub use countdown::CountdownCommand;

/// Base trait for all commands
#[async_trait]
pub trait Command {
    /// Execute the command
    async fn execute(&self) -> Result<()>;
} 