use super::StreamCommand;
use crate::result::{Error, ErrorKind, Result};
use regex::Regex;

const RE_QUERY_RECEIVED_MESSAGE: &str = r"^RESULT (?P<flush_count>\d+)\r\n$";
#[derive(Debug, Default)]
pub struct FlushCommand<'a> {
    pub collection: &'a str,
    pub bucket: Option<&'a str>,
    pub object: Option<&'a str>,
}

impl StreamCommand for FlushCommand<'_> {
    type Response = usize;

    fn message(&self) -> String {
        let mut message = match (self.bucket, self.object) {
            (Some(bucket), Some(object)) => {
                format!("FLUSHO {} {} {}", self.collection, bucket, object)
            }
            (Some(bucket), None) => format!("FLUSHB {} {}", self.collection, bucket),
            (None, None) => format!("FLUSHC {}", self.collection),
            _ => panic!("Invalid flush command"),
        };
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
            Some(caps) => caps["flush_count"].parse().map_err(|_| {
                Error::new(ErrorKind::QueryResponseError(
                    "Cannot parse sonic response to uint",
                ))
            }),
        }
    }
}
