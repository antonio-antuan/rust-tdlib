use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains the storage usage statistics for a specific file type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorageStatisticsByFileType {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// File type

    #[serde(skip_serializing_if = "FileType::_is_default")]
    file_type: FileType,
    /// Total size of the files
    size: i64,
    /// Total number of files
    count: i32,
}

impl RObject for StorageStatisticsByFileType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl StorageStatisticsByFileType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStorageStatisticsByFileTypeBuilder {
        let mut inner = StorageStatisticsByFileType::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDStorageStatisticsByFileTypeBuilder { inner }
    }

    pub fn file_type(&self) -> &FileType {
        &self.file_type
    }

    pub fn size(&self) -> i64 {
        self.size
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}

#[doc(hidden)]
pub struct RTDStorageStatisticsByFileTypeBuilder {
    inner: StorageStatisticsByFileType,
}

impl RTDStorageStatisticsByFileTypeBuilder {
    pub fn build(&self) -> StorageStatisticsByFileType {
        self.inner.clone()
    }

    pub fn file_type<T: AsRef<FileType>>(&mut self, file_type: T) -> &mut Self {
        self.inner.file_type = file_type.as_ref().clone();
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
}

impl AsRef<StorageStatisticsByFileType> for StorageStatisticsByFileType {
    fn as_ref(&self) -> &StorageStatisticsByFileType {
        self
    }
}

impl AsRef<StorageStatisticsByFileType> for RTDStorageStatisticsByFileTypeBuilder {
    fn as_ref(&self) -> &StorageStatisticsByFileType {
        &self.inner
    }
}
