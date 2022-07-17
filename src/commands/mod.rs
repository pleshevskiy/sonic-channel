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

pub(crate) use quit::*;
pub(crate) use start::*;

pub(crate) use ping::*;

#[cfg(feature = "ingest")]
pub(crate) use count::*;
#[cfg(feature = "ingest")]
pub(crate) use flush::*;
#[cfg(feature = "ingest")]
pub(crate) use pop::*;
#[cfg(feature = "ingest")]
pub(crate) use push::*;

#[cfg(feature = "search")]
pub(crate) use query::*;
#[cfg(feature = "search")]
pub(crate) use suggest::*;

#[cfg(feature = "control")]
pub(crate) use trigger::*;

use crate::protocol;
use crate::result::Result;

pub trait StreamCommand {
    type Response;

    fn request(&self) -> protocol::Request;

    fn receive(&self, res: protocol::Response) -> Result<Self::Response>;
}
