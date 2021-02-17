use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Stops the downloading of a file. If a file has already been downloaded, does nothing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CancelDownloadFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of a file to stop downloading
    file_id: i32,
    /// Pass true to stop downloading only if it hasn't been started, i.e. request hasn't been sent to server
    only_if_pending: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CancelDownloadFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CancelDownloadFile {}

impl CancelDownloadFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCancelDownloadFileBuilder {
        let mut inner = CancelDownloadFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "cancelDownloadFile".to_string();

        RTDCancelDownloadFileBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }

    pub fn only_if_pending(&self) -> bool {
        self.only_if_pending
    }
}

#[doc(hidden)]
pub struct RTDCancelDownloadFileBuilder {
    inner: CancelDownloadFile,
}

impl RTDCancelDownloadFileBuilder {
    pub fn build(&self) -> CancelDownloadFile {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }

    pub fn only_if_pending(&mut self, only_if_pending: bool) -> &mut Self {
        self.inner.only_if_pending = only_if_pending;
        self
    }
}

impl AsRef<CancelDownloadFile> for CancelDownloadFile {
    fn as_ref(&self) -> &CancelDownloadFile {
        self
    }
}

impl AsRef<CancelDownloadFile> for RTDCancelDownloadFileBuilder {
    fn as_ref(&self) -> &CancelDownloadFile {
        &self.inner
    }
}
