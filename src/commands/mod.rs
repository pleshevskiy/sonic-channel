mod quit;
mod start;

mod ping;

#[cfg(feature = "ingest")]
mod flush;
#[cfg(feature = "ingest")]
mod push;

#[cfg(feature = "search")]
mod query;
#[cfg(feature = "search")]
mod suggest;

pub use quit::QuitCommand;
pub use start::StartCommand;

pub use ping::PingCommand;

#[cfg(feature = "ingest")]
pub use flush::FlushCommand;
#[cfg(feature = "ingest")]
pub use push::PushCommand;

#[cfg(feature = "search")]
pub use query::QueryCommand;
#[cfg(feature = "search")]
pub use suggest::SuggestCommand;

use crate::result::Result;

pub trait StreamCommand {
    type Response;

    const READ_LINES_COUNT: usize = 1;

    fn message(&self) -> String;

    fn receive(&self, message: String) -> Result<Self::Response>;
}
