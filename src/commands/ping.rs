use super::StreamCommand;
use crate::protocol;
use crate::result::*;

#[derive(Debug, Default)]
pub struct PingCommand;

impl StreamCommand for PingCommand {
    type Response = ();

    fn format(&self) -> String {
        String::from("PING\r\n")
    }

    fn send(&self) -> protocol::Request {
        protocol::Request::Ping
    }

    fn receive(&self, res: protocol::Response) -> Result<Self::Response> {
        if matches!(res, protocol::Response::Pong) {
            Ok(())
        } else {
            Err(Error::WrongResponse)
        }
    }
}
