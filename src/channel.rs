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
            use $cmd_name:ident
            for fn $fn_name:ident
            $(<$($lt:lifetime)+>)?
            ($($args:tt)*)
            ;
        )*
    ) => {
        $(init_commands!(use $cmd_name for fn $fn_name $(<$($lt)+>)? ($($args)*));)*
    };

    (use $cmd_name:ident for fn $fn_name:ident $(<$($lt:lifetime)+>)? ($($arg_name:ident : $arg_type:ty,)*)) => {
        pub fn $fn_name $(<$($lt)+>)? (
            &self,
            $($arg_name: $arg_type),*
        ) -> crate::result::Result<
            <$cmd_name as crate::commands::StreamCommand>::Response,
        > {
            let command = $cmd_name { $($arg_name,)* ..Default::default() };
            self.run_command(command)
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub enum ChannelMode {
    #[cfg(feature = "search")]
    Search,

    #[cfg(feature = "ingest")]
    Ingest,

    #[cfg(feature = "control")]
    Control,
}

impl ChannelMode {
    pub fn to_str(&self) -> &str {
        #[cfg(any(feature = "ingest", feature = "search", feature = "control"))]
        match self {
            #[cfg(feature = "search")]
            ChannelMode::Search => "search",

            #[cfg(feature = "ingest")]
            ChannelMode::Ingest => "ingest",

            #[cfg(feature = "control")]
            ChannelMode::Control => "control",
        }

        // Actually we'll not see this text because we cannot call this function for enum
        // without enum value, but Rust compiler want this case.
        #[cfg(all(
            not(feature = "ingest"),
            not(feature = "search"),
            not(feature = "control")
        ))]
        "unitialized"
    }
}

impl fmt::Display for ChannelMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug)]
pub struct SonicChannel {
    stream: TcpStream,
    mode: Option<ChannelMode>, // None – Uninitialized mode
    max_buffer_size: usize,
    protocol_version: usize,
}

impl SonicChannel {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream = TcpStream::connect(addr).map_err(|_| Error::new(ErrorKind::ConnectToServer))?;

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

    pub fn run_command<SC: StreamCommand>(&self, command: SC) -> Result<SC::Response> {
        self.write(&command)?;
        let message = self.read(SC::READ_LINES_COUNT)?;
        command.receive(message)
    }

    #[cfg(any(feature = "ingest", feature = "search", feature = "control"))]
    pub fn start<S: ToString>(&mut self, mode: ChannelMode, password: S) -> Result<()> {
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

    init_commands! {
        use QuitCommand for fn quit();
    }

    #[cfg(any(feature = "ingest", feature = "search", feature = "control"))]
    init_commands! {
        use PingCommand for fn ping();
    }

    #[cfg(feature = "ingest")]
    init_commands! {
        use PushCommand for fn push<'a>(
            collection: &'a str,
            bucket: &'a str,
            object: &'a str,
            text: &'a str,
        );

        use PushCommand for fn push_with_locale<'a>(
            collection: &'a str,
            bucket: &'a str,
            object: &'a str,
            text: &'a str,
            locale: Option<&'a str>,
        );
    }

    #[cfg(feature = "search")]
    init_commands! {
        use QueryCommand for fn query<'a>(
            collection: &'a str,
            bucket: &'a str,
            terms: &'a str,
        );

        use QueryCommand for fn query_with_limit<'a>(
            collection: &'a str,
            bucket: &'a str,
            terms: &'a str,
            limit: Option<usize>,
        );

        use QueryCommand for fn query_with_limit_and_offset<'a>(
            collection: &'a str,
            bucket: &'a str,
            terms: &'a str,
            limit: Option<usize>,
            offset: Option<usize>,
        );
    }

    #[cfg(feature = "control")]
    init_commands! {}
}
