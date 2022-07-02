use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Optimizes storage usage, i.e. deletes some files and returns new storage usage statistics. Secret thumbnails can't be deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptimizeStorage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Limit on the total size of files after deletion, in bytes. Pass 1 to use the default limit

    #[serde(default)]
    size: i64,
    /// Limit on the time that has passed since the last time a file was accessed (or creation time for some filesystems). Pass 1 to use the default limit

    #[serde(default)]
    ttl: i32,
    /// Limit on the total count of files after deletion. Pass 1 to use the default limit

    #[serde(default)]
    count: i32,
    /// The amount of time after the creation of a file during which it can't be deleted, in seconds. Pass 1 to use the default value

    #[serde(default)]
    immunity_delay: i32,
    /// If non-empty, only files with the given types are considered. By default, all types except thumbnails, profile photos, stickers and wallpapers are deleted

    #[serde(default)]
    file_types: Vec<FileType>,
    /// If non-empty, only files from the given chats are considered. Use 0 as chat identifier to delete files not belonging to any chat (e.g., profile photos)

    #[serde(default)]
    chat_ids: Vec<i64>,
    /// If non-empty, files from the given chats are excluded. Use 0 as chat identifier to exclude all files not belonging to any chat (e.g., profile photos)

    #[serde(default)]
    exclude_chat_ids: Vec<i64>,
    /// Pass true if statistics about the files that were deleted must be returned instead of the whole storage usage statistics. Affects only returned statistics

    #[serde(default)]
    return_deleted_file_statistics: bool,
    /// Same as in getStorageStatistics. Affects only returned statistics

    #[serde(default)]
    chat_limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for OptimizeStorage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for OptimizeStorage {}

impl OptimizeStorage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> OptimizeStorageBuilder {
        let mut inner = OptimizeStorage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "optimizeStorage".to_string();

        OptimizeStorageBuilder { inner }
    }

    pub fn size(&self) -> i64 {
        self.size
    }

    pub fn ttl(&self) -> i32 {
        self.ttl
    }

    pub fn count(&self) -> i32 {
        self.count
    }

    pub fn immunity_delay(&self) -> i32 {
        self.immunity_delay
    }

    pub fn file_types(&self) -> &Vec<FileType> {
        &self.file_types
    }

    pub fn chat_ids(&self) -> &Vec<i64> {
        &self.chat_ids
    }

    pub fn exclude_chat_ids(&self) -> &Vec<i64> {
        &self.exclude_chat_ids
    }

    pub fn return_deleted_file_statistics(&self) -> bool {
        self.return_deleted_file_statistics
    }

    pub fn chat_limit(&self) -> i32 {
        self.chat_limit
    }
}

#[doc(hidden)]
pub struct OptimizeStorageBuilder {
    inner: OptimizeStorage,
}

#[deprecated]
pub type RTDOptimizeStorageBuilder = OptimizeStorageBuilder;

impl OptimizeStorageBuilder {
    pub fn build(&self) -> OptimizeStorage {
        self.inner.clone()
    }

    pub fn size(&mut self, size: i64) -> &mut Self {
        self.inner.size = size;
        self
    }

    pub fn ttl(&mut self, ttl: i32) -> &mut Self {
        self.inner.ttl = ttl;
        self
    }

    pub fn count(&mut self, count: i32) -> &mut Self {
        self.inner.count = count;
        self
    }

    pub fn immunity_delay(&mut self, immunity_delay: i32) -> &mut Self {
        self.inner.immunity_delay = immunity_delay;
        self
    }

    pub fn file_types(&mut self, file_types: Vec<FileType>) -> &mut Self {
        self.inner.file_types = file_types;
        self
    }

    pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
        self.inner.chat_ids = chat_ids;
        self
    }

    pub fn exclude_chat_ids(&mut self, exclude_chat_ids: Vec<i64>) -> &mut Self {
        self.inner.exclude_chat_ids = exclude_chat_ids;
        self
    }

    pub fn return_deleted_file_statistics(
        &mut self,
        return_deleted_file_statistics: bool,
    ) -> &mut Self {
        self.inner.return_deleted_file_statistics = return_deleted_file_statistics;
        self
    }

    pub fn chat_limit(&mut self, chat_limit: i32) -> &mut Self {
        self.inner.chat_limit = chat_limit;
        self
    }
}

impl AsRef<OptimizeStorage> for OptimizeStorage {
    fn as_ref(&self) -> &OptimizeStorage {
        self
    }
}

impl AsRef<OptimizeStorage> for OptimizeStorageBuilder {
    fn as_ref(&self) -> &OptimizeStorage {
        &self.inner
    }
}
