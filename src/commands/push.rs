use super::StreamCommand;
use crate::misc::ObjDest;
use crate::protocol;
use crate::result::*;

/// Parameters for the `push` command.
#[derive(Debug)]
pub struct PushRequest {
    /// Collection, bucket and object where we should push search data in the index.
    pub dest: ObjDest,
    /// Search data to be added
    pub text: String,
    /// Language of the search data. If None, the client will try to determine based on the `text`.
    pub lang: Option<whatlang::Lang>,
}

impl PushRequest {
    /// Creates a base push request
    pub fn new(dest: ObjDest, text: impl ToString) -> Self {
        Self {
            dest,
            text: text.to_string(),
            lang: None,
        }
    }

    /// Set a language for the request.
    pub fn lang(mut self, lang: whatlang::Lang) -> Self {
        self.lang = Some(lang);
        self
    }
}

#[derive(Debug)]
pub struct PushCommand {
    pub(crate) req: PushRequest,
}

impl StreamCommand for PushCommand {
    type Response = ();

    fn request(&self) -> protocol::Request {
        let req = &self.req;

        let lang = req
            .lang
            .or_else(|| {
                whatlang::detect(&req.text).and_then(|i| (i.confidence() == 1.0).then(|| i.lang()))
            })
            .map(|l| l.code());

        protocol::Request::Push {
            collection: req.dest.collection().clone(),
            bucket: req
                .dest
                .bucket_opt()
                .cloned()
                // TODO: use a global context for default bucket value
                .unwrap_or_else(|| String::from("default")),
            object: req.dest.object().clone(),
            terms: req.text.to_string(),
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
