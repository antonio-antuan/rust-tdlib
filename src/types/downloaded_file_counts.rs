use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains number of being downloaded and recently downloaded files found
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DownloadedFileCounts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Number of active file downloads found, including paused
    active_count: i32,
    /// Number of paused file downloads found
    paused_count: i32,
    /// Number of completed file downloads found
    completed_count: i32,
}

impl RObject for DownloadedFileCounts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl DownloadedFileCounts {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDownloadedFileCountsBuilder {
        let mut inner = DownloadedFileCounts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDDownloadedFileCountsBuilder { inner }
    }

    pub fn active_count(&self) -> i32 {
        self.active_count
    }

    pub fn paused_count(&self) -> i32 {
        self.paused_count
    }

    pub fn completed_count(&self) -> i32 {
        self.completed_count
    }
}

#[doc(hidden)]
pub struct RTDDownloadedFileCountsBuilder {
    inner: DownloadedFileCounts,
}

impl RTDDownloadedFileCountsBuilder {
    pub fn build(&self) -> DownloadedFileCounts {
        self.inner.clone()
    }

    pub fn active_count(&mut self, active_count: i32) -> &mut Self {
        self.inner.active_count = active_count;
        self
    }

    pub fn paused_count(&mut self, paused_count: i32) -> &mut Self {
        self.inner.paused_count = paused_count;
        self
    }

    pub fn completed_count(&mut self, completed_count: i32) -> &mut Self {
        self.inner.completed_count = completed_count;
        self
    }
}

impl AsRef<DownloadedFileCounts> for DownloadedFileCounts {
    fn as_ref(&self) -> &DownloadedFileCounts {
        self
    }
}

impl AsRef<DownloadedFileCounts> for RTDDownloadedFileCountsBuilder {
    fn as_ref(&self) -> &DownloadedFileCounts {
        &self.inner
    }
}
