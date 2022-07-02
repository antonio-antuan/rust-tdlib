use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns list of proxies that are currently set up. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetProxies {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetProxies {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetProxies {}

impl GetProxies {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetProxiesBuilder {
        let mut inner = GetProxies::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getProxies".to_string();

        GetProxiesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetProxiesBuilder {
    inner: GetProxies,
}

#[deprecated]
pub type RTDGetProxiesBuilder = GetProxiesBuilder;

impl GetProxiesBuilder {
    pub fn build(&self) -> GetProxies {
        self.inner.clone()
    }
}

impl AsRef<GetProxies> for GetProxies {
    fn as_ref(&self) -> &GetProxies {
        self
    }
}

impl AsRef<GetProxies> for GetProxiesBuilder {
    fn as_ref(&self) -> &GetProxies {
        &self.inner
    }
}
