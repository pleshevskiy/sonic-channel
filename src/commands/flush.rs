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

    fn request(&self) -> protocol::Request {
        protocol::Request::Flush {
            collection: self.collection.to_string(),
            bucket: self.bucket.map(String::from),
            object: self.object.map(String::from),
        }
    }

    fn receive(&self, res: protocol::Response) -> Result<Self::Response> {
        if let protocol::Response::Result(count) = res {
            Ok(count)
        } else {
            Err(Error::WrongResponse)
        }
    }
}
