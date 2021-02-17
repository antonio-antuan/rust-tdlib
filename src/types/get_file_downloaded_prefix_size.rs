use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns file downloaded prefix size from a given offset
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetFileDownloadedPrefixSize {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the file
    file_id: i32,
    /// Offset from which downloaded prefix size should be calculated
    offset: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetFileDownloadedPrefixSize {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetFileDownloadedPrefixSize {}

impl GetFileDownloadedPrefixSize {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetFileDownloadedPrefixSizeBuilder {
        let mut inner = GetFileDownloadedPrefixSize::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getFileDownloadedPrefixSize".to_string();

        RTDGetFileDownloadedPrefixSizeBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }
}

#[doc(hidden)]
pub struct RTDGetFileDownloadedPrefixSizeBuilder {
    inner: GetFileDownloadedPrefixSize,
}

impl RTDGetFileDownloadedPrefixSizeBuilder {
    pub fn build(&self) -> GetFileDownloadedPrefixSize {
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
}

impl AsRef<GetFileDownloadedPrefixSize> for GetFileDownloadedPrefixSize {
    fn as_ref(&self) -> &GetFileDownloadedPrefixSize {
        self
    }
}

impl AsRef<GetFileDownloadedPrefixSize> for RTDGetFileDownloadedPrefixSizeBuilder {
    fn as_ref(&self) -> &GetFileDownloadedPrefixSize {
        &self.inner
    }
}
