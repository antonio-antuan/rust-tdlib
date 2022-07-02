use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a proxy server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Proxy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the proxy

    #[serde(default)]
    id: i32,
    /// Proxy server IP address

    #[serde(default)]
    server: String,
    /// Proxy server port

    #[serde(default)]
    port: i32,
    /// Point in time (Unix timestamp) when the proxy was last used; 0 if never

    #[serde(default)]
    last_used_date: i32,
    /// True, if the proxy is enabled now

    #[serde(default)]
    is_enabled: bool,
    /// Type of the proxy

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "ProxyType::_is_default")]
    type_: ProxyType,
}

impl RObject for Proxy {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Proxy {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ProxyBuilder {
        let mut inner = Proxy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ProxyBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn server(&self) -> &String {
        &self.server
    }

    pub fn port(&self) -> i32 {
        self.port
    }

    pub fn last_used_date(&self) -> i32 {
        self.last_used_date
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn type_(&self) -> &ProxyType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct ProxyBuilder {
    inner: Proxy,
}

#[deprecated]
pub type RTDProxyBuilder = ProxyBuilder;

impl ProxyBuilder {
    pub fn build(&self) -> Proxy {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
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

    pub fn last_used_date(&mut self, last_used_date: i32) -> &mut Self {
        self.inner.last_used_date = last_used_date;
        self
    }

    pub fn is_enabled(&mut self, is_enabled: bool) -> &mut Self {
        self.inner.is_enabled = is_enabled;
        self
    }

    pub fn type_<T: AsRef<ProxyType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<Proxy> for Proxy {
    fn as_ref(&self) -> &Proxy {
        self
    }
}

impl AsRef<Proxy> for ProxyBuilder {
    fn as_ref(&self) -> &Proxy {
        &self.inner
    }
}
