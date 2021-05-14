use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Stops the uploading of a file. Supported only for files uploaded by using uploadFile. For other files the behavior is undefined
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CancelUploadFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the file to stop uploading
    file_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CancelUploadFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CancelUploadFile {}

impl CancelUploadFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCancelUploadFileBuilder {
        let mut inner = CancelUploadFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "cancelUploadFile".to_string();

        RTDCancelUploadFileBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }
}

#[doc(hidden)]
pub struct RTDCancelUploadFileBuilder {
    inner: CancelUploadFile,
}

impl RTDCancelUploadFileBuilder {
    pub fn build(&self) -> CancelUploadFile {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }
}

impl AsRef<CancelUploadFile> for CancelUploadFile {
    fn as_ref(&self) -> &CancelUploadFile {
        self
    }
}

impl AsRef<CancelUploadFile> for RTDCancelUploadFileBuilder {
    fn as_ref(&self) -> &CancelUploadFile {
        &self.inner
    }
}
