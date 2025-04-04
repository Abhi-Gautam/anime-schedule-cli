use anyhow::Result;
use async_trait::async_trait;

pub mod schedule;

pub use schedule::ScheduleCommand;

/// Base trait for all commands
#[async_trait]
pub trait Command {
    /// Execute the command
    async fn execute(&self) -> Result<()>;
}
