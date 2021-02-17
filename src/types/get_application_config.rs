use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns application config, provided by the server. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetApplicationConfig {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetApplicationConfig {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDJsonValue for GetApplicationConfig {}

impl RFunction for GetApplicationConfig {}

impl GetApplicationConfig {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetApplicationConfigBuilder {
        let mut inner = GetApplicationConfig::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getApplicationConfig".to_string();

        RTDGetApplicationConfigBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetApplicationConfigBuilder {
    inner: GetApplicationConfig,
}

impl RTDGetApplicationConfigBuilder {
    pub fn build(&self) -> GetApplicationConfig {
        self.inner.clone()
    }
}

impl AsRef<GetApplicationConfig> for GetApplicationConfig {
    fn as_ref(&self) -> &GetApplicationConfig {
        self
    }
}

impl AsRef<GetApplicationConfig> for RTDGetApplicationConfigBuilder {
    fn as_ref(&self) -> &GetApplicationConfig {
        &self.inner
    }
}
