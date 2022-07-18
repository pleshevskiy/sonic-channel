mod ping;
mod quit;
mod start;

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

pub(crate) use self::{ping::PingCommand, quit::QuitCommand, start::StartCommand};

#[cfg(feature = "ingest")]
pub(crate) use self::{
    count::CountCommand, flush::FlushCommand, pop::PopCommand, push::PushCommand,
};
#[cfg(feature = "ingest")]
pub use self::{count::CountRequest, flush::FlushRequest, pop::PopRequest, push::PushRequest};

#[cfg(feature = "search")]
pub(crate) use self::{query::QueryCommand, suggest::SuggestCommand};
#[cfg(feature = "search")]
pub use self::{query::QueryRequest, suggest::SuggestRequest};

#[cfg(feature = "control")]
pub(crate) use trigger::TriggerCommand;
#[cfg(feature = "control")]
pub use trigger::TriggerRequest;

use crate::protocol;
use crate::result::Result;

#[doc(hidden)]
pub trait StreamCommand {
    type Response;

    fn request(&self) -> protocol::Request;

    fn receive(&self, res: protocol::Response) -> Result<Self::Response>;
}
