use super::StreamCommand;
use crate::channel::ChannelMode;
use crate::errors::SonicError;
use regex::Regex;

const RE_START_RECEIVED_MESSAGE: &str = r"(?x)
    STARTED 
    \s # started with mode
    (?P<mode>search|ingest|control) 
    \s # wich protocol used
    protocol\((?P<protocol>\d+)\) 
    \s # maximum buffer size
    buffer\((?P<buffer_size>\d+)\)
";

#[derive(Debug)]
pub struct StartCommand {
    pub mode: ChannelMode,
    pub password: String,
}

pub struct StartCommandResponse {
    pub protocol_version: usize,
    pub max_buffer_size: usize,
    pub mode: ChannelMode,
}

impl StreamCommand for StartCommand {
    type Response = StartCommandResponse;

    fn message(&self) -> String {
        format!("START {} {}\r\n", self.mode, self.password)
    }

    fn receive(&self, message: String) -> Result<Self::Response, SonicError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(RE_START_RECEIVED_MESSAGE).unwrap();
        }

        dbg!(&message);

        match RE.captures(&message) {
            None => Err(SonicError::SwitchMode),
            Some(caps) => {
                if self.mode.to_str() != &caps["mode"] {
                    return Err(SonicError::SwitchMode);
                }

                let protocol_version: usize =
                    caps["protocol"].parse().expect("Must be digit by regex");
                let max_buffer_size: usize =
                    caps["buffer_size"].parse().expect("Must be digit by regex");

                Ok(StartCommandResponse {
                    protocol_version,
                    max_buffer_size,
                    mode: self.mode,
                })
            }
        }
    }
}
