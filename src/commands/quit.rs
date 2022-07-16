use super::StreamCommand;
use crate::protocol;
use crate::result::*;

#[derive(Debug, Default)]
pub struct QuitCommand;

impl StreamCommand for QuitCommand {
    type Response = ();

    fn format(&self) -> String {
        String::from("QUIT\r\n")
    }

    fn send(&self) -> protocol::Request {
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
