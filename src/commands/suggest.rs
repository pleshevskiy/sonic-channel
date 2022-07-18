use super::StreamCommand;
use crate::misc::Dest;
use crate::protocol;
use crate::result::*;

/// Parameters for the `suggest` command.
#[derive(Debug)]
pub struct SuggestRequest {
    /// Collection and bucket where we should search for suggested words.
    pub dest: Dest,
    /// Base word.
    pub word: String,
    /// Limit of result words.
    pub limit: Option<usize>,
}

impl SuggestRequest {
    /// Creates a base suggest request.
    pub fn new(dest: Dest, word: impl ToString) -> Self {
        Self {
            dest,
            word: word.to_string(),
            limit: None,
        }
    }

    /// Set a limit for the request.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug)]
pub struct SuggestCommand {
    pub(crate) req: SuggestRequest,
}

impl StreamCommand for SuggestCommand {
    type Response = Vec<String>;

    fn request(&self) -> protocol::Request {
        let dest = &self.req.dest;

        protocol::Request::Suggest {
            collection: dest.collection().clone(),
            bucket: dest
                .bucket_opt()
                .cloned()
                .unwrap_or_else(|| String::from("default")),
            word: self.req.word.to_string(),
            limit: self.req.limit,
        }
    }

    fn receive(&self, res: protocol::Response) -> Result<Self::Response> {
        if let protocol::Response::Event(protocol::EventKind::Suggest, _id, words) = res {
            Ok(words)
        } else {
            Err(Error::WrongResponse)
        }
    }
}
