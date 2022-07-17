use super::StreamCommand;
use crate::channels::ChannelMode;
use crate::protocol;
use crate::result::*;

#[derive(Debug)]
pub struct StartCommand {
    pub(crate) mode: ChannelMode,
    pub(crate) password: String,
}

#[derive(Debug)]
pub struct StartCommandResponse {
    pub protocol_version: protocol::Version,
    pub max_buffer_size: usize,
    pub mode: ChannelMode,
}

impl StreamCommand for StartCommand {
    type Response = StartCommandResponse;

    fn request(&self) -> protocol::Request {
        protocol::Request::Start {
            mode: self.mode,
            password: self.password.to_string(),
        }
    }

    fn receive(&self, res: protocol::Response) -> Result<Self::Response> {
        if let protocol::Response::Started(payload) = res {
            Ok(StartCommandResponse {
                protocol_version: payload
                    .protocol_version
                    .try_into()
                    // TODO: better error
                    .map_err(|_| Error::SwitchMode)?,
                max_buffer_size: payload.max_buffer_size,
                mode: self.mode,
            })
        } else {
            Err(Error::SwitchMode)
        }
    }
}
