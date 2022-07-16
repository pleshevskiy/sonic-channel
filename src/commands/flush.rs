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
        let collection = self.collection.to_string();
        let req = match (self.bucket.map(String::from), self.object.map(String::from)) {
            (Some(bucket), Some(object)) => protocol::FlushRequest::Object {
                collection,
                bucket,
                object,
            },
            (Some(bucket), None) => protocol::FlushRequest::Bucket { collection, bucket },
            (None, None) => protocol::FlushRequest::Collection(collection),
            _ => panic!("Invalid flush command"),
        };

        protocol::Request::Flush(req)
    }

    fn receive(&self, res: protocol::Response) -> Result<Self::Response> {
        if let protocol::Response::Result(count) = res {
            Ok(count)
        } else {
            Err(Error::WrongResponse)
        }
    }
}
