use super::StreamCommand;
use crate::protocol;
use crate::result::*;
use std::path::PathBuf;

/// Parameters for the `trigger` command.
#[derive(Debug)]
pub enum TriggerRequest<'a> {
    /// Consolidate indexed search data instead of waiting for the next automated
    /// consolidation tick.
    Consolidate,

    /// Backup KV + FST to <path>/<BACKUP_{KV/FST}_PATH>
    /// See [sonic backend source code](https://github.com/valeriansaliou/sonic/blob/master/src/channel/command.rs#L808)
    /// for more information.
    Backup(&'a str),

    /// Restore KV + FST from <path> if you already have backup with the same name.
    Restore(&'a str),
}

#[derive(Debug)]
pub struct TriggerCommand<'a> {
    pub(crate) req: TriggerRequest<'a>,
}

impl StreamCommand for TriggerCommand<'_> {
    type Response = ();

    fn request(&self) -> protocol::Request {
        let req = match self.req {
            TriggerRequest::Consolidate => protocol::TriggerRequest::Consolidate,
            TriggerRequest::Backup(path) => protocol::TriggerRequest::Backup(PathBuf::from(path)),
            TriggerRequest::Restore(path) => protocol::TriggerRequest::Restore(PathBuf::from(path)),
        };

        protocol::Request::Trigger(req)
    }

    fn receive(&self, res: protocol::Response) -> Result<Self::Response> {
        if matches!(res, protocol::Response::Ok) {
            Ok(())
        } else {
            Err(Error::WrongResponse)
        }
    }
}
