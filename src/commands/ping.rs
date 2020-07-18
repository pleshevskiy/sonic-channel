use super::StreamCommand;
use crate::errors::SonicError;

#[derive(Debug, Default)]
pub struct PingCommand;

impl StreamCommand for PingCommand {
    type Response = bool;

    fn message(&self) -> String {
        String::from("PING\r\n")
    }

    fn receive(&self, message: String) -> Result<<Self as StreamCommand>::Response, SonicError> {
        dbg!(&message);
        Ok(message == "PONG\r\n")
    }
}
