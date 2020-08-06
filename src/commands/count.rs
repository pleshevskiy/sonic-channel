use super::StreamCommand;
use crate::result::*;

#[derive(Debug, Default)]
pub struct CountCommand<'a> {
    pub collection: &'a str,
    pub bucket: Option<&'a str>,
    pub object: Option<&'a str>,
}

impl StreamCommand for CountCommand<'_> {
    type Response = usize;

    fn message(&self) -> String {
        let mut message = format!("COUNT {}", self.collection);
        if let Some(bucket) = self.bucket {
            message.push_str(&format!(" {}", bucket));

            if let Some(object) = self.object {
                message.push_str(&format!(" {}", object));
            }
        }
        message.push_str("\r\n");
        message
    }

    fn receive(&self, message: String) -> Result<Self::Response> {
        if message.starts_with("RESULT ") {
            let count = message.split_whitespace().last().unwrap_or_default();
            count.parse().map_err(|_| {
                Error::new(ErrorKind::QueryResponseError(
                    "Cannot parse count of count method response to usize",
                ))
            })
        } else {
            Err(Error::new(ErrorKind::WrongSonicResponse))
        }
    }
}
