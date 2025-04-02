use async_trait::async_trait;
use anyhow::Result;

pub mod schedule;
pub mod search;
pub mod info;
pub mod top;

pub use schedule::ScheduleCommand;
pub use search::SearchCommand;
pub use info::InfoCommand;
pub use top::TopCommand;

/// Base trait for all commands
#[async_trait]
pub trait Command {
    /// Execute the command
    async fn execute(&self) -> Result<()>;
} 