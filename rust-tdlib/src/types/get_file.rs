use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a file; this is an offline request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the file to get
    file_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetFile {}

impl GetFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetFileBuilder {
        let mut inner = GetFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getFile".to_string();

        RTDGetFileBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }
}

#[doc(hidden)]
pub struct RTDGetFileBuilder {
    inner: GetFile,
}

impl RTDGetFileBuilder {
    pub fn build(&self) -> GetFile {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }
}

impl AsRef<GetFile> for GetFile {
    fn as_ref(&self) -> &GetFile {
        self
    }
}

impl AsRef<GetFile> for RTDGetFileBuilder {
    fn as_ref(&self) -> &GetFile {
        &self.inner
    }
}
