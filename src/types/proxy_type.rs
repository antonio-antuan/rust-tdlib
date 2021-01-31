use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the type of a proxy server
pub trait TDProxyType: Debug + RObject {}

/// Describes the type of a proxy server
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ProxyType {
    #[doc(hidden)]
    _Default(()),
    /// A HTTP transparent proxy server
    Http(ProxyTypeHttp),
    /// An MTProto proxy server
    Mtproto(ProxyTypeMtproto),
    /// A SOCKS5 proxy server
    Socks5(ProxyTypeSocks5),
}

impl Default for ProxyType {
    fn default() -> Self {
        ProxyType::_Default(())
    }
}

impl<'de> Deserialize<'de> for ProxyType {
    fn deserialize<D>(deserializer: D) -> Result<ProxyType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          ProxyType,
          (proxyTypeHttp, Http);
          (proxyTypeMtproto, Mtproto);
          (proxyTypeSocks5, Socks5);

        )(deserializer)
    }
}

impl RObject for ProxyType {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            ProxyType::Http(t) => t.td_name(),
            ProxyType::Mtproto(t) => t.td_name(),
            ProxyType::Socks5(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            ProxyType::Http(t) => t.extra(),
            ProxyType::Mtproto(t) => t.extra(),
            ProxyType::Socks5(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl ProxyType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ProxyType::_Default(_))
    }
}

impl AsRef<ProxyType> for ProxyType {
    fn as_ref(&self) -> &ProxyType {
        self
    }
}

/// A HTTP transparent proxy server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProxyTypeHttp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Username for logging in; may be empty
    username: String,
    /// Password for logging in; may be empty
    password: String,
    /// Pass true if the proxy supports only HTTP requests and doesn't support transparent TCP connections via HTTP CONNECT method
    http_only: bool,
}

impl RObject for ProxyTypeHttp {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "proxyTypeHttp"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDProxyType for ProxyTypeHttp {}

impl ProxyTypeHttp {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDProxyTypeHttpBuilder {
        let mut inner = ProxyTypeHttp::default();
        inner.td_name = "proxyTypeHttp".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDProxyTypeHttpBuilder { inner }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn http_only(&self) -> bool {
        self.http_only
    }
}

#[doc(hidden)]
pub struct RTDProxyTypeHttpBuilder {
    inner: ProxyTypeHttp,
}

impl RTDProxyTypeHttpBuilder {
    pub fn build(&self) -> ProxyTypeHttp {
        self.inner.clone()
    }

    pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
        self.inner.username = username.as_ref().to_string();
        self
    }

    pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.inner.password = password.as_ref().to_string();
        self
    }

    pub fn http_only(&mut self, http_only: bool) -> &mut Self {
        self.inner.http_only = http_only;
        self
    }
}

impl AsRef<ProxyTypeHttp> for ProxyTypeHttp {
    fn as_ref(&self) -> &ProxyTypeHttp {
        self
    }
}

impl AsRef<ProxyTypeHttp> for RTDProxyTypeHttpBuilder {
    fn as_ref(&self) -> &ProxyTypeHttp {
        &self.inner
    }
}

/// An MTProto proxy server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProxyTypeMtproto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// The proxy's secret in hexadecimal encoding
    secret: String,
}

impl RObject for ProxyTypeMtproto {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "proxyTypeMtproto"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDProxyType for ProxyTypeMtproto {}

impl ProxyTypeMtproto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDProxyTypeMtprotoBuilder {
        let mut inner = ProxyTypeMtproto::default();
        inner.td_name = "proxyTypeMtproto".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDProxyTypeMtprotoBuilder { inner }
    }

    pub fn secret(&self) -> &String {
        &self.secret
    }
}

#[doc(hidden)]
pub struct RTDProxyTypeMtprotoBuilder {
    inner: ProxyTypeMtproto,
}

impl RTDProxyTypeMtprotoBuilder {
    pub fn build(&self) -> ProxyTypeMtproto {
        self.inner.clone()
    }

    pub fn secret<T: AsRef<str>>(&mut self, secret: T) -> &mut Self {
        self.inner.secret = secret.as_ref().to_string();
        self
    }
}

impl AsRef<ProxyTypeMtproto> for ProxyTypeMtproto {
    fn as_ref(&self) -> &ProxyTypeMtproto {
        self
    }
}

impl AsRef<ProxyTypeMtproto> for RTDProxyTypeMtprotoBuilder {
    fn as_ref(&self) -> &ProxyTypeMtproto {
        &self.inner
    }
}

/// A SOCKS5 proxy server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProxyTypeSocks5 {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Username for logging in; may be empty
    username: String,
    /// Password for logging in; may be empty
    password: String,
}

impl RObject for ProxyTypeSocks5 {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "proxyTypeSocks5"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDProxyType for ProxyTypeSocks5 {}

impl ProxyTypeSocks5 {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDProxyTypeSocks5Builder {
        let mut inner = ProxyTypeSocks5::default();
        inner.td_name = "proxyTypeSocks5".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDProxyTypeSocks5Builder { inner }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

#[doc(hidden)]
pub struct RTDProxyTypeSocks5Builder {
    inner: ProxyTypeSocks5,
}

impl RTDProxyTypeSocks5Builder {
    pub fn build(&self) -> ProxyTypeSocks5 {
        self.inner.clone()
    }

    pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
        self.inner.username = username.as_ref().to_string();
        self
    }

    pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.inner.password = password.as_ref().to_string();
        self
    }
}

impl AsRef<ProxyTypeSocks5> for ProxyTypeSocks5 {
    fn as_ref(&self) -> &ProxyTypeSocks5 {
        self
    }
}

impl AsRef<ProxyTypeSocks5> for RTDProxyTypeSocks5Builder {
    fn as_ref(&self) -> &ProxyTypeSocks5 {
        &self.inner
    }
}
