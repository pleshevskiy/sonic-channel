use std::io::{self, BufWriter, Write};
use std::{path::PathBuf, str::FromStr};

use crate::{result::*, ChannelMode};

#[derive(Debug, Default)]
pub struct Protocol {
    #[allow(dead_code)]
    version: Version,
}

impl From<Version> for Protocol {
    fn from(version: Version) -> Self {
        Self { version }
    }
}

impl Protocol {
    pub fn format_request(&self, req: Request) -> io::Result<Vec<u8>> {
        let mut res = BufWriter::new(Vec::new());

        match req {
            Request::Quit => write!(res, "QUIT")?,

            Request::Ping => write!(res, "PING")?,

            Request::Start { mode, password } => write!(res, "START {} {}", mode, password)?,

            #[rustfmt::skip]
            Request::Count { collection, bucket, object } => match (bucket, object) {
                (Some(b), Some(o)) => write!(res, "COUNT {} {} {}", collection, b, o)?,
                (Some(b), None) => write!(res, "COUNT {} {}", collection, b)?,
                (None, None) => write!(res, "COUNT {}", collection)?,
                _ => panic!("Wrong protocol format"),
            },

            #[rustfmt::skip]
            Request::Flush { collection, bucket, object } => match (bucket, object) {
                (Some(b), Some(o)) => write!(res, "FLUSHO {} {} {}", collection, b, o)?,
                (Some(b), None) => write!(res, "FLUSHB {} {}", collection, b)?,
                (None, None) => write!(res, "FLUSHC {}", collection)?,
                _ => panic!("Wrong protocol format"),
            },

            #[rustfmt::skip]
            Request::Pop { collection, bucket, object, terms } => {
                write!(res, "POP {} {} {} \"{}\"", collection, bucket, object, terms)?
            },
            #[rustfmt::skip]
            Request::Push { collection, bucket, object, terms, lang } => {
                let oneline_terms = remove_multiline(&terms);
                write!(res, "PUSH {} {} {} \"{}\"", collection, bucket, object, oneline_terms)?;
                if let Some(lang) = lang {
                    write!(res, " LANG({})", lang)?
                }
            }

            #[rustfmt::skip]
            Request::Query { collection, bucket, terms, offset, limit, lang } => {
                write!(res, "QUERY {} {} \"{}\"", collection, bucket, terms)?;
                if let Some(limit) = limit {
                    write!(res, " LIMIT({})", limit)?;
                }
                if let Some(offset) = offset {
                    write!(res, " OFFSET({})", offset)?;
                }
                if let Some(lang) = lang {
                    write!(res, " LANG({})", lang)?;
                }
            }
            #[rustfmt::skip]
            Request::Suggest { collection, bucket, word, limit } => {
                write!(res, "SUGGEST {} {} \"{}\"", collection, bucket, word)?;
                if let Some(limit) = limit {
                    write!(res, " LIMIT({})", limit)?;
                }
            }

            Request::Trigger(triger_req) => match triger_req {
                TriggerRequest::Consolidate => write!(res, "TRIGGER consolidate")?,
                TriggerRequest::Backup(path) => {
                    write!(res, "TRIGGER backup {}", path.to_str().unwrap())?
                }
                TriggerRequest::Restore(path) => {
                    write!(res, "TRIGGER restore {}", path.to_str().unwrap())?
                }
            },
        }

        write!(res, "\r\n")?;
        res.flush()?;

        Ok(res.into_inner()?)
    }

    pub fn parse_response(&self, line: &str) -> Result<Response> {
        let mut segments = line.split_whitespace();
        match segments.next() {
            Some("STARTED") => match (segments.next(), segments.next(), segments.next()) {
                (Some(_raw_mode), Some(raw_protocol), Some(raw_buffer_size)) => {
                    Ok(Response::Started(StartedPayload {
                        protocol_version: parse_server_config(raw_protocol)?,
                        max_buffer_size: parse_server_config(raw_buffer_size)?,
                    }))
                }
                _ => Err(Error::WrongResponse),
            },
            Some("PENDING") => {
                let event_id = segments
                    .next()
                    .map(String::from)
                    .ok_or(Error::WrongResponse)?;
                Ok(Response::Pending(event_id))
            }
            Some("RESULT") => match segments.next() {
                Some(num) => num
                    .parse()
                    .map(Response::Result)
                    .map_err(|_| Error::WrongResponse),
                _ => Err(Error::WrongResponse),
            },
            Some("EVENT") => {
                let event_kind = match segments.next() {
                    Some("SUGGEST") => Ok(EventKind::Suggest),
                    Some("QUERY") => Ok(EventKind::Query),
                    _ => Err(Error::WrongResponse),
                }?;

                let event_id = segments
                    .next()
                    .map(String::from)
                    .ok_or(Error::WrongResponse)?;

                let objects = segments.map(String::from).collect();

                Ok(Response::Event(event_kind, event_id, objects))
            }
            Some("OK") => Ok(Response::Ok),
            Some("ENDED") => Ok(Response::Ended),
            Some("CONNECTED") => Ok(Response::Connected),
            Some("ERR") => match segments.next() {
                Some(message) => Err(Error::SonicServer(String::from(message))),
                _ => Err(Error::WrongResponse),
            },
            _ => Err(Error::WrongResponse),
        }
    }
}

//===========================================================================//
// Primitives                                                                //
//===========================================================================//

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Version {
    V1 = 1,
}

impl Default for Version {
    fn default() -> Self {
        Self::V1
    }
}

impl TryFrom<u8> for Version {
    type Error = ();

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::V1),
            _ => Err(()),
        }
    }
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
    pub protocol_version: u8,
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

#[derive(Debug)]
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
    Flush {
        collection: String,
        bucket: Option<String>,
        object: Option<String>,
    },
    Count {
        collection: String,
        bucket: Option<String>,
        object: Option<String>,
    },
}

#[derive(Debug)]
pub enum TriggerRequest {
    Consolidate,
    Backup(PathBuf),
    Restore(PathBuf),
}

//===========================================================================//
// Utils                                                                     //
//===========================================================================//

fn parse_server_config<T: FromStr>(raw: &str) -> Result<T> {
    raw.split_terminator(&['(', ')'])
        .nth(1)
        .ok_or(Error::WrongResponse)?
        .parse()
        .map_err(|_| Error::WrongResponse)
}

fn remove_multiline(text: &str) -> String {
    text.lines()
        .enumerate()
        .fold(String::new(), |mut acc, (i, line)| {
            if i != 0 && !line.is_empty() && !acc.is_empty() && !acc.ends_with(' ') {
                acc.push(' ');
            }

            acc.push_str(line);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_protocol() {
        match parse_server_config::<u8>("protocol(1)") {
            Ok(protocol) => assert_eq!(protocol, 1),
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_parse_buffer_size() {
        match parse_server_config::<usize>("buffer_size(20000)") {
            Ok(buffer_size) => assert_eq!(buffer_size, 20000),
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_make_single_line() {
        let text = "
Hello
World
";

        let expected_text = "Hello World";
        assert_eq!(remove_multiline(text), expected_text);
    }
}
