use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Removes all files from the file download list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveAllFilesFromDownloads {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pass true to remove only active downloads, including paused
    only_active: bool,
    /// Pass true to remove only completed downloads
    only_completed: bool,
    /// Pass true to delete the file from the TDLib file cache
    delete_from_cache: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveAllFilesFromDownloads {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveAllFilesFromDownloads {}

impl RemoveAllFilesFromDownloads {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRemoveAllFilesFromDownloadsBuilder {
        let mut inner = RemoveAllFilesFromDownloads::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeAllFilesFromDownloads".to_string();

        RTDRemoveAllFilesFromDownloadsBuilder { inner }
    }

    pub fn only_active(&self) -> bool {
        self.only_active
    }

    pub fn only_completed(&self) -> bool {
        self.only_completed
    }

    pub fn delete_from_cache(&self) -> bool {
        self.delete_from_cache
    }
}

#[doc(hidden)]
pub struct RTDRemoveAllFilesFromDownloadsBuilder {
    inner: RemoveAllFilesFromDownloads,
}

impl RTDRemoveAllFilesFromDownloadsBuilder {
    pub fn build(&self) -> RemoveAllFilesFromDownloads {
        self.inner.clone()
    }

    pub fn only_active(&mut self, only_active: bool) -> &mut Self {
        self.inner.only_active = only_active;
        self
    }

    pub fn only_completed(&mut self, only_completed: bool) -> &mut Self {
        self.inner.only_completed = only_completed;
        self
    }

    pub fn delete_from_cache(&mut self, delete_from_cache: bool) -> &mut Self {
        self.inner.delete_from_cache = delete_from_cache;
        self
    }
}

impl AsRef<RemoveAllFilesFromDownloads> for RemoveAllFilesFromDownloads {
    fn as_ref(&self) -> &RemoveAllFilesFromDownloads {
        self
    }
}

impl AsRef<RemoveAllFilesFromDownloads> for RTDRemoveAllFilesFromDownloadsBuilder {
    fn as_ref(&self) -> &RemoveAllFilesFromDownloads {
        &self.inner
    }
}
