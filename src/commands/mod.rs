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

#[cfg(feature = "control")]
mod trigger;

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

#[cfg(feature = "control")]
pub(crate) use trigger::{TriggerAction, TriggerCommand};

use crate::protocol;
use crate::result::Result;

pub trait StreamCommand {
    type Response;

    fn request(&self) -> protocol::Request;

    fn receive(&self, res: protocol::Response) -> Result<Self::Response>;
}
