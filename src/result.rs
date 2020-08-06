use std::error::Error as StdError;
use std::fmt;

/// Sugar if you expect only sonic-channel error type in result
pub type Result<T> = std::result::Result<T, Error>;

/// Wrap for sonic channel error kind. This type has std::error::Error
/// implementation and you can use boxed trait for catch other errors
/// like this.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl StdError for Error {}

impl Error {
    /// Creates new Error with sonic channel error kind
    ///
    /// ```rust
    /// use sonic_channel::result::*;
    ///
    /// let err = Error::new(ErrorKind::ConnectToServer);
    /// ```
    pub fn new(kind: ErrorKind) -> Self {
        Error { kind }
    }
}

/// All error kinds that you can see in sonic-channel crate.
#[derive(Debug)]
pub enum ErrorKind {
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
    QueryResponseError(&'static str),

    /// Response from sonic server are wrong! Actually it may happen if you use 
    /// unsupported sonic backend version. Please write issue to the github repo.
    WrongSonicResponse,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        match self.kind {
            ErrorKind::ConnectToServer => write!(f, "Cannot connect to server"),
            ErrorKind::WriteToStream => write!(f, "Cannot write data to stream"),
            ErrorKind::ReadStream => write!(f, "Cannot read sonic response from stream"),
            ErrorKind::SwitchMode => write!(f, "Cannot switch channel mode"),
            ErrorKind::RunCommand => write!(f, "Cannot run command in current mode"),
            ErrorKind::QueryResponseError(message) => {
                write!(f, "Error in query response: {}", message)
            }
            ErrorKind::WrongSonicResponse => {
                write!(f, "Sonic response are wrong. Please write issue to github.")
            }
        }
    }
}
