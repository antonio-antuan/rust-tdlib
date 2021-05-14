use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a file by its remote ID; this is an offline request. Can be used to register a URL as a file for further uploading, or sending as a message. Even the request succeeds, the file can be used only if it is still accessible to the user. For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRemoteFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Remote identifier of the file to get
    remote_file_id: String,
    /// File type, if known

    #[serde(skip_serializing_if = "FileType::_is_default")]
    file_type: FileType,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetRemoteFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetRemoteFile {}

impl GetRemoteFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetRemoteFileBuilder {
        let mut inner = GetRemoteFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getRemoteFile".to_string();

        RTDGetRemoteFileBuilder { inner }
    }

    pub fn remote_file_id(&self) -> &String {
        &self.remote_file_id
    }

    pub fn file_type(&self) -> &FileType {
        &self.file_type
    }
}

#[doc(hidden)]
pub struct RTDGetRemoteFileBuilder {
    inner: GetRemoteFile,
}

impl RTDGetRemoteFileBuilder {
    pub fn build(&self) -> GetRemoteFile {
        self.inner.clone()
    }

    pub fn remote_file_id<T: AsRef<str>>(&mut self, remote_file_id: T) -> &mut Self {
        self.inner.remote_file_id = remote_file_id.as_ref().to_string();
        self
    }

    pub fn file_type<T: AsRef<FileType>>(&mut self, file_type: T) -> &mut Self {
        self.inner.file_type = file_type.as_ref().clone();
        self
    }
}

impl AsRef<GetRemoteFile> for GetRemoteFile {
    fn as_ref(&self) -> &GetRemoteFile {
        self
    }
}

impl AsRef<GetRemoteFile> for RTDGetRemoteFileBuilder {
    fn as_ref(&self) -> &GetRemoteFile {
        &self.inner
    }
}
