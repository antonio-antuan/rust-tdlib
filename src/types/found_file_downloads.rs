use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of downloaded files, found by a search
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundFileDownloads {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Total number of suitable files, ignoring offset
    total_counts: DownloadedFileCounts,
    /// The list of files

    #[serde(default)]
    files: Vec<FileDownload>,
    /// The offset for the next request. If empty, there are no more results

    #[serde(default)]
    next_offset: String,
}

impl RObject for FoundFileDownloads {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl FoundFileDownloads {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FoundFileDownloadsBuilder {
        let mut inner = FoundFileDownloads::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FoundFileDownloadsBuilder { inner }
    }

    pub fn total_counts(&self) -> &DownloadedFileCounts {
        &self.total_counts
    }

    pub fn files(&self) -> &Vec<FileDownload> {
        &self.files
    }

    pub fn next_offset(&self) -> &String {
        &self.next_offset
    }
}

#[doc(hidden)]
pub struct FoundFileDownloadsBuilder {
    inner: FoundFileDownloads,
}

#[deprecated]
pub type RTDFoundFileDownloadsBuilder = FoundFileDownloadsBuilder;

impl FoundFileDownloadsBuilder {
    pub fn build(&self) -> FoundFileDownloads {
        self.inner.clone()
    }

    pub fn total_counts<T: AsRef<DownloadedFileCounts>>(&mut self, total_counts: T) -> &mut Self {
        self.inner.total_counts = total_counts.as_ref().clone();
        self
    }

    pub fn files(&mut self, files: Vec<FileDownload>) -> &mut Self {
        self.inner.files = files;
        self
    }

    pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
        self.inner.next_offset = next_offset.as_ref().to_string();
        self
    }
}

impl AsRef<FoundFileDownloads> for FoundFileDownloads {
    fn as_ref(&self) -> &FoundFileDownloads {
        self
    }
}

impl AsRef<FoundFileDownloads> for FoundFileDownloadsBuilder {
    fn as_ref(&self) -> &FoundFileDownloads {
        &self.inner
    }
}
