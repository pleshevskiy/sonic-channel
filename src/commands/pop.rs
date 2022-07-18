use super::StreamCommand;
use crate::misc::ObjDest;
use crate::protocol;
use crate::result::*;

#[derive(Debug)]
pub struct PopRequest {
    pub dest: ObjDest,
    pub text: String,
}

impl PopRequest {
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
