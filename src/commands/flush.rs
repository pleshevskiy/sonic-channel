use super::StreamCommand;
use crate::misc::OptDest;
use crate::protocol;
use crate::result::*;

#[derive(Debug)]
pub struct FlushRequest(OptDest);

impl FlushRequest {
    pub fn collection(collection: impl ToString) -> FlushRequest {
        Self(OptDest::col(collection))
    }

    pub fn bucket(collection: impl ToString, bucket: impl ToString) -> FlushRequest {
        Self(OptDest::col_buc(collection, bucket))
    }

    pub fn object(
        collection: impl ToString,
        bucket: impl ToString,
        object: impl ToString,
    ) -> FlushRequest {
        Self(OptDest::col_buc_obj(collection, bucket, object))
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
