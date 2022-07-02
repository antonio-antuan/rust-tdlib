use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Adds the specified data to data usage statistics. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddNetworkStatistics {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The network statistics entry with the data to be added to statistics

    #[serde(skip_serializing_if = "NetworkStatisticsEntry::_is_default")]
    entry: NetworkStatisticsEntry,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddNetworkStatistics {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddNetworkStatistics {}

impl AddNetworkStatistics {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddNetworkStatisticsBuilder {
        let mut inner = AddNetworkStatistics::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addNetworkStatistics".to_string();

        AddNetworkStatisticsBuilder { inner }
    }

    pub fn entry(&self) -> &NetworkStatisticsEntry {
        &self.entry
    }
}

#[doc(hidden)]
pub struct AddNetworkStatisticsBuilder {
    inner: AddNetworkStatistics,
}

#[deprecated]
pub type RTDAddNetworkStatisticsBuilder = AddNetworkStatisticsBuilder;

impl AddNetworkStatisticsBuilder {
    pub fn build(&self) -> AddNetworkStatistics {
        self.inner.clone()
    }

    pub fn entry<T: AsRef<NetworkStatisticsEntry>>(&mut self, entry: T) -> &mut Self {
        self.inner.entry = entry.as_ref().clone();
        self
    }
}

impl AsRef<AddNetworkStatistics> for AddNetworkStatistics {
    fn as_ref(&self) -> &AddNetworkStatistics {
        self
    }
}

impl AsRef<AddNetworkStatistics> for AddNetworkStatisticsBuilder {
    fn as_ref(&self) -> &AddNetworkStatistics {
        &self.inner
    }
}
