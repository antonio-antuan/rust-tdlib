use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sends a simple network request to the Telegram servers via proxy; for testing only. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestProxy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Proxy server IP address

    #[serde(default)]
    server: String,
    /// Proxy server port

    #[serde(default)]
    port: i32,
    /// Proxy type

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "ProxyType::_is_default")]
    type_: ProxyType,
    /// Identifier of a datacenter, with which to test connection

    #[serde(default)]
    dc_id: i32,
    /// The maximum overall timeout for the request

    #[serde(default)]
    timeout: f32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TestProxy {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TestProxy {}

impl TestProxy {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TestProxyBuilder {
        let mut inner = TestProxy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "testProxy".to_string();

        TestProxyBuilder { inner }
    }

    pub fn server(&self) -> &String {
        &self.server
    }

    pub fn port(&self) -> i32 {
        self.port
    }

    pub fn type_(&self) -> &ProxyType {
        &self.type_
    }

    pub fn dc_id(&self) -> i32 {
        self.dc_id
    }

    pub fn timeout(&self) -> f32 {
        self.timeout
    }
}

#[doc(hidden)]
pub struct TestProxyBuilder {
    inner: TestProxy,
}

#[deprecated]
pub type RTDTestProxyBuilder = TestProxyBuilder;

impl TestProxyBuilder {
    pub fn build(&self) -> TestProxy {
        self.inner.clone()
    }

    pub fn server<T: AsRef<str>>(&mut self, server: T) -> &mut Self {
        self.inner.server = server.as_ref().to_string();
        self
    }

    pub fn port(&mut self, port: i32) -> &mut Self {
        self.inner.port = port;
        self
    }

    pub fn type_<T: AsRef<ProxyType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn dc_id(&mut self, dc_id: i32) -> &mut Self {
        self.inner.dc_id = dc_id;
        self
    }

    pub fn timeout(&mut self, timeout: f32) -> &mut Self {
        self.inner.timeout = timeout;
        self
    }
}

impl AsRef<TestProxy> for TestProxy {
    fn as_ref(&self) -> &TestProxy {
        self
    }
}

impl AsRef<TestProxy> for TestProxyBuilder {
    fn as_ref(&self) -> &TestProxy {
        &self.inner
    }
}
