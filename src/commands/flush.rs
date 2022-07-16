use super::StreamCommand;
use crate::protocol;
use crate::result::*;

#[derive(Debug, Default)]
pub struct FlushCommand<'a> {
    pub collection: &'a str,
    pub bucket: Option<&'a str>,
    pub object: Option<&'a str>,
}

impl StreamCommand for FlushCommand<'_> {
    type Response = usize;

    fn format(&self) -> String {
        let mut message = match (self.bucket, self.object) {
            (Some(bucket), Some(object)) => {
                format!("FLUSHO {} {} {}", self.collection, bucket, object)
            }
            (Some(bucket), None) => format!("FLUSHB {} {}", self.collection, bucket),
            (None, None) => format!("FLUSHC {}", self.collection),
            _ => panic!("Invalid flush command"),
        };
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
