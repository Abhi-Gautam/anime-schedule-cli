use async_trait::async_trait;
use anyhow::Result;

pub mod schedule;
pub mod search;
pub mod info;

pub use schedule::ScheduleCommand;

/// Base trait for all commands
#[async_trait]
pub trait Command {
    /// Execute the command
    async fn execute(&self) -> Result<()>;
} 