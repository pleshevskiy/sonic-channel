use super::StreamCommand;
use crate::protocol;
use crate::result::*;

#[derive(Debug, Default)]
pub struct CountCommand<'a> {
    pub collection: &'a str,
    pub bucket: Option<&'a str>,
    pub object: Option<&'a str>,
}

impl StreamCommand for CountCommand<'_> {
    type Response = usize;

    fn format(&self) -> String {
        let mut message = format!("COUNT {}", self.collection);
        if let Some(bucket) = self.bucket {
            message.push_str(&format!(" {}", bucket));

            if let Some(object) = self.object {
                message.push_str(&format!(" {}", object));
            }
        }
        message.push_str("\r\n");
        message
    }

    fn send(&self) -> protocol::Request {
        protocol::Request::Count {
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
