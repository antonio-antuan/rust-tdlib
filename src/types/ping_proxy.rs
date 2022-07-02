use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Computes time needed to receive a response from a Telegram server through a proxy. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PingProxy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Proxy identifier. Use 0 to ping a Telegram server without a proxy

    #[serde(default)]
    proxy_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for PingProxy {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for PingProxy {}

impl PingProxy {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PingProxyBuilder {
        let mut inner = PingProxy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "pingProxy".to_string();

        PingProxyBuilder { inner }
    }

    pub fn proxy_id(&self) -> i32 {
        self.proxy_id
    }
}

#[doc(hidden)]
pub struct PingProxyBuilder {
    inner: PingProxy,
}

#[deprecated]
pub type RTDPingProxyBuilder = PingProxyBuilder;

impl PingProxyBuilder {
    pub fn build(&self) -> PingProxy {
        self.inner.clone()
    }

    pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self {
        self.inner.proxy_id = proxy_id;
        self
    }
}

impl AsRef<PingProxy> for PingProxy {
    fn as_ref(&self) -> &PingProxy {
        self
    }
}

impl AsRef<PingProxy> for PingProxyBuilder {
    fn as_ref(&self) -> &PingProxy {
        &self.inner
    }
}
