use super::StreamCommand;
use crate::protocol;
use crate::result::*;

#[derive(Debug, Default)]
pub struct PopCommand<'a> {
    pub collection: &'a str,
    pub bucket: &'a str,
    pub object: &'a str,
    pub text: &'a str,
}

impl StreamCommand for PopCommand<'_> {
    type Response = usize;

    fn format(&self) -> String {
        let mut message = format!(
            r#"POP {} {} {} "{}""#,
            self.collection, self.bucket, self.object, self.text
        );
        message.push_str("\r\n");
        message
    }

    fn receive(&self, res: protocol::Response) -> Result<Self::Response> {
        if let protocol::Response::Result(count) = res {
            Ok(count)
        } else {
            Err(Error::WrongResponse)
        }
    }
}
