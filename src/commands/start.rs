use super::StreamCommand;
use crate::channels::ChannelMode;
use crate::protocol;
use crate::result::*;

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

    fn format(&self) -> String {
        format!("START {} {}\r\n", self.mode, self.password)
    }

    fn send(&self) -> protocol::Request {
        protocol::Request::Start {
            mode: self.mode,
            password: self.password,
        }
    }

    fn receive(&self, res: protocol::Response) -> Result<Self::Response> {
        if let protocol::Response::Started(res) = res {
            Ok(StartCommandResponse {
                protocol_version: res.protocol_version,
                max_buffer_size: res.max_buffer_size,
                mode: self.mode,
            })
        } else {
            Err(Error::SwitchMode)
        }
    }
}
