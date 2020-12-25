
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains the exact storage usage statistics split by chats and file type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorageStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total size of files
  size: i64,
  /// Total number of files
  count: i64,
  /// Statistics split by chats
  by_chat: Vec<StorageStatisticsByChat>,
  
}

impl RObject for StorageStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "storageStatistics" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl StorageStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStorageStatisticsBuilder {
    let mut inner = StorageStatistics::default();
    inner.td_name = "storageStatistics".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDStorageStatisticsBuilder { inner }
  }

  pub fn size(&self) -> i64 { self.size }

  pub fn count(&self) -> i64 { self.count }

  pub fn by_chat(&self) -> &Vec<StorageStatisticsByChat> { &self.by_chat }

}

#[doc(hidden)]
pub struct RTDStorageStatisticsBuilder {
  inner: StorageStatistics
}

impl RTDStorageStatisticsBuilder {
  pub fn build(&self) -> StorageStatistics { self.inner.clone() }

   
  pub fn size(&mut self, size: i64) -> &mut Self {
    self.inner.size = size;
    self
  }

   
  pub fn count(&mut self, count: i64) -> &mut Self {
    self.inner.count = count;
    self
  }

   
  pub fn by_chat(&mut self, by_chat: Vec<StorageStatisticsByChat>) -> &mut Self {
    self.inner.by_chat = by_chat;
    self
  }

}

impl AsRef<StorageStatistics> for StorageStatistics {
  fn as_ref(&self) -> &StorageStatistics { self }
}

impl AsRef<StorageStatistics> for RTDStorageStatisticsBuilder {
  fn as_ref(&self) -> &StorageStatistics { &self.inner }
}



