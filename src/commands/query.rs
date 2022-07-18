use super::StreamCommand;
use crate::misc::Dest;
use crate::protocol;
use crate::result::*;

/// Parameters for the `query` command
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// Collection and bucket where we should search for objects.
    pub dest: Dest,
    /// Searchable terms.
    pub terms: String,
    /// Language of the search data. If None, the client will try to determine based on the `terms`.
    pub lang: Option<whatlang::Lang>,
    /// Limit of result objects.
    pub limit: Option<usize>,
    /// The number of result objects we want to skip.
    pub offset: Option<usize>,
}

impl QueryRequest {
    /// Creates base query request.
    pub fn new(dest: Dest, terms: impl ToString) -> Self {
        Self {
            dest,
            terms: terms.to_string(),
            lang: None,
            limit: None,
            offset: None,
        }
    }

    /// Set a language for the request.
    pub fn lang(mut self, lang: whatlang::Lang) -> Self {
        self.lang = Some(lang);
        self
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

    /// Set the pagination for the request. Automatic offset calculation based on provided
    /// limit and page.
    ///
    /// Note: the first page is 0;
    pub fn pag(self, page: usize, limit: usize) -> Self {
        let offset = page * limit;
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
