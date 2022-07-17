#[derive(Debug)]
pub struct ObjDest(Dest, String);

impl ObjDest {
    pub fn new(cb: Dest, o: impl ToString) -> Self {
        Self(cb, o.to_string())
    }

    pub fn collection(&self) -> &String {
        &self.0.collection()
    }

    pub fn bucket(&self) -> Option<&String> {
        self.0.bucket()
    }

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

    pub fn collection(&self) -> &String {
        &self.collection
    }

    pub fn bucket(&self) -> Option<&String> {
        self.bucket.as_ref()
    }
}
