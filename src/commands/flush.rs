use super::StreamCommand;
use crate::result::{Error, ErrorKind, Result};

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
        if message.starts_with("RESULT ") {
            let count = message.split_whitespace().last().unwrap_or_default();
            count.parse().map_err(|_| {
                Error::new(ErrorKind::QueryResponseError(
                    "Cannot parse count of flush method response to usize",
                ))
            })
        } else {
            Err(Error::new(ErrorKind::QueryResponseError(
                "Sonic response are wrong. Please write issue to github.",
            )))
        }
    }
}
