use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains database statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatabaseStatistics {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Database statistics in an unspecified human-readable format

    #[serde(default)]
    statistics: String,
}

impl RObject for DatabaseStatistics {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl DatabaseStatistics {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DatabaseStatisticsBuilder {
        let mut inner = DatabaseStatistics::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DatabaseStatisticsBuilder { inner }
    }

    pub fn statistics(&self) -> &String {
        &self.statistics
    }
}

#[doc(hidden)]
pub struct DatabaseStatisticsBuilder {
    inner: DatabaseStatistics,
}

#[deprecated]
pub type RTDDatabaseStatisticsBuilder = DatabaseStatisticsBuilder;

impl DatabaseStatisticsBuilder {
    pub fn build(&self) -> DatabaseStatistics {
        self.inner.clone()
    }

    pub fn statistics<T: AsRef<str>>(&mut self, statistics: T) -> &mut Self {
        self.inner.statistics = statistics.as_ref().to_string();
        self
    }
}

impl AsRef<DatabaseStatistics> for DatabaseStatistics {
    fn as_ref(&self) -> &DatabaseStatistics {
        self
    }
}

impl AsRef<DatabaseStatistics> for DatabaseStatisticsBuilder {
    fn as_ref(&self) -> &DatabaseStatistics {
        &self.inner
    }
}
