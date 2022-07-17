use super::StreamCommand;
use crate::misc::OptDest;
use crate::protocol;
use crate::result::*;

#[derive(Debug)]
pub struct CountRequest(OptDest);

impl CountRequest {
    pub fn buckets(collection: impl ToString) -> CountRequest {
        Self(OptDest::col(collection))
    }

    pub fn objects(collection: impl ToString, bucket: impl ToString) -> CountRequest {
        Self(OptDest::col_buc(collection, bucket))
    }

    pub fn words(
        collection: impl ToString,
        bucket: impl ToString,
        object: impl ToString,
    ) -> CountRequest {
        Self(OptDest::col_buc_obj(collection, bucket, object))
    }
}

#[derive(Debug)]
pub struct CountCommand {
    pub(crate) req: CountRequest,
}

impl StreamCommand for CountCommand {
    type Response = usize;

    fn request(&self) -> protocol::Request {
        let dest = &self.req.0;
        protocol::Request::Count {
            collection: dest.collection.clone(),
            bucket: dest.bucket.clone(),
            object: dest.object.clone(),
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
