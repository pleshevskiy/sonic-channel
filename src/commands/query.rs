use super::StreamCommand;
use crate::misc::Dest;
use crate::protocol;
use crate::result::*;

#[derive(Debug)]
pub struct QueryRequest<'a> {
    pub dest: Dest,
    pub terms: &'a str,
    pub lang: Option<whatlang::Lang>,
}

#[derive(Debug)]
pub struct PagQueryRequest<'a> {
    pub dest: Dest,
    pub terms: &'a str,
    pub lang: Option<whatlang::Lang>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl<'a> From<QueryRequest<'a>> for PagQueryRequest<'a> {
    fn from(req: QueryRequest<'a>) -> Self {
        Self {
            dest: req.dest,
            terms: req.terms,
            lang: req.lang,
            limit: None,
            offset: None,
        }
    }
}

#[derive(Debug)]
pub struct QueryCommand<'a> {
    pub(crate) req: PagQueryRequest<'a>,
}

impl StreamCommand for QueryCommand<'_> {
    type Response = Vec<String>;

    fn request(&self) -> protocol::Request {
        let dest = &self.req.dest;
        let lang = self
            .req
            .lang
            .or_else(|| {
                whatlang::detect(self.req.terms)
                    .and_then(|i| (i.confidence() == 1.0).then(|| i.lang()))
            })
            .map(|l| l.code());

        protocol::Request::Query {
            collection: dest.collection().clone(),
            bucket: dest
                .bucket_opt()
                .cloned()
                .unwrap_or_else(|| String::from("default")),
            terms: self.req.terms.to_string(),
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
