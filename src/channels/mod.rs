#[cfg(feature = "search")]
mod search;
#[cfg(feature = "search")]
pub use search::*;

#[cfg(feature = "ingest")]
mod ingest;
#[cfg(feature = "ingest")]
pub use ingest::*;

#[cfg(feature = "control")]
mod control;
#[cfg(feature = "control")]
pub use control::*;

use std::cell::RefCell;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};

use crate::commands::{StartCommand, StreamCommand};
use crate::protocol;
use crate::result::*;

const DEFAULT_SONIC_PROTOCOL_VERSION: usize = 1;
const UNINITIALIZED_MODE_MAX_BUFFER_SIZE: usize = 200;

/// Channel modes supported by sonic search backend.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelMode {
    /// Sonic server search channel mode.
    ///
    /// In this mode you can use `query`, `suggest`, `ping` and `quit` commands.
    ///
    /// Note: This mode requires enabling the `search` feature.
    #[cfg(feature = "search")]
    Search,

    /// Sonic server ingest channel mode.
    ///
    /// In this mode you can use `push`, `pop`, `flushc`, `flushb`, `flusho`,
    /// `bucket_count`, `object_count`, `word_count`, `ping` and `quit` commands.
    ///
    /// Note: This mode requires enabling the `ingest` feature.
    #[cfg(feature = "ingest")]
    Ingest,

    /// Sonic server control channel mode.
    ///
    /// In this mode you can use `consolidate`, `backup`, `restore`,
    /// `ping` and `quit` commands.
    ///
    /// Note: This mode requires enabling the `control` feature.
    #[cfg(feature = "control")]
    Control,
}

impl ChannelMode {
    /// Converts enum to &str
    pub fn as_str(&self) -> &str {
        match self {
            #[cfg(feature = "search")]
            ChannelMode::Search => "search",

            #[cfg(feature = "ingest")]
            ChannelMode::Ingest => "ingest",

            #[cfg(feature = "control")]
            ChannelMode::Control => "control",
        }
    }
}

impl std::fmt::Display for ChannelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Root and Heart of this library.
///
/// You can connect to the sonic search backend and run all supported protocol methods.
///
#[derive(Debug)]
pub struct SonicStream {
    stream: RefCell<TcpStream>,
    reader: RefCell<BufReader<TcpStream>>,
    mode: Option<ChannelMode>, // None – Uninitialized mode
    max_buffer_size: usize,
    protocol_version: usize,
}

impl SonicStream {
    fn write<SC: StreamCommand>(&self, command: &SC) -> Result<()> {
        let message = command.format();
        self.stream
            .borrow_mut()
            .write_all(message.as_bytes())
            .map_err(|_| Error::WriteToStream)?;
        Ok(())
    }

    fn read_line(&self) -> Result<protocol::Response> {
        let line = {
            let mut line = String::with_capacity(self.max_buffer_size);
            self.reader
                .borrow_mut()
                .read_line(&mut line)
                .map_err(|_| Error::ReadStream)?;
            line
        };

        dbg!(&line);

        let mut segments = line.split_whitespace();
        match segments.next() {
            Some("STARTED") => match (segments.next(), segments.next(), segments.next()) {
                (Some(_raw_mode), Some(raw_protocol), Some(raw_buffer_size)) => {
                    Ok(protocol::Response::Started(protocol::StartedPayload {
                        protocol_version: protocol::parse_server_config(raw_protocol)?,
                        max_buffer_size: protocol::parse_server_config(raw_buffer_size)?,
                    }))
                }
                _ => Err(Error::WrongResponse),
            },
            Some("PENDING") => {
                let event_id = segments
                    .next()
                    .map(String::from)
                    .ok_or(Error::WrongResponse)?;
                Ok(protocol::Response::Pending(event_id))
            }
            Some("RESULT") => match segments.next() {
                Some(num) => num
                    .parse()
                    .map(protocol::Response::Result)
                    .map_err(|_| Error::WrongResponse),
                _ => Err(Error::WrongResponse),
            },
            Some("EVENT") => {
                let event_kind = match segments.next() {
                    Some("SUGGEST") => Ok(protocol::EventKind::Suggest),
                    Some("QUERY") => Ok(protocol::EventKind::Query),
                    _ => Err(Error::WrongResponse),
                }?;

                let event_id = segments
                    .next()
                    .map(String::from)
                    .ok_or(Error::WrongResponse)?;

                let objects = segments.map(String::from).collect();

                Ok(protocol::Response::Event(event_kind, event_id, objects))
            }
            Some("OK") => Ok(protocol::Response::Ok),
            Some("ENDED") => Ok(protocol::Response::Ended),
            Some("CONNECTED") => Ok(protocol::Response::Connected),
            Some("ERR") => match segments.next() {
                Some(message) => Err(Error::SonicServer(String::from(message))),
                _ => Err(Error::WrongResponse),
            },
            _ => Err(Error::WrongResponse),
        }
    }

