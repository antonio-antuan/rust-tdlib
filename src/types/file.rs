use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct File {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique file identifier

    #[serde(default)]
    id: i32,
    /// File size, in bytes; 0 if unknown

    #[serde(default)]
    size: i64,
    /// Approximate file size in bytes in case the exact file size is unknown. Can be used to show download/upload progress

    #[serde(default)]
    expected_size: i64,
    /// Information about the local copy of the file
    local: LocalFile,
    /// Information about the remote copy of the file
    remote: RemoteFile,
}

impl RObject for File {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl File {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileBuilder {
        let mut inner = File::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn size(&self) -> i64 {
        self.size
    }

    pub fn expected_size(&self) -> i64 {
        self.expected_size
    }

    pub fn local(&self) -> &LocalFile {
        &self.local
    }

    pub fn remote(&self) -> &RemoteFile {
        &self.remote
    }
}

#[doc(hidden)]
pub struct FileBuilder {
    inner: File,
}

#[deprecated]
pub type RTDFileBuilder = FileBuilder;

impl FileBuilder {
    pub fn build(&self) -> File {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn size(&mut self, size: i64) -> &mut Self {
        self.inner.size = size;
        self
    }

    pub fn expected_size(&mut self, expected_size: i64) -> &mut Self {
        self.inner.expected_size = expected_size;
        self
    }

    pub fn local<T: AsRef<LocalFile>>(&mut self, local: T) -> &mut Self {
        self.inner.local = local.as_ref().clone();
        self
    }

    pub fn remote<T: AsRef<RemoteFile>>(&mut self, remote: T) -> &mut Self {
        self.inner.remote = remote.as_ref().clone();
        self
    }
}

impl AsRef<File> for File {
    fn as_ref(&self) -> &File {
        self
    }
}

impl AsRef<File> for FileBuilder {
    fn as_ref(&self) -> &File {
        &self.inner
    }
}
