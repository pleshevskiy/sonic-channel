use super::StreamCommand;
use crate::result::*;
use regex::Regex;

const RE_SUGGEST_RECEIVED_MESSAGE: &str = r"(?x)
    ^PENDING\s(?P<pending_suggest_id>\w+)\r\n
    EVENT\sSUGGEST\s(?P<event_suggest_id>\w+)\s(?P<words>.*?)\r\n$
";

#[derive(Debug, Default)]
pub struct SuggestCommand<'a> {
    pub collection: &'a str,
    pub bucket: &'a str,
    pub word: &'a str,
    pub limit: Option<usize>,
}

impl StreamCommand for SuggestCommand<'_> {
    type Response = Vec<String>;

    const READ_LINES_COUNT: usize = 2;

    fn message(&self) -> String {
        let mut message = format!(
            r#"SUGGEST {} {} "{}""#,
            self.collection, self.bucket, self.word
        );
        if let Some(limit) = self.limit.as_ref() {
            message.push_str(&format!(" LIMIT({})", limit));
        }
        message.push_str("\r\n");
        message
    }

    fn receive(&self, message: String) -> Result<Self::Response> {
        lazy_static! {
            static ref RE: Regex = Regex::new(RE_SUGGEST_RECEIVED_MESSAGE).unwrap();
        }

        dbg!(&message);

        match RE.captures(&message) {
            None => Err(Error::new(ErrorKind::WrongSonicResponse)),
            Some(caps) => {
                if caps["pending_suggest_id"] != caps["event_suggest_id"] {
                    Err(Error::new(ErrorKind::QueryResponseError(
                        "Pending id and event id don't match",
                    )))
                } else if caps["words"].is_empty() {
                    Ok(vec![])
                } else {
                    Ok(caps["words"].split_whitespace().map(str::to_owned).collect())
                }
            }
        }
    }
}
