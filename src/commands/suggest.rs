use super::StreamCommand;
use crate::protocol;
use crate::result::*;

#[derive(Debug, Default)]
pub struct SuggestCommand<'a> {
    pub collection: &'a str,
    pub bucket: &'a str,
    pub word: &'a str,
    pub limit: Option<usize>,
}

impl StreamCommand for SuggestCommand<'_> {
    type Response = Vec<String>;

    fn request(&self) -> protocol::Request {
        protocol::Request::Suggest {
            collection: self.collection.to_string(),
            bucket: self.bucket.to_string(),
            word: self.word.to_string(),
            limit: self.limit,
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
