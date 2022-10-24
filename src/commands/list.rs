use super::StreamCommand;
use crate::misc::Dest;
use crate::protocol;
use crate::result::*;

/// Parameters for the `suggest` command.
#[derive(Debug)]
pub struct ListRequest {
    /// Collection and bucket where we should enumerate all words in index.
    pub dest: Dest,
    /// Limit of result words.
    pub limit: Option<usize>,
    /// Offset of result words.
    pub offset: Option<usize>,
}

impl ListRequest {
    /// Creates a base suggest request.
    pub fn new(dest: Dest) -> Self {
        Self {
            dest,
            limit: None,
            offset: None,
        }
    }

    /// Set a limit for the request.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set an offset for the request.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

#[derive(Debug)]
pub struct ListCommand {
    pub(crate) req: ListRequest,
}

impl StreamCommand for ListCommand {
    type Response = Vec<String>;

    fn request(&self) -> protocol::Request {
        let dest = &self.req.dest;

        protocol::Request::List {
            collection: dest.collection().clone(),
            bucket: dest
                .bucket_opt()
                .cloned()
                .unwrap_or_else(|| String::from("default")),
            limit: self.req.limit,
            offset: self.req.offset,
        }
    }

    fn receive(&self, res: protocol::Response) -> Result<Self::Response> {
        if let protocol::Response::Event(protocol::EventKind::List, _id, words) = res {
            Ok(words)
        } else {
            Err(Error::WrongResponse)
        }
    }
}
