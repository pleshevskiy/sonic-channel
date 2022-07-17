use super::StreamCommand;
use crate::misc::Dest;
use crate::protocol;
use crate::result::*;

#[derive(Debug)]
pub struct CountRequest {
    pub dest: Dest,
    pub obj: Option<String>,
}

impl CountRequest {
    pub fn buckets(collection: impl ToString) -> CountRequest {
        Self {
            dest: Dest::col(collection),
            obj: None,
        }
    }

    pub fn objects(collection: impl ToString, bucket: impl ToString) -> CountRequest {
        Self {
            dest: Dest::col_buc(collection, bucket),
            obj: None,
        }
    }

    pub fn words(
        collection: impl ToString,
        bucket: impl ToString,
        object: impl ToString,
    ) -> CountRequest {
        Self {
            dest: Dest::col_buc(collection, bucket),
            obj: Some(object.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct CountCommand {
    pub req: CountRequest,
}

impl StreamCommand for CountCommand {
    type Response = usize;

    fn request(&self) -> protocol::Request {
        let req = self.req;
        protocol::Request::Count {
            collection: *req.dest.collection(),
            bucket: req.dest.bucket().cloned(),
            object: req.obj,
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
