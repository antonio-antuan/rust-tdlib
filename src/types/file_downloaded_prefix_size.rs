use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains size of downloaded prefix of a file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileDownloadedPrefixSize {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The prefix size, in bytes
    size: i64,
}

impl RObject for FileDownloadedPrefixSize {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl FileDownloadedPrefixSize {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileDownloadedPrefixSizeBuilder {
        let mut inner = FileDownloadedPrefixSize::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileDownloadedPrefixSizeBuilder { inner }
    }

    pub fn size(&self) -> i64 {
        self.size
    }
}

#[doc(hidden)]
pub struct RTDFileDownloadedPrefixSizeBuilder {
    inner: FileDownloadedPrefixSize,
}

impl RTDFileDownloadedPrefixSizeBuilder {
    pub fn build(&self) -> FileDownloadedPrefixSize {
        self.inner.clone()
    }

    pub fn size(&mut self, size: i64) -> &mut Self {
        self.inner.size = size;
        self
    }
}

impl AsRef<FileDownloadedPrefixSize> for FileDownloadedPrefixSize {
    fn as_ref(&self) -> &FileDownloadedPrefixSize {
        self
    }
}

impl AsRef<FileDownloadedPrefixSize> for RTDFileDownloadedPrefixSizeBuilder {
    fn as_ref(&self) -> &FileDownloadedPrefixSize {
        &self.inner
    }
}
