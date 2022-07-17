#[derive(Debug)]
pub struct ObjDest(Dest, String);

impl ObjDest {
    pub fn new(cb: Dest, o: impl ToString) -> Self {
        Self(cb, o.to_string())
    }

    #[inline]
    pub fn collection(&self) -> &String {
        self.0.collection()
    }

    #[inline]
    pub fn bucket_opt(&self) -> Option<&String> {
        self.0.bucket_opt()
    }

    #[inline]
    pub fn object(&self) -> &String {
        &self.1
    }
}

#[derive(Debug, Clone)]
pub struct Dest {
    collection: String,
    bucket: Option<String>,
}

impl Dest {
    pub fn col_buc(c: impl ToString, b: impl ToString) -> Self {
        Self::col(c).buc(b)
    }

    pub fn col(c: impl ToString) -> Self {
        Self {
            collection: c.to_string(),
            bucket: None,
        }
    }

    pub fn buc(mut self, b: impl ToString) -> Self {
        self.bucket = Some(b.to_string());
        self
    }

    pub fn obj(self, o: impl ToString) -> ObjDest {
        ObjDest::new(self, o)
    }

    #[inline]
    pub fn collection(&self) -> &String {
        &self.collection
    }

    #[inline]
    pub fn bucket_opt(&self) -> Option<&String> {
        self.bucket.as_ref()
    }
}

#[derive(Debug)]
pub(crate) struct OptDest {
    pub(crate) collection: String,
    pub(crate) bucket: Option<String>,
    pub(crate) object: Option<String>,
}

impl OptDest {
    pub(crate) fn col(c: impl ToString) -> Self {
        Self {
            collection: c.to_string(),
            bucket: None,
            object: None,
        }
    }

    pub(crate) fn col_buc(c: impl ToString, b: impl ToString) -> Self {
        Self {
            collection: c.to_string(),
            bucket: Some(b.to_string()),
            object: None,
        }
    }

    pub(crate) fn col_buc_obj(c: impl ToString, b: impl ToString, o: impl ToString) -> Self {
        Self {
            collection: c.to_string(),
            bucket: Some(b.to_string()),
            object: Some(o.to_string()),
        }
    }
}
