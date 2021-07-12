use super::StreamCommand;
use crate::channels::ChannelMode;
use crate::result::*;
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

#[derive(Debug)]
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

    fn receive(&self, message: String) -> Result<Self::Response> {
        lazy_static! {
            static ref RE: Regex = Regex::new(RE_START_RECEIVED_MESSAGE).unwrap();
        }

        if let Some(caps) = RE.captures(&message) {
            if self.mode.as_str() != &caps["mode"] {
                Err(Error::new(ErrorKind::SwitchMode))
            } else {
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
        } else {
            Err(Error::new(ErrorKind::SwitchMode))
        }
    }
}
