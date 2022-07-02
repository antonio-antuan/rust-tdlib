use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// A full list of available network statistic entries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkStatistics {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Point in time (Unix timestamp) from which the statistics are collected

    #[serde(default)]
    since_date: i32,
    /// Network statistics entries

    #[serde(default)]
    entries: Vec<NetworkStatisticsEntry>,
}

impl RObject for NetworkStatistics {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl NetworkStatistics {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NetworkStatisticsBuilder {
        let mut inner = NetworkStatistics::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NetworkStatisticsBuilder { inner }
    }

    pub fn since_date(&self) -> i32 {
        self.since_date
    }

    pub fn entries(&self) -> &Vec<NetworkStatisticsEntry> {
        &self.entries
    }
}

#[doc(hidden)]
pub struct NetworkStatisticsBuilder {
    inner: NetworkStatistics,
}

#[deprecated]
pub type RTDNetworkStatisticsBuilder = NetworkStatisticsBuilder;

impl NetworkStatisticsBuilder {
    pub fn build(&self) -> NetworkStatistics {
        self.inner.clone()
    }

    pub fn since_date(&mut self, since_date: i32) -> &mut Self {
        self.inner.since_date = since_date;
        self
    }

    pub fn entries(&mut self, entries: Vec<NetworkStatisticsEntry>) -> &mut Self {
        self.inner.entries = entries;
        self
    }
}

impl AsRef<NetworkStatistics> for NetworkStatistics {
    fn as_ref(&self) -> &NetworkStatistics {
        self
    }
}

impl AsRef<NetworkStatistics> for NetworkStatisticsBuilder {
    fn as_ref(&self) -> &NetworkStatistics {
        &self.inner
    }
}
