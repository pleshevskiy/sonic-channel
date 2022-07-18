use super::StreamCommand;
use crate::misc::*;
use crate::protocol;
use crate::result::*;

/// Parameters for the `flush` command.
#[derive(Debug)]
pub struct FlushRequest(OptDest);

impl FlushRequest {
    /// Creates a new request to flush all data in the collection.
    pub fn collection(collection: impl ToString) -> FlushRequest {
        Self(OptDest::col(collection))
    }

    /// Creates a new request to flush all data in the collection bucket.
    pub fn bucket(collection: impl ToString, bucket: impl ToString) -> FlushRequest {
        Self(OptDest::col_buc(collection, bucket))
    }

    /// Creates a new request to flush all data in the collection bucket object.
    pub fn object(
        collection: impl ToString,
        bucket: impl ToString,
        object: impl ToString,
    ) -> FlushRequest {
        Self(OptDest::col_buc_obj(collection, bucket, object))
    }
}

impl From<Dest> for FlushRequest {
    fn from(d: Dest) -> Self {
        Self(OptDest::from(d))
    }
}

impl From<ObjDest> for FlushRequest {
    fn from(d: ObjDest) -> Self {
        Self(OptDest::from(d))
    }
}

#[derive(Debug)]
pub struct FlushCommand {
    pub(crate) req: FlushRequest,
}

impl StreamCommand for FlushCommand {
    type Response = usize;

    fn request(&self) -> protocol::Request {
        let dest = &self.req.0;

        protocol::Request::Flush {
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
