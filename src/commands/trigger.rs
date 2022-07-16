use super::StreamCommand;
use crate::protocol;
use crate::result::*;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum TriggerAction<'a> {
    Consolidate,
    Backup(&'a str),
    Restore(&'a str),
}

impl Default for TriggerAction<'_> {
    fn default() -> Self {
        TriggerAction::Consolidate
    }
}

impl fmt::Display for TriggerAction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        match self {
            TriggerAction::Consolidate => write!(f, "consolidate"),
            TriggerAction::Backup(data) => write!(f, "backup {}", data),
            TriggerAction::Restore(data) => write!(f, "restore {}", data),
        }
    }
}

#[derive(Debug, Default)]
pub struct TriggerCommand<'a> {
    pub action: TriggerAction<'a>,
}

impl StreamCommand for TriggerCommand<'_> {
    type Response = ();

    fn request(&self) -> protocol::Request {
        let req = match self.action {
            TriggerAction::Consolidate => protocol::TriggerRequest::Consolidate,
            TriggerAction::Backup(path) => protocol::TriggerRequest::Backup(PathBuf::from(path)),
            TriggerAction::Restore(path) => protocol::TriggerRequest::Restore(PathBuf::from(path)),
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