    pub(crate) fn run_command<SC: StreamCommand>(&self, command: SC) -> Result<SC::Response> {
        self.write(&command)?;
        let res = loop {
            let res = self.read_line()?;
            if !matches!(&res, protocol::Response::Pending(_)) {
                break res;
            }
        };
        command.receive(res)
    }

    fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream = TcpStream::connect(addr).map_err(|_| Error::ConnectToServer)?;
        let read_stream = stream.try_clone().map_err(|_| Error::ConnectToServer)?;

        let channel = SonicStream {
            reader: RefCell::new(BufReader::new(read_stream)),
            stream: RefCell::new(stream),
            mode: None,
            max_buffer_size: UNINITIALIZED_MODE_MAX_BUFFER_SIZE,
            protocol_version: DEFAULT_SONIC_PROTOCOL_VERSION,
        };

        let res = channel.read_line()?;
        if matches!(res, protocol::Response::Connected) {
            Ok(channel)
        } else {
            Err(Error::ConnectToServer)
        }
    }

    fn start<S: ToString>(&mut self, mode: ChannelMode, password: S) -> Result<()> {
        if self.mode.is_some() {
            return Err(Error::RunCommand);
        }

        let command = StartCommand {
            mode,
            password: password.to_string(),
        };
        let response = self.run_command(command)?;

        self.max_buffer_size = response.max_buffer_size;
        self.protocol_version = response.protocol_version;
        self.mode = Some(response.mode);

        Ok(())
    }

    /// Connect to the search backend in chosen mode.
    ///
    /// I think we shouldn't separate commands connect and start because we haven't
    /// possibility to change channel in sonic server, if we already chosen one of them. 🤔
    ///
    /// ```rust,no_run
    /// use sonic_channel::*;
    ///
    /// fn main() -> result::Result<()> {
    ///     let channel = SearchChannel::start(
    ///         "localhost:1491",
    ///         "SecretPassword"
    ///     )?;
    ///
    ///     // Now you can use all method of Search channel.
    ///     let objects = channel.query("search", "default", "beef");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub(crate) fn connect_with_start<A, S>(mode: ChannelMode, addr: A, password: S) -> Result<Self>
    where
        A: ToSocketAddrs,
        S: ToString,
    {
        let mut channel = Self::connect(addr)?;
        channel.start(mode, password)?;
        Ok(channel)
    }
}

/// This trait should be implemented for all supported sonic channels
pub trait SonicChannel {
    /// Sonic channel struct
    type Channel;

    /// Returns reference for sonic stream of connection
    fn stream(&self) -> &SonicStream;

    /// Connects to sonic backend and run start command.
    ///
    /// ```rust,no_run
    /// # use sonic_channel::*;
    /// # fn main() -> result::Result<()> {
    /// let search_channel = SearchChannel::start(
    ///     "localhost:1491",
    ///     "SecretPassword",
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    fn start<A, S>(addr: A, password: S) -> Result<Self::Channel>
    where
        A: ToSocketAddrs,
        S: ToString;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_channel_enums() {
        assert_eq!(format!("{}", ChannelMode::Search), String::from("search"));
        assert_eq!(format!("{}", ChannelMode::Ingest), String::from("ingest"));
        assert_eq!(format!("{}", ChannelMode::Control), String::from("control"));
    }
}
