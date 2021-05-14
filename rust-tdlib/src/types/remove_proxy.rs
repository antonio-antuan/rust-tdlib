use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Removes a proxy server. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveProxy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Proxy identifier
    proxy_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveProxy {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveProxy {}

impl RemoveProxy {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRemoveProxyBuilder {
        let mut inner = RemoveProxy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeProxy".to_string();

        RTDRemoveProxyBuilder { inner }
    }

    pub fn proxy_id(&self) -> i32 {
        self.proxy_id
    }
}

#[doc(hidden)]
pub struct RTDRemoveProxyBuilder {
    inner: RemoveProxy,
}

impl RTDRemoveProxyBuilder {
    pub fn build(&self) -> RemoveProxy {
        self.inner.clone()
    }

    pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self {
        self.inner.proxy_id = proxy_id;
        self
    }
}

impl AsRef<RemoveProxy> for RemoveProxy {
    fn as_ref(&self) -> &RemoveProxy {
        self
    }
}

impl AsRef<RemoveProxy> for RTDRemoveProxyBuilder {
    fn as_ref(&self) -> &RemoveProxy {
        &self.inner
    }
}
