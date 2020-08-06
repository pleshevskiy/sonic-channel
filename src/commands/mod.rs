mod quit;
mod start;

mod ping;

#[cfg(feature = "ingest")]
mod count;
#[cfg(feature = "ingest")]
mod flush;
#[cfg(feature = "ingest")]
mod pop;
#[cfg(feature = "ingest")]
mod push;

#[cfg(feature = "search")]
mod query;
#[cfg(feature = "search")]
mod suggest;

pub(crate) use quit::QuitCommand;
pub(crate) use start::StartCommand;

pub(crate) use ping::PingCommand;

#[cfg(feature = "ingest")]
pub(crate) use count::CountCommand;
#[cfg(feature = "ingest")]
pub(crate) use flush::FlushCommand;
#[cfg(feature = "ingest")]
pub(crate) use pop::PopCommand;
#[cfg(feature = "ingest")]
pub(crate) use push::PushCommand;

#[cfg(feature = "search")]
pub(crate) use query::QueryCommand;
#[cfg(feature = "search")]
pub(crate) use suggest::SuggestCommand;

use crate::result::Result;

pub trait StreamCommand {
    type Response;

    const READ_LINES_COUNT: usize = 1;

    fn message(&self) -> String;

    fn receive(&self, message: String) -> Result<Self::Response>;
}
