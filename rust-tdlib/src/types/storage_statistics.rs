use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains the exact storage usage statistics split by chats and file type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorageStatistics {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Total size of files
    size: i64,
    /// Total number of files
    count: i32,
    /// Statistics split by chats
    by_chat: Vec<StorageStatisticsByChat>,
}

impl RObject for StorageStatistics {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl StorageStatistics {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStorageStatisticsBuilder {
        let mut inner = StorageStatistics::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDStorageStatisticsBuilder { inner }
    }

    pub fn size(&self) -> i64 {
        self.size
    }

    pub fn count(&self) -> i32 {
        self.count
    }

    pub fn by_chat(&self) -> &Vec<StorageStatisticsByChat> {
        &self.by_chat
    }
}

#[doc(hidden)]
pub struct RTDStorageStatisticsBuilder {
    inner: StorageStatistics,
}

impl RTDStorageStatisticsBuilder {
    pub fn build(&self) -> StorageStatistics {
        self.inner.clone()
    }

    pub fn size(&mut self, size: i64) -> &mut Self {
        self.inner.size = size;
        self
    }

    pub fn count(&mut self, count: i32) -> &mut Self {
        self.inner.count = count;
        self
    }

    pub fn by_chat(&mut self, by_chat: Vec<StorageStatisticsByChat>) -> &mut Self {
        self.inner.by_chat = by_chat;
        self
    }
}

impl AsRef<StorageStatistics> for StorageStatistics {
    fn as_ref(&self) -> &StorageStatistics {
        self
    }
}

impl AsRef<StorageStatistics> for RTDStorageStatisticsBuilder {
    fn as_ref(&self) -> &StorageStatistics {
        &self.inner
    }
}
