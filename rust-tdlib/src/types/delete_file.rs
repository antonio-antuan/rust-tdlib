use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Deletes a file from the TDLib file cache
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the file to delete
    file_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteFile {}

impl DeleteFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeleteFileBuilder {
        let mut inner = DeleteFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteFile".to_string();

        RTDDeleteFileBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }
}

#[doc(hidden)]
pub struct RTDDeleteFileBuilder {
    inner: DeleteFile,
}

impl RTDDeleteFileBuilder {
    pub fn build(&self) -> DeleteFile {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }
}

impl AsRef<DeleteFile> for DeleteFile {
    fn as_ref(&self) -> &DeleteFile {
        self
    }
}

impl AsRef<DeleteFile> for RTDDeleteFileBuilder {
    fn as_ref(&self) -> &DeleteFile {
        &self.inner
    }
}
