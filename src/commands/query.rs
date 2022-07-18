use super::StreamCommand;
use crate::misc::Dest;
use crate::protocol;
use crate::result::*;

#[derive(Debug)]
pub struct QueryRequest {
    pub dest: Dest,
    pub terms: String,
    pub lang: Option<whatlang::Lang>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl QueryRequest {
    pub fn new(dest: Dest, terms: impl ToString) -> Self {
        Self {
            dest,
            terms: terms.to_string(),
            lang: None,
            limit: None,
            offset: None,
        }
    }

    pub fn lang(mut self, lang: whatlang::Lang) -> Self {
        self.lang = Some(lang);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn pag(self, offset: usize, limit: usize) -> Self {
        self.offset(offset).limit(limit)
    }
}

#[derive(Debug)]
pub struct QueryCommand {
    pub(crate) req: QueryRequest,
}

impl StreamCommand for QueryCommand {
    type Response = Vec<String>;

    fn request(&self) -> protocol::Request {
        let dest = &self.req.dest;
        let lang = self
            .req
            .lang
            .or_else(|| {
                whatlang::detect(&self.req.terms)
                    .and_then(|i| (i.confidence() == 1.0).then(|| i.lang()))
            })
            .map(|l| l.code());

        protocol::Request::Query {
            collection: dest.collection().clone(),
            bucket: dest
                .bucket_opt()
                .cloned()
                .unwrap_or_else(|| String::from("default")),
            terms: self.req.terms.clone(),
            offset: self.req.offset,
            limit: self.req.limit,
            lang,
        }
    }

    fn receive(&self, res: protocol::Response) -> Result<Self::Response> {
        if let protocol::Response::Event(protocol::EventKind::Query, _id, objects) = res {
            Ok(objects)
        } else {
            Err(Error::WrongResponse)
        }
    }
}
