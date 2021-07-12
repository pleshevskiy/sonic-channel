use super::StreamCommand;
use crate::result::*;

#[derive(Debug, Default)]
pub struct PushCommand<'a> {
    pub collection: &'a str,
    pub bucket: &'a str,
    pub object: &'a str,
    pub text: &'a str,
    pub locale: Option<&'a str>,
}

impl StreamCommand for PushCommand<'_> {
    type Response = bool;

    fn message(&self) -> String {
        let mut message = format!(
            r#"PUSH {} {} {} "{}""#,
            self.collection, self.bucket, self.object, self.text
        );
        if let Some(locale) = self.locale.as_ref() {
            message.push_str(&format!(" LANG({})", locale));
        }
        message.push_str("\r\n");
        message
    }

    fn receive(&self, message: String) -> Result<Self::Response> {
        if message == "OK\r\n" {
            Ok(true)
        } else {
            Err(Error::new(ErrorKind::WrongSonicResponse))
        }
    }
}
