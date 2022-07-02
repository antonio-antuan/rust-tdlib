use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of proxy servers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Proxies {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of proxy servers

    #[serde(default)]
    proxies: Vec<Proxy>,
}

impl RObject for Proxies {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Proxies {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ProxiesBuilder {
        let mut inner = Proxies::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ProxiesBuilder { inner }
    }

    pub fn proxies(&self) -> &Vec<Proxy> {
        &self.proxies
    }
}

#[doc(hidden)]
pub struct ProxiesBuilder {
    inner: Proxies,
}

#[deprecated]
pub type RTDProxiesBuilder = ProxiesBuilder;

impl ProxiesBuilder {
    pub fn build(&self) -> Proxies {
        self.inner.clone()
    }

    pub fn proxies(&mut self, proxies: Vec<Proxy>) -> &mut Self {
        self.inner.proxies = proxies;
        self
    }
}

impl AsRef<Proxies> for Proxies {
    fn as_ref(&self) -> &Proxies {
        self
    }
}

impl AsRef<Proxies> for ProxiesBuilder {
    fn as_ref(&self) -> &Proxies {
        &self.inner
    }
}
