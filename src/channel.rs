use crate::commands::*;
use crate::result::*;
use std::fmt;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

const DEFAULT_SONIC_PROTOCOL_VERSION: usize = 1;
const MAX_LINE_BUFFER_SIZE: usize = 20000;
const UNINITIALIZED_MODE_MAX_BUFFER_SIZE: usize = 200;
const BUFFER_LINE_SEPARATOR: u8 = '\n' as u8;

macro_rules! init_commands {
    (
        $(
            $(#[$outer:meta])*
            use $cmd_name:ident
            for fn $fn_name:ident
            $(<$($lt:lifetime)+>)? (
                $($args:tt)*
            );
        )*
    ) => {
        $(
            init_commands!(
                $(#[$outer])*
                use $cmd_name
                for fn $fn_name $(<$($lt)+>)? (
                    $($args)*
                )
            );
        )*
    };

    (
        $(#[$outer:meta])*
        use $cmd_name:ident
        for fn $fn_name:ident $(<$($lt:lifetime)+>)? (
            $($arg_name:ident : $arg_type:ty $( => $arg_value:expr)?,)*
        )
    ) => {
        $(#[$outer])*
        pub fn $fn_name $(<$($lt)+>)? (
            &self,
            $($arg_name: $arg_type),*
        ) -> $crate::result::Result<
            <$cmd_name as $crate::commands::StreamCommand>::Response,
        > {
            let command = $cmd_name { $($arg_name $(: $arg_value)?,)* ..Default::default() };
            self.run_command(command)
        }
    };
}

/// Channel modes supported by sonic search backend.
#[derive(Debug, Clone, Copy)]
pub enum ChannelMode {
    /// Search mode
    #[cfg(feature = "search")]
    Search,

    /// Ingest mode
    #[cfg(feature = "ingest")]
    Ingest,

    /// Control mode
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
pub struct SonicChannel {
    stream: TcpStream,
    mode: Option<ChannelMode>, // None – Uninitialized mode
    max_buffer_size: usize,
    protocol_version: usize,
}

impl SonicChannel {
    fn write<SC: StreamCommand>(&self, command: &SC) -> Result<()> {
        let mut writer = BufWriter::with_capacity(self.max_buffer_size, &self.stream);
        let message = command.message();
        dbg!(&message);
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

    fn run_command<SC: StreamCommand>(&self, command: SC) -> Result<SC::Response> {
        self.write(&command)?;
        let message = self.read(SC::READ_LINES_COUNT)?;
        command.receive(message)
    }

    fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream =
            TcpStream::connect(addr).map_err(|_| Error::new(ErrorKind::ConnectToServer))?;

        let channel = SonicChannel {
            stream,
            mode: None,
            max_buffer_size: UNINITIALIZED_MODE_MAX_BUFFER_SIZE,
            protocol_version: DEFAULT_SONIC_PROTOCOL_VERSION,
        };

        let message = channel.read(1)?;
        dbg!(&message);
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
    /// ```rust
    /// use sonic_channel::*;
    /// 
    /// SonicChannel::connect_with_start(ChannelMode::Search, "localhost:1491", "SecretPassword");
    /// ```
    pub fn connect_with_start<A, S>(mode: ChannelMode, addr: A, password: S) -> Result<Self>
    where
        A: ToSocketAddrs,
        S: ToString,
    {
        let mut channel = Self::connect(addr)?;
        channel.start(mode, password)?;
        Ok(channel)
    }

    // TODO: add examples for commands.
    init_commands! {
        #[doc="Stop connection."]
        use QuitCommand for fn quit();

        #[doc="Ping server."]
        use PingCommand for fn ping();
    }

    #[cfg(feature = "ingest")]
    init_commands! {
        #[doc="Push search data in the index."]
        use PushCommand for fn push<'a>(
            collection: &'a str,
            bucket: &'a str,
            object: &'a str,
            text: &'a str,
        );

        #[doc="Push search data in the index with locale parameter in ISO 639-3 code."]
        use PushCommand for fn push_with_locale<'a>(
            collection: &'a str,
            bucket: &'a str,
            object: &'a str,
            text: &'a str,
            locale: &'a str => Some(locale),
        );

        #[doc="Flush all indexed data from collections."]
        use FlushCommand for fn flushc<'a>(
            collection: &'a str,
        );

        #[doc="Flush all indexed data from bucket in a collection."]
        use FlushCommand for fn flushb<'a>(
            collection: &'a str,
            bucket: &'a str => Some(bucket),
        );

        #[doc="Flush all indexed data from an object in a bucket in collection."]
        use FlushCommand for fn flusho<'a>(
            collection: &'a str,
            bucket: &'a str => Some(bucket),
            object: &'a str => Some(object),
        );
    }

    #[cfg(feature = "search")]
    init_commands! {
        #[doc="Query objects in database."]
        use QueryCommand for fn query<'a>(
            collection: &'a str,
            bucket: &'a str,
            terms: &'a str,
        );

        #[doc="Query limited objects in database."]
        use QueryCommand for fn query_with_limit<'a>(
            collection: &'a str,
            bucket: &'a str,
            terms: &'a str,
            limit: usize => Some(limit),
        );

        #[doc="Query limited objects with offset in database."]
        use QueryCommand for fn query_with_limit_and_offset<'a>(
            collection: &'a str,
            bucket: &'a str,
            terms: &'a str,
            limit: usize => Some(limit),
            offset: usize => Some(offset),
        );

        #[doc="Suggest auto-completes words."]
        use SuggestCommand for fn suggest<'a>(
            collection: &'a str,
            bucket: &'a str,
            word: &'a str,
        );

        #[doc="Suggest auto-completes words with limit."]
        use SuggestCommand for fn suggest_with_limit<'a>(
            collection: &'a str,
            bucket: &'a str,
            word: &'a str,
            limit: usize => Some(limit),
        );
    }

    #[cfg(feature = "control")]
    init_commands! {}
}
