
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains the storage usage statistics for a specific file type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorageStatisticsByFileType {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// File type
  file_type: FileType,
  /// Total size of the files
  size: i64,
  /// Total number of files
  count: i64,
  
}

impl RObject for StorageStatisticsByFileType {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "storageStatisticsByFileType" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl StorageStatisticsByFileType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStorageStatisticsByFileTypeBuilder {
    let mut inner = StorageStatisticsByFileType::default();
    inner.td_name = "storageStatisticsByFileType".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDStorageStatisticsByFileTypeBuilder { inner }
  }

  pub fn file_type(&self) -> &FileType { &self.file_type }

  pub fn size(&self) -> i64 { self.size }

  pub fn count(&self) -> i64 { self.count }

}

#[doc(hidden)]
pub struct RTDStorageStatisticsByFileTypeBuilder {
  inner: StorageStatisticsByFileType
}

impl RTDStorageStatisticsByFileTypeBuilder {
  pub fn build(&self) -> StorageStatisticsByFileType { self.inner.clone() }

   
  pub fn file_type<T: AsRef<FileType>>(&mut self, file_type: T) -> &mut Self {
    self.inner.file_type = file_type.as_ref().clone();
    self
  }

   
  pub fn size(&mut self, size: i64) -> &mut Self {
    self.inner.size = size;
    self
  }

   
  pub fn count(&mut self, count: i64) -> &mut Self {
    self.inner.count = count;
    self
  }

}

impl AsRef<StorageStatisticsByFileType> for StorageStatisticsByFileType {
  fn as_ref(&self) -> &StorageStatisticsByFileType { self }
}

impl AsRef<StorageStatisticsByFileType> for RTDStorageStatisticsByFileTypeBuilder {
  fn as_ref(&self) -> &StorageStatisticsByFileType { &self.inner }
}



