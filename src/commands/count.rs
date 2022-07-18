use super::StreamCommand;
use crate::misc::*;
use crate::protocol;
use crate::result::*;

/// Parameters for the `count` command.
#[derive(Debug)]
pub struct CountRequest(OptDest);

impl CountRequest {
    /// Creates a new request to get the number of buckets in the collection.
    pub fn buckets(collection: impl ToString) -> CountRequest {
        Self(OptDest::col(collection))
    }

    /// Creates a new request to get the number of objects in the collection bucket.
    pub fn objects(collection: impl ToString, bucket: impl ToString) -> CountRequest {
        Self(OptDest::col_buc(collection, bucket))
    }

    /// Creates a new request to get the number of words in the collection bucket object.
    pub fn words(
        collection: impl ToString,
        bucket: impl ToString,
        object: impl ToString,
    ) -> CountRequest {
        Self(OptDest::col_buc_obj(collection, bucket, object))
    }
}

impl From<Dest> for CountRequest {
    fn from(d: Dest) -> Self {
        Self(OptDest::from(d))
    }
}

impl From<ObjDest> for CountRequest {
    fn from(d: ObjDest) -> Self {
        Self(OptDest::from(d))
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
