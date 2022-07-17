use super::StreamCommand;
use crate::protocol;
use crate::result::*;

#[derive(Debug)]
pub struct QuitCommand;

impl StreamCommand for QuitCommand {
    type Response = ();

    fn request(&self) -> protocol::Request {
        protocol::Request::Quit
    }

    fn receive(&self, res: protocol::Response) -> Result<Self::Response> {
        if matches!(res, protocol::Response::Ended) {
            Ok(())
        } else {
            Err(Error::WrongResponse)
        }
    }
}
