use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Enables a proxy. Only one proxy can be enabled at a time. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnableProxy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Proxy identifier

    #[serde(default)]
    proxy_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EnableProxy {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EnableProxy {}

impl EnableProxy {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EnableProxyBuilder {
        let mut inner = EnableProxy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "enableProxy".to_string();

        EnableProxyBuilder { inner }
    }

    pub fn proxy_id(&self) -> i32 {
        self.proxy_id
    }
}

#[doc(hidden)]
pub struct EnableProxyBuilder {
    inner: EnableProxy,
}

#[deprecated]
pub type RTDEnableProxyBuilder = EnableProxyBuilder;

impl EnableProxyBuilder {
    pub fn build(&self) -> EnableProxy {
        self.inner.clone()
    }

    pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self {
        self.inner.proxy_id = proxy_id;
        self
    }
}

impl AsRef<EnableProxy> for EnableProxy {
    fn as_ref(&self) -> &EnableProxy {
        self
    }
}

impl AsRef<EnableProxy> for EnableProxyBuilder {
    fn as_ref(&self) -> &EnableProxy {
        &self.inner
    }
}
