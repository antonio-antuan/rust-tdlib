use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Stops the preliminary uploading of a file. Supported only for files uploaded by using preliminaryUploadFile. For other files the behavior is undefined
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CancelPreliminaryUploadFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the file to stop uploading

    #[serde(default)]
    file_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CancelPreliminaryUploadFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CancelPreliminaryUploadFile {}

impl CancelPreliminaryUploadFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CancelPreliminaryUploadFileBuilder {
        let mut inner = CancelPreliminaryUploadFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "cancelPreliminaryUploadFile".to_string();

        CancelPreliminaryUploadFileBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }
}

#[doc(hidden)]
pub struct CancelPreliminaryUploadFileBuilder {
    inner: CancelPreliminaryUploadFile,
}

#[deprecated]
pub type RTDCancelPreliminaryUploadFileBuilder = CancelPreliminaryUploadFileBuilder;

impl CancelPreliminaryUploadFileBuilder {
    pub fn build(&self) -> CancelPreliminaryUploadFile {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }
}

impl AsRef<CancelPreliminaryUploadFile> for CancelPreliminaryUploadFile {
    fn as_ref(&self) -> &CancelPreliminaryUploadFile {
        self
    }
}

impl AsRef<CancelPreliminaryUploadFile> for CancelPreliminaryUploadFileBuilder {
    fn as_ref(&self) -> &CancelPreliminaryUploadFile {
        &self.inner
    }
}
