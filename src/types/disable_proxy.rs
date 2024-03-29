use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Disables the currently enabled proxy. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisableProxy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DisableProxy {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DisableProxy {}

impl DisableProxy {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DisableProxyBuilder {
        let mut inner = DisableProxy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "disableProxy".to_string();

        DisableProxyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct DisableProxyBuilder {
    inner: DisableProxy,
}

#[deprecated]
pub type RTDDisableProxyBuilder = DisableProxyBuilder;

impl DisableProxyBuilder {
    pub fn build(&self) -> DisableProxy {
        self.inner.clone()
    }
}

impl AsRef<DisableProxy> for DisableProxy {
    fn as_ref(&self) -> &DisableProxy {
        self
    }
}

impl AsRef<DisableProxy> for DisableProxyBuilder {
    fn as_ref(&self) -> &DisableProxy {
        &self.inner
    }
}
