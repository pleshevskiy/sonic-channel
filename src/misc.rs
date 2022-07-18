/// Search data destination. Contains collection, bucket and object.
#[derive(Debug, PartialEq, Eq)]
pub struct ObjDest(Dest, String);

impl ObjDest {
    /// Creates a new object destination from base destination (`Dest`) and object id.
    ///
    /// ```rust
    /// # use sonic_channel::{Dest, ObjDest};
    /// let base_dest = Dest::col_buc("wiki", "user:1");
    /// let dest = ObjDest::new(base_dest, "article:1");
    /// assert_eq!(dest.collection(), "wiki");
    /// assert_eq!(dest.bucket_opt(), Some(&String::from("user:1")));
    /// assert_eq!(dest.object(), "article:1");
    /// ```
    pub fn new(cb: Dest, o: impl ToString) -> Self {
        Self(cb, o.to_string())
    }

    /// Returns the collection.
    #[inline]
    pub fn collection(&self) -> &String {
        self.0.collection()
    }

    /// Returns the optional bucket.
    #[inline]
    pub fn bucket_opt(&self) -> Option<&String> {
        self.0.bucket_opt()
    }

    /// Returns the object id.
    #[inline]
    pub fn object(&self) -> &String {
        &self.1
    }
}

/// Search objects destination. Contains collection and bucket.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dest {
    collection: String,
    bucket: Option<String>,
}

impl Dest {
    /// Creates a new destination with collection and bucket.
    ///
    /// ```rust
    /// # use sonic_channel::Dest;
    /// let dest = Dest::col_buc("wiki", "user:1");
    /// assert_eq!(dest.collection(), "wiki");
    /// assert_eq!(dest.bucket_opt(), Some(&String::from("user:1")));
    /// ```
    pub fn col_buc(c: impl ToString, b: impl ToString) -> Self {
        Self::col(c).buc(b)
    }

    /// Creates a new destination with collection.
    ///
    /// ```rust
    /// # use sonic_channel::Dest;
    /// let dest = Dest::col("wiki");
    /// assert_eq!(dest.collection(), "wiki");
    /// ```
    pub fn col(c: impl ToString) -> Self {
        Self {
            collection: c.to_string(),
            bucket: None,
        }
    }

    /// Set bucket for the destination.
    ///
    /// ```rust
    /// # use sonic_channel::Dest;
    /// let dest = Dest::col("wiki").buc("user:1");
    /// assert_eq!(dest.collection(), "wiki");
    /// assert_eq!(dest.bucket_opt(), Some(&String::from("user:1")));
    /// ```
    pub fn buc(mut self, b: impl ToString) -> Self {
        self.bucket = Some(b.to_string());
        self
    }

    /// Set object id to the destination and transform to object destination (`ObjDest`).
    ///
    /// Short for `ObjDest::new(dest, object_id)`
    ///
    /// ```rust
    /// # use sonic_channel::Dest;
    /// let dest = Dest::col_buc("wiki", "user:1").obj("article:1");
    /// assert_eq!(dest.collection(), "wiki");
    /// assert_eq!(dest.bucket_opt(), Some(&String::from("user:1")));
    /// assert_eq!(dest.object(), "article:1");
    /// ```
    pub fn obj(self, o: impl ToString) -> ObjDest {
        ObjDest::new(self, o)
    }

    /// Returns the collection.
    #[inline]
    pub fn collection(&self) -> &String {
        &self.collection
    }

    /// Returns the optional bucket.
    #[inline]
    pub fn bucket_opt(&self) -> Option<&String> {
        self.bucket.as_ref()
    }
}

#[cfg(feature = "ingest")]
#[derive(Debug)]
pub(crate) struct OptDest {
    pub(crate) collection: String,
    pub(crate) bucket: Option<String>,
    pub(crate) object: Option<String>,
}

#[cfg(feature = "ingest")]
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

#[cfg(feature = "ingest")]
impl From<Dest> for OptDest {
    fn from(d: Dest) -> Self {
        Self {
            collection: d.collection,
            bucket: d.bucket,
            object: None,
        }
    }
}

#[cfg(feature = "ingest")]
impl From<ObjDest> for OptDest {
    fn from(ObjDest(dest, obj): ObjDest) -> Self {
        Self {
            collection: dest.collection,
            bucket: dest.bucket,
            object: Some(obj),
        }
    }
}
