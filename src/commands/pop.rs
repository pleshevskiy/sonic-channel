use super::StreamCommand;
use crate::result::{Error, ErrorKind, Result};

#[doc(hidden)]
#[derive(Debug, Default)]
pub struct PopCommand<'a> {
    pub collection: &'a str,
    pub bucket: &'a str,
    pub object: &'a str,
    pub text: &'a str,
}

impl StreamCommand for PopCommand<'_> {
    type Response = u32;

    fn message(&self) -> String {
        let mut message = format!(
            r#"POP {} {} {} "{}""#,
            self.collection, self.bucket, self.object, self.text
        );
        message.push_str("\r\n");
        message
    }

    fn receive(&self, message: String) -> Result<Self::Response> {
        if message.starts_with("RESULT") {
            let count = message.split_whitespace().last().unwrap_or_default();
            count
                .parse()
                .map_err(|_| Error::new(ErrorKind::QueryResponseError("Cannot parse pop count")))

        } else {
            Err(Error::new(ErrorKind::QueryResponseError("Cannot parse result")))
        }
    }
}
