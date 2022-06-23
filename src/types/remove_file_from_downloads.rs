use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Removes a file from the file download list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveFileFromDownloads {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the downloaded file
    file_id: i32,
    /// Pass true to delete the file from the TDLib file cache
    delete_from_cache: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveFileFromDownloads {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveFileFromDownloads {}

impl RemoveFileFromDownloads {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRemoveFileFromDownloadsBuilder {
        let mut inner = RemoveFileFromDownloads::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeFileFromDownloads".to_string();

        RTDRemoveFileFromDownloadsBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }

    pub fn delete_from_cache(&self) -> bool {
        self.delete_from_cache
    }
}

#[doc(hidden)]
pub struct RTDRemoveFileFromDownloadsBuilder {
    inner: RemoveFileFromDownloads,
}

impl RTDRemoveFileFromDownloadsBuilder {
    pub fn build(&self) -> RemoveFileFromDownloads {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }

    pub fn delete_from_cache(&mut self, delete_from_cache: bool) -> &mut Self {
        self.inner.delete_from_cache = delete_from_cache;
        self
    }
}

impl AsRef<RemoveFileFromDownloads> for RemoveFileFromDownloads {
    fn as_ref(&self) -> &RemoveFileFromDownloads {
        self
    }
}

impl AsRef<RemoveFileFromDownloads> for RTDRemoveFileFromDownloadsBuilder {
    fn as_ref(&self) -> &RemoveFileFromDownloads {
        &self.inner
    }
}
