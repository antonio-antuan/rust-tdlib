use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains the storage usage statistics for a specific chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorageStatisticsByChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier; 0 if none

    #[serde(default)]
    chat_id: i64,
    /// Total size of the files in the chat, in bytes

    #[serde(default)]
    size: i64,
    /// Total number of files in the chat

    #[serde(default)]
    count: i32,
    /// Statistics split by file types

    #[serde(default)]
    by_file_type: Vec<StorageStatisticsByFileType>,
}

impl RObject for StorageStatisticsByChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl StorageStatisticsByChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StorageStatisticsByChatBuilder {
        let mut inner = StorageStatisticsByChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StorageStatisticsByChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn size(&self) -> i64 {
        self.size
    }

    pub fn count(&self) -> i32 {
        self.count
    }

    pub fn by_file_type(&self) -> &Vec<StorageStatisticsByFileType> {
        &self.by_file_type
    }
}

#[doc(hidden)]
pub struct StorageStatisticsByChatBuilder {
    inner: StorageStatisticsByChat,
}

#[deprecated]
pub type RTDStorageStatisticsByChatBuilder = StorageStatisticsByChatBuilder;

impl StorageStatisticsByChatBuilder {
    pub fn build(&self) -> StorageStatisticsByChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn size(&mut self, size: i64) -> &mut Self {
        self.inner.size = size;
        self
    }

    pub fn count(&mut self, count: i32) -> &mut Self {
        self.inner.count = count;
        self
    }

    pub fn by_file_type(&mut self, by_file_type: Vec<StorageStatisticsByFileType>) -> &mut Self {
        self.inner.by_file_type = by_file_type;
        self
    }
}

impl AsRef<StorageStatisticsByChat> for StorageStatisticsByChat {
    fn as_ref(&self) -> &StorageStatisticsByChat {
        self
    }
}

impl AsRef<StorageStatisticsByChat> for StorageStatisticsByChatBuilder {
    fn as_ref(&self) -> &StorageStatisticsByChat {
        &self.inner
    }
}
