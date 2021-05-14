use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Edits an existing proxy server for network requests. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditProxy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Proxy identifier
    proxy_id: i32,
    /// Proxy server IP address
    server: String,
    /// Proxy server port
    port: i32,
    /// True, if the proxy should be enabled
    enable: bool,
    /// Proxy type

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "ProxyType::_is_default")]
    type_: ProxyType,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditProxy {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditProxy {}

impl EditProxy {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDEditProxyBuilder {
        let mut inner = EditProxy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editProxy".to_string();

        RTDEditProxyBuilder { inner }
    }

    pub fn proxy_id(&self) -> i32 {
        self.proxy_id
    }

    pub fn server(&self) -> &String {
        &self.server
    }

    pub fn port(&self) -> i32 {
        self.port
    }

    pub fn enable(&self) -> bool {
        self.enable
    }

    pub fn type_(&self) -> &ProxyType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct RTDEditProxyBuilder {
    inner: EditProxy,
}

impl RTDEditProxyBuilder {
    pub fn build(&self) -> EditProxy {
        self.inner.clone()
    }

    pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self {
        self.inner.proxy_id = proxy_id;
        self
    }

    pub fn server<T: AsRef<str>>(&mut self, server: T) -> &mut Self {
        self.inner.server = server.as_ref().to_string();
        self
    }

    pub fn port(&mut self, port: i32) -> &mut Self {
        self.inner.port = port;
        self
    }

    pub fn enable(&mut self, enable: bool) -> &mut Self {
        self.inner.enable = enable;
        self
    }

    pub fn type_<T: AsRef<ProxyType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<EditProxy> for EditProxy {
    fn as_ref(&self) -> &EditProxy {
        self
    }
}

impl AsRef<EditProxy> for RTDEditProxyBuilder {
    fn as_ref(&self) -> &EditProxy {
        &self.inner
    }
}
