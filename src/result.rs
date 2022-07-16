use crate::channels::ChannelMode;

/// Sugar if you expect only sonic-channel error type in result
pub type Result<T> = std::result::Result<T, Error>;

/// Wrap for sonic channel error kind. This type has std::error::Error
/// implementation and you can use boxed trait for catch other errors
/// like this.

/// All error kinds that you can see in sonic-channel crate.
#[derive(Debug)]
pub enum Error {
    /// Cannot connect to the sonic search backend.
    ConnectToServer,

    /// Cannot write message to stream.
    WriteToStream,

    /// Cannot read message in stream.
    ReadStream,

    /// Cannot switch channel mode from uninitialized.
    SwitchMode,

    /// Cannot run command in current mode.
    RunCommand,

    /// Error in query response with additional message.
    QueryResponse(&'static str),

    /// Response from sonic server are wrong! Actually it may happen if you use
    /// unsupported sonic backend version. Please write issue to the github repo.
    WrongResponse,

    /// You cannot run the command in current channel.
    UnsupportedCommand((&'static str, Option<ChannelMode>)),

    /// This error appears if the error occurred on the server side
    SonicServer(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            ConnectToServer => f.write_str("Cannot connect to server"),
            WriteToStream => f.write_str("Cannot write data to stream"),
            ReadStream => f.write_str("Cannot read sonic response from stream"),
            SwitchMode => f.write_str("Cannot switch channel mode"),
            RunCommand => f.write_str("Cannot run command in current mode"),
            QueryResponse(message) => {
                write!(f, "Error in query response: {}", message)
            }
            WrongResponse => {
                write!(f, "Client cannot parse response from sonic server. Please write an issue to github (https://github.com/pleshevskiy/sonic-channel).")
            }
            UnsupportedCommand((command_name, channel_mode)) => {
                if let Some(channel_mode) = channel_mode {
                    write!(
                        f,
                        "You cannot use `{}` command in {} sonic channel mode",
                        command_name, channel_mode
                    )
                } else {
                    write!(
                        f,
                        "You need to connect to sonic channel before use {} command",
                        command_name
                    )
                }
            }
            SonicServer(message) => write!(f, "Sonic Server-side error: {}", message),
        }
    }
}

impl std::error::Error for Error {}
