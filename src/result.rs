use std::error::Error as StdError;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl StdError for Error {}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error { kind }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    ConnectToServer,
    WriteToStream,
    ReadStream,
    SwitchMode,
    RunCommand,
    QueryResponseError(&'static str),
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
        }
    }
}
