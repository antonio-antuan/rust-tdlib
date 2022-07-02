use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns network data usage statistics. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetNetworkStatistics {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// If true, returns only data for the current library launch

    #[serde(default)]
    only_current: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetNetworkStatistics {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetNetworkStatistics {}

impl GetNetworkStatistics {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetNetworkStatisticsBuilder {
        let mut inner = GetNetworkStatistics::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getNetworkStatistics".to_string();

        GetNetworkStatisticsBuilder { inner }
    }

    pub fn only_current(&self) -> bool {
        self.only_current
    }
}

#[doc(hidden)]
pub struct GetNetworkStatisticsBuilder {
    inner: GetNetworkStatistics,
}

#[deprecated]
pub type RTDGetNetworkStatisticsBuilder = GetNetworkStatisticsBuilder;

impl GetNetworkStatisticsBuilder {
    pub fn build(&self) -> GetNetworkStatistics {
        self.inner.clone()
    }

    pub fn only_current(&mut self, only_current: bool) -> &mut Self {
        self.inner.only_current = only_current;
        self
    }
}

impl AsRef<GetNetworkStatistics> for GetNetworkStatistics {
    fn as_ref(&self) -> &GetNetworkStatistics {
        self
    }
}

impl AsRef<GetNetworkStatistics> for GetNetworkStatisticsBuilder {
    fn as_ref(&self) -> &GetNetworkStatistics {
        &self.inner
    }
}
