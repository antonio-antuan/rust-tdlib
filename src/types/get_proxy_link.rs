use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns an HTTPS link, which can be used to add a proxy. Available only for SOCKS5 and MTProto proxies. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetProxyLink {
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

impl RObject for GetProxyLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetProxyLink {}

impl GetProxyLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetProxyLinkBuilder {
        let mut inner = GetProxyLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getProxyLink".to_string();

        RTDGetProxyLinkBuilder { inner }
    }

    pub fn proxy_id(&self) -> i32 {
        self.proxy_id
    }
}

#[doc(hidden)]
pub struct RTDGetProxyLinkBuilder {
    inner: GetProxyLink,
}

impl RTDGetProxyLinkBuilder {
    pub fn build(&self) -> GetProxyLink {
        self.inner.clone()
    }

    pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self {
        self.inner.proxy_id = proxy_id;
        self
    }
}

impl AsRef<GetProxyLink> for GetProxyLink {
    fn as_ref(&self) -> &GetProxyLink {
        self
    }
}

impl AsRef<GetProxyLink> for RTDGetProxyLinkBuilder {
    fn as_ref(&self) -> &GetProxyLink {
        &self.inner
    }
}
