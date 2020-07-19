mod quit;
mod start;

mod ping;

#[cfg(feature = "ingest")]
mod push;
#[cfg(feature = "search")]
mod query;

pub use quit::QuitCommand;
pub use start::StartCommand;

pub use ping::PingCommand;
#[cfg(feature = "ingest")]
pub use push::PushCommand;
#[cfg(feature = "search")]
pub use query::QueryCommand;

use crate::result::Result;

pub trait StreamCommand {
    type Response;

    const READ_LINES_COUNT: usize = 1;

    fn message(&self) -> String;

    fn receive(&self, message: String) -> Result<Self::Response>;
}
