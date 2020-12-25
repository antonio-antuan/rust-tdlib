
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains database statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatabaseStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Database statistics in an unspecified human-readable format
  statistics: String,
  
}

impl RObject for DatabaseStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "databaseStatistics" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl DatabaseStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDatabaseStatisticsBuilder {
    let mut inner = DatabaseStatistics::default();
    inner.td_name = "databaseStatistics".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDatabaseStatisticsBuilder { inner }
  }

  pub fn statistics(&self) -> &String { &self.statistics }

}

#[doc(hidden)]
pub struct RTDDatabaseStatisticsBuilder {
  inner: DatabaseStatistics
}

impl RTDDatabaseStatisticsBuilder {
  pub fn build(&self) -> DatabaseStatistics { self.inner.clone() }

   
  pub fn statistics<T: AsRef<str>>(&mut self, statistics: T) -> &mut Self {
    self.inner.statistics = statistics.as_ref().to_string();
    self
  }

}

impl AsRef<DatabaseStatistics> for DatabaseStatistics {
  fn as_ref(&self) -> &DatabaseStatistics { self }
}

impl AsRef<DatabaseStatistics> for RTDDatabaseStatisticsBuilder {
  fn as_ref(&self) -> &DatabaseStatistics { &self.inner }
}



