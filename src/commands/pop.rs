use super::StreamCommand;
use crate::misc::ObjDest;
use crate::protocol;
use crate::result::*;

/// Parameters for the `pop` command.
#[derive(Debug)]
pub struct PopRequest {
    /// Collection, bucket and object where we should pop search data from index.
    pub dest: ObjDest,
    /// Search data to be deleted
    pub text: String,
}

impl PopRequest {
    /// Creates a base pop request.
    pub fn new(dest: ObjDest, text: impl ToString) -> Self {
        Self {
            dest,
            text: text.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct PopCommand {
    pub(crate) req: PopRequest,
}

impl StreamCommand for PopCommand {
    type Response = usize;

    fn request(&self) -> protocol::Request {
        let dest = &self.req.dest;
        protocol::Request::Pop {
            collection: dest.collection().clone(),
            bucket: dest
                .bucket_opt()
                .cloned()
                // TODO: use a global context for default bucket value
                .unwrap_or_else(|| String::from("default")),
            object: dest.object().clone(),
            terms: self.req.text.to_string(),
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
