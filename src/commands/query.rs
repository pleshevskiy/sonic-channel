use super::StreamCommand;
use crate::result::*;
use regex::Regex;

const RE_QUERY_RECEIVED_MESSAGE: &str = r"(?x)
    ^PENDING\s(?P<pending_query_id>\w+)\r\n
    EVENT\sQUERY\s(?P<event_query_id>\w+)\s(?P<objects>.*?)\r\n$
";

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

    const READ_LINES_COUNT: usize = 2;

    fn message(&self) -> String {
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
        message.push_str("\r\n");
        message
    }

    fn receive(&self, message: String) -> Result<Self::Response> {
        lazy_static! {
            static ref RE: Regex = Regex::new(RE_QUERY_RECEIVED_MESSAGE).unwrap();
        }

        dbg!(&message);

        match RE.captures(&message) {
            None => Err(Error::new(ErrorKind::QueryResponseError(
                "Sonic response are wrong. Please write issue to github.",
            ))),
            Some(caps) => {
                if &caps["pending_query_id"] != &caps["event_query_id"] {
                    Err(Error::new(ErrorKind::QueryResponseError(
                        "Pending id and event id don't match",
                    )))
                } else if caps["objects"].is_empty() {
                    Ok(vec![])
                } else {
                    let objects = caps["objects"]
                        .split(" ")
                        .map(String::from)
                        .collect::<Vec<String>>();

                    Ok(objects)
                }
            }
        }
    }
}
