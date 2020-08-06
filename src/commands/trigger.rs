use super::StreamCommand;
use crate::result::*;
use std::fmt;

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
    type Response = bool;

    fn message(&self) -> String {
        format!("TRIGGER {}\r\n", self.action)
    }

    fn receive(&self, message: String) -> Result<Self::Response> {
        if message == "OK\r\n" {
            Ok(true)
        } else {
            Err(Error::new(ErrorKind::WrongSonicResponse))
        }
    }
}
