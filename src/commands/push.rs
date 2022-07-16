use super::StreamCommand;
use crate::protocol;
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
    type Response = ();

    fn request(&self) -> protocol::Request {
        let lang = whatlang::detect(self.text)
            .and_then(|i| (i.confidence() == 1.0).then(|| i.lang().code()));
        protocol::Request::Push {
            collection: self.collection.to_string(),
            bucket: self.bucket.to_string(),
            object: self.object.to_string(),
            terms: self.text.to_string(),
            lang,
        }
    }

    fn receive(&self, res: protocol::Response) -> Result<Self::Response> {
        if matches!(res, protocol::Response::Ok) {
            Ok(())
        } else {
            Err(Error::WrongResponse)
        }
    }
}
