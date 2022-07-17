use super::StreamCommand;
use crate::misc::Dest;
use crate::protocol;
use crate::result::*;

#[derive(Debug)]
pub struct SuggestRequest<'a> {
    pub dest: Dest,
    pub word: &'a str,
}

#[derive(Debug)]
pub struct LimSuggestRequest<'a> {
    pub dest: Dest,
    pub word: &'a str,
    pub limit: Option<usize>,
}

impl<'a> From<SuggestRequest<'a>> for LimSuggestRequest<'a> {
    fn from(req: SuggestRequest<'a>) -> Self {
        Self {
            dest: req.dest,
            word: req.word,
            limit: None,
        }
    }
}

#[derive(Debug)]
pub struct SuggestCommand<'a> {
    pub(crate) req: LimSuggestRequest<'a>,
}

impl StreamCommand for SuggestCommand<'_> {
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
