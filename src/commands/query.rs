use super::StreamCommand;
use crate::protocol;
use crate::result::*;

#[derive(Debug, Default)]
pub struct QueryCommand<'a> {
    pub collection: &'a str,
    pub bucket: &'a str,
    pub terms: &'a str,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl StreamCommand for QueryCommand<'_> {
    type Response = Vec<String>;

    fn format(&self) -> String {
        let mut message = format!(
            r#"QUERY {} {} "{}""#,
            self.collection, self.bucket, self.terms
        );
        if let Some(limit) = self.limit.as_ref() {
            message.push_str(&format!(" LIMIT({})", limit));
        }
        if let Some(offset) = self.offset.as_ref() {
            message.push_str(&format!(" OFFSET({})", offset));
        }

        // use greyblake/whatlang-rs to autodect locale
        if let Some(info) = whatlang::detect(self.terms) {
            if info.confidence() == 1.0 {
                message.push_str(&format!(" LANG({})", info.lang().code()));
            }
        }

        message.push_str("\r\n");
        message
    }

    fn send(&self) -> protocol::Request {
        let lang = whatlang::detect(self.terms)
            .and_then(|i| (i.confidence() == 1.0).then(|| i.lang().code()));
        protocol::Request::Query {
            collection: self.collection.to_string(),
            bucket: self.bucket.to_string(),
            terms: self.terms.to_string(),
            offset: self.offset,
            limit: self.limit,
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
