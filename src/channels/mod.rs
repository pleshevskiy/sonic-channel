#[cfg(feature = "search")]
mod search;
#[cfg(feature = "search")]
use crate::commands::StartCommand;
pub use search::*;

#[cfg(feature = "ingest")]
mod ingest;
#[cfg(feature = "ingest")]
pub use ingest::*;

#[cfg(feature = "control")]
mod control;
#[cfg(feature = "control")]
pub use control::*;

use crate::commands::StreamCommand;
use crate::result::*;
use std::fmt;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

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
    pub fn to_str(&self) -> &str {
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

impl fmt::Display for ChannelMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(f, "{}", self.to_str())
    }
}

/// Root and Heart of this library.
///
/// You can connect to the sonic search backend and run all supported protocol methods.
///
#[derive(Debug)]
pub struct SonicStream {
    stream: TcpStream,
    mode: Option<ChannelMode>, // None – Uninitialized mode
    max_buffer_size: usize,
    protocol_version: usize,
}

impl SonicStream {
    fn write<SC: StreamCommand>(&self, command: &SC) -> Result<()> {
        let mut writer = BufWriter::with_capacity(self.max_buffer_size, &self.stream);
        let message = command.message();
        writer
            .write_all(message.as_bytes())
            .map_err(|_| Error::new(ErrorKind::WriteToStream))?;
        Ok(())
    }

    fn read(&self, max_read_lines: usize) -> Result<String> {
        let mut reader = BufReader::with_capacity(self.max_buffer_size, &self.stream);
        let mut message = String::new();

        let mut lines_read = 0;
        while lines_read < max_read_lines {
            reader
                .read_line(&mut message)
                .map_err(|_| Error::new(ErrorKind::ReadStream))?;
            lines_read += 1;
        }

        Ok(message)
    }

    pub(crate) fn run_command<SC: StreamCommand>(&self, command: SC) -> Result<SC::Response> {
        self.write(&command)?;
        let message = self.read(SC::READ_LINES_COUNT)?;
        command.receive(message)
    }

    fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream =
            TcpStream::connect(addr).map_err(|_| Error::new(ErrorKind::ConnectToServer))?;

        let channel = SonicStream {
            stream,
            mode: None,
            max_buffer_size: UNINITIALIZED_MODE_MAX_BUFFER_SIZE,
            protocol_version: DEFAULT_SONIC_PROTOCOL_VERSION,
        };

        let message = channel.read(1)?;
        // TODO: need to add support for versions
        if message.starts_with("CONNECTED") {
            Ok(channel)
        } else {
            Err(Error::new(ErrorKind::ConnectToServer))
        }
    }

    fn start<S: ToString>(&mut self, mode: ChannelMode, password: S) -> Result<()> {
        if self.mode.is_some() {
            return Err(Error::new(ErrorKind::RunCommand));
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
