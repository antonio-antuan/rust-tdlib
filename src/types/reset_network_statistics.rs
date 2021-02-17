use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Resets all network data usage statistics to zero. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetNetworkStatistics {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ResetNetworkStatistics {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ResetNetworkStatistics {}

impl ResetNetworkStatistics {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDResetNetworkStatisticsBuilder {
        let mut inner = ResetNetworkStatistics::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resetNetworkStatistics".to_string();

        RTDResetNetworkStatisticsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDResetNetworkStatisticsBuilder {
    inner: ResetNetworkStatistics,
}

impl RTDResetNetworkStatisticsBuilder {
    pub fn build(&self) -> ResetNetworkStatistics {
        self.inner.clone()
    }
}

impl AsRef<ResetNetworkStatistics> for ResetNetworkStatistics {
    fn as_ref(&self) -> &ResetNetworkStatistics {
        self
    }
}

impl AsRef<ResetNetworkStatistics> for RTDResetNetworkStatisticsBuilder {
    fn as_ref(&self) -> &ResetNetworkStatistics {
        &self.inner
    }
}
