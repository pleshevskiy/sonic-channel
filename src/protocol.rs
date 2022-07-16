use std::path::PathBuf;

use crate::{result::*, ChannelMode};

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
    Start {
        mode: ChannelMode,
        password: String,
    },
    Quit,
    Ping,
    Trigger(TriggerRequest),
    Suggest {
        collection: String,
        bucket: String,
        word: String,
        limit: Option<usize>,
    },
    Query {
        collection: String,
        bucket: String,
        terms: String,
        offset: Option<usize>,
        limit: Option<usize>,
        lang: Option<&'static str>,
    },
    Push {
        collection: String,
        bucket: String,
        object: String,
        terms: String,
        lang: Option<&'static str>,
    },
    Pop {
        collection: String,
        bucket: String,
        object: String,
        terms: String,
    },
    Flush(FlushRequest),
    Count {
        collection: String,
        bucket: Option<String>,
        object: Option<String>,
    },
}

pub enum TriggerRequest {
    Consolidate,
    Backup(PathBuf),
    Restore(PathBuf),
}

pub enum FlushRequest {
    Collection(String),
    Bucket {
        collection: String,
        bucket: String,
    },
    Object {
        collection: String,
        bucket: String,
        object: String,
    },
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
