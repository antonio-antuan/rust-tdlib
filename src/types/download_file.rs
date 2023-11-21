use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Downloads a file from the cloud. Download progress and completion of the download will be notified through updateFile updates
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DownloadFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the file to download

    #[serde(default)]
    file_id: i32,
    /// Priority of the download (1-32). The higher the priority, the earlier the file will be downloaded. If the priorities of two files are equal, then the last one for which downloadFile/addFileToDownloads was called will be downloaded first

    #[serde(default)]
    priority: i32,
    /// The starting position from which the file needs to be downloaded

    #[serde(default)]
    offset: i64,
    /// Number of bytes which need to be downloaded starting from the "offset" position before the download will automatically be canceled; use 0 to download without a limit

    #[serde(default)]
    limit: i64,
    /// Pass true to return response only after the file download has succeeded, has failed, has been canceled, or a new downloadFile request with different offset/limit parameters was sent; pass false to return file state immediately, just after the download has been started

    #[serde(default)]
    synchronous: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DownloadFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DownloadFile {}

impl DownloadFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DownloadFileBuilder {
        let mut inner = DownloadFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "downloadFile".to_string();

        DownloadFileBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }

    pub fn priority(&self) -> i32 {
        self.priority
    }

    pub fn offset(&self) -> i64 {
        self.offset
    }

    pub fn limit(&self) -> i64 {
        self.limit
    }

    pub fn synchronous(&self) -> bool {
        self.synchronous
    }
}

#[doc(hidden)]
pub struct DownloadFileBuilder {
    inner: DownloadFile,
}

#[deprecated]
pub type RTDDownloadFileBuilder = DownloadFileBuilder;

impl DownloadFileBuilder {
    pub fn build(&self) -> DownloadFile {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }

    pub fn priority(&mut self, priority: i32) -> &mut Self {
        self.inner.priority = priority;
        self
    }

    pub fn offset(&mut self, offset: i64) -> &mut Self {
        self.inner.offset = offset;
        self
    }

    pub fn limit(&mut self, limit: i64) -> &mut Self {
        self.inner.limit = limit;
        self
    }

    pub fn synchronous(&mut self, synchronous: bool) -> &mut Self {
        self.inner.synchronous = synchronous;
        self
    }
}

impl AsRef<DownloadFile> for DownloadFile {
    fn as_ref(&self) -> &DownloadFile {
        self
    }
}

impl AsRef<DownloadFile> for DownloadFileBuilder {
    fn as_ref(&self) -> &DownloadFile {
        &self.inner
    }
}
