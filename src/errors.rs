use std::fmt;


#[derive(Debug)]
pub enum SonicError {
    ConnectToServer,
    WriteToStream,
    ReadStream,
    SwitchMode,
    RunCommand,
    QueryResponseError(&'static str),
}

impl fmt::Display for SonicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let message = match self {
            SonicError::ConnectToServer => String::from("Cannot connect to server"),
            SonicError::WriteToStream => String::from("Cannot write data to stream"),
            SonicError::ReadStream => String::from("Cannot read sonic response from stream"),
            SonicError::SwitchMode => String::from("Cannot switch channel mode"),
            SonicError::RunCommand => String::from("Cannot run command in current mode"),
            SonicError::QueryResponseError(message) => format!("Error in query response: {}", message)
        };

        write!(f, "{}", message)
    }
}
