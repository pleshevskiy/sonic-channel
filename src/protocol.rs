use std::path::PathBuf;

use crate::result::*;

//===========================================================================//
// Primitives                                                                //
//===========================================================================//

#[repr(u8)]
pub enum Version {
    V1 = 1,
}

//===========================================================================//
// Response                                                                  //
//===========================================================================//

pub type EventId = String;

#[derive(Debug)]
pub enum Response {
    Ok,
    Ended,
    Connected,
    Pending(EventId),
    Pong,
    Started(StartedPayload),
    Result(usize),
    Event(EventKind, EventId, Vec<String>),
}

#[derive(Debug)]
pub struct StartedPayload {
    pub protocol_version: usize,
    pub max_buffer_size: usize,
}

#[derive(Debug)]
pub enum EventKind {
    Suggest,
    Query,
}

//===========================================================================//
// Request                                                                   //
//===========================================================================//

pub enum Request {
    Trigger(TriggerRequest),
    Suggest(SuggestRequest),
}

pub enum TriggerRequest {
    Consolidate,
    Backup(PathBuf),
    Restore(PathBuf),
}

pub struct SuggestRequest {
    collection: String,
    bucket: String,
    word: String,
}

//===========================================================================//
// Utils                                                                     //
//===========================================================================//

pub(crate) fn parse_server_config(raw: &str) -> Result<usize> {
    raw.split_terminator(&['(', ')'])
        .nth(1)
        .ok_or(Error::WrongResponse)?
        .parse()
        .map_err(|_| Error::WrongResponse)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_protocol() {
        match parse_server_config("protocol(1)") {
            Ok(protocol) => assert_eq!(protocol, 1),
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_parse_buffer_size() {
        match parse_server_config("buffer_size(20000)") {
            Ok(buffer_size) => assert_eq!(buffer_size, 20000),
            _ => unreachable!(),
        }
    }
}
