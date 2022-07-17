use super::StreamCommand;
use crate::protocol;
use crate::result::*;
use std::path::PathBuf;

#[derive(Debug)]
pub enum TriggerRequest<'a> {
    Consolidate,
    Backup(&'a str),
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
