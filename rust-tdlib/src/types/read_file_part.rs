use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Reads a part of a file from the TDLib file cache and returns read bytes. This method is intended to be used only if the application has no direct access to TDLib's file system, because it is usually slower than a direct read from the file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadFilePart {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the file. The file must be located in the TDLib file cache
    file_id: i32,
    /// The offset from which to read the file
    offset: i32,
    /// Number of bytes to read. An error will be returned if there are not enough bytes available in the file from the specified position. Pass 0 to read all available data from the specified position
    count: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReadFilePart {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReadFilePart {}

impl ReadFilePart {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDReadFilePartBuilder {
        let mut inner = ReadFilePart::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "readFilePart".to_string();

        RTDReadFilePartBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}

#[doc(hidden)]
pub struct RTDReadFilePartBuilder {
    inner: ReadFilePart,
}

impl RTDReadFilePartBuilder {
    pub fn build(&self) -> ReadFilePart {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }

    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.inner.offset = offset;
        self
    }

    pub fn count(&mut self, count: i32) -> &mut Self {
        self.inner.count = count;
        self
    }
}

impl AsRef<ReadFilePart> for ReadFilePart {
    fn as_ref(&self) -> &ReadFilePart {
        self
    }
}

impl AsRef<ReadFilePart> for RTDReadFilePartBuilder {
    fn as_ref(&self) -> &ReadFilePart {
        &self.inner
    }
}
