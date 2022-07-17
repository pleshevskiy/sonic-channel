use super::StreamCommand;
use crate::misc::ObjDest;
use crate::protocol;
use crate::result::*;

#[derive(Debug)]
pub struct PushRequest<'a> {
    pub dest: ObjDest,
    pub text: &'a str,
    pub lang: Option<whatlang::Lang>,
}

#[derive(Debug)]
pub struct PushCommand<'a> {
    pub(crate) req: PushRequest<'a>,
}

impl StreamCommand for PushCommand<'_> {
    type Response = ();

    fn request(&self) -> protocol::Request {
        let req = &self.req;

        let lang = req
            .lang
            .or_else(|| {
                whatlang::detect(req.text).and_then(|i| (i.confidence() == 1.0).then(|| i.lang()))
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
