use super::StreamCommand;
use crate::protocol;
use crate::result::*;

#[derive(Debug)]
pub struct PingCommand;

impl StreamCommand for PingCommand {
    type Response = ();

    fn request(&self) -> protocol::Request {
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
