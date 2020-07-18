use super::StreamCommand;
use crate::errors::SonicError;

#[derive(Debug, Default)]
pub struct QuitCommand;

impl StreamCommand for QuitCommand {
    type Response = bool;

    fn message(&self) -> String {
        String::from("QUIT\r\n")
    }

    fn receive(&self, message: String) -> Result<<Self as StreamCommand>::Response, SonicError> {
        dbg!(&message);
        Ok(message.starts_with("ENDED "))
    }
}
