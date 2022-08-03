use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents the type of a network
pub trait TDNetworkType: Debug + RObject {}

/// Represents the type of a network
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NetworkType {
    #[doc(hidden)]
    _Default,
    /// A mobile network
    #[serde(rename = "networkTypeMobile")]
    Mobile(NetworkTypeMobile),
    /// A mobile roaming network
    #[serde(rename = "networkTypeMobileRoaming")]
    MobileRoaming(NetworkTypeMobileRoaming),
    /// The network is not available
    #[serde(rename = "networkTypeNone")]
    None(NetworkTypeNone),
    /// A different network type (e.g., Ethernet network)
    #[serde(rename = "networkTypeOther")]
    Other(NetworkTypeOther),
    /// A Wi-Fi network
    #[serde(rename = "networkTypeWiFi")]
    WiFi(NetworkTypeWiFi),
}

impl Default for NetworkType {
    fn default() -> Self {
        NetworkType::_Default
    }
}

impl RObject for NetworkType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            NetworkType::Mobile(t) => t.extra(),
            NetworkType::MobileRoaming(t) => t.extra(),
            NetworkType::None(t) => t.extra(),
            NetworkType::Other(t) => t.extra(),
            NetworkType::WiFi(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            NetworkType::Mobile(t) => t.client_id(),
            NetworkType::MobileRoaming(t) => t.client_id(),
            NetworkType::None(t) => t.client_id(),
            NetworkType::Other(t) => t.client_id(),
            NetworkType::WiFi(t) => t.client_id(),

            _ => None,
        }
    }
}

impl NetworkType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, NetworkType::_Default)
    }
}

impl AsRef<NetworkType> for NetworkType {
    fn as_ref(&self) -> &NetworkType {
        self
    }
}

/// A mobile network
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkTypeMobile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for NetworkTypeMobile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNetworkType for NetworkTypeMobile {}

impl NetworkTypeMobile {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NetworkTypeMobileBuilder {
        let mut inner = NetworkTypeMobile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NetworkTypeMobileBuilder { inner }
    }
}

#[doc(hidden)]
pub struct NetworkTypeMobileBuilder {
    inner: NetworkTypeMobile,
}

#[deprecated]
pub type RTDNetworkTypeMobileBuilder = NetworkTypeMobileBuilder;

impl NetworkTypeMobileBuilder {
    pub fn build(&self) -> NetworkTypeMobile {
        self.inner.clone()
    }
}

impl AsRef<NetworkTypeMobile> for NetworkTypeMobile {
    fn as_ref(&self) -> &NetworkTypeMobile {
        self
    }
}

impl AsRef<NetworkTypeMobile> for NetworkTypeMobileBuilder {
    fn as_ref(&self) -> &NetworkTypeMobile {
        &self.inner
    }
}

/// A mobile roaming network
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkTypeMobileRoaming {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for NetworkTypeMobileRoaming {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNetworkType for NetworkTypeMobileRoaming {}

impl NetworkTypeMobileRoaming {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NetworkTypeMobileRoamingBuilder {
        let mut inner = NetworkTypeMobileRoaming::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NetworkTypeMobileRoamingBuilder { inner }
    }
}

#[doc(hidden)]
pub struct NetworkTypeMobileRoamingBuilder {
    inner: NetworkTypeMobileRoaming,
}

#[deprecated]
pub type RTDNetworkTypeMobileRoamingBuilder = NetworkTypeMobileRoamingBuilder;

impl NetworkTypeMobileRoamingBuilder {
    pub fn build(&self) -> NetworkTypeMobileRoaming {
        self.inner.clone()
    }
}

impl AsRef<NetworkTypeMobileRoaming> for NetworkTypeMobileRoaming {
    fn as_ref(&self) -> &NetworkTypeMobileRoaming {
        self
    }
}

impl AsRef<NetworkTypeMobileRoaming> for NetworkTypeMobileRoamingBuilder {
    fn as_ref(&self) -> &NetworkTypeMobileRoaming {
        &self.inner
    }
}

/// The network is not available
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkTypeNone {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for NetworkTypeNone {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNetworkType for NetworkTypeNone {}

impl NetworkTypeNone {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NetworkTypeNoneBuilder {
        let mut inner = NetworkTypeNone::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NetworkTypeNoneBuilder { inner }
    }
}

#[doc(hidden)]
pub struct NetworkTypeNoneBuilder {
    inner: NetworkTypeNone,
}

#[deprecated]
pub type RTDNetworkTypeNoneBuilder = NetworkTypeNoneBuilder;

impl NetworkTypeNoneBuilder {
    pub fn build(&self) -> NetworkTypeNone {
        self.inner.clone()
    }
}

impl AsRef<NetworkTypeNone> for NetworkTypeNone {
    fn as_ref(&self) -> &NetworkTypeNone {
        self
    }
}

impl AsRef<NetworkTypeNone> for NetworkTypeNoneBuilder {
    fn as_ref(&self) -> &NetworkTypeNone {
        &self.inner
    }
}

/// A different network type (e.g., Ethernet network)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkTypeOther {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for NetworkTypeOther {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNetworkType for NetworkTypeOther {}

impl NetworkTypeOther {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NetworkTypeOtherBuilder {
        let mut inner = NetworkTypeOther::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NetworkTypeOtherBuilder { inner }
    }
}

#[doc(hidden)]
pub struct NetworkTypeOtherBuilder {
    inner: NetworkTypeOther,
}

#[deprecated]
pub type RTDNetworkTypeOtherBuilder = NetworkTypeOtherBuilder;

impl NetworkTypeOtherBuilder {
    pub fn build(&self) -> NetworkTypeOther {
        self.inner.clone()
    }
}

impl AsRef<NetworkTypeOther> for NetworkTypeOther {
    fn as_ref(&self) -> &NetworkTypeOther {
        self
    }
}

impl AsRef<NetworkTypeOther> for NetworkTypeOtherBuilder {
    fn as_ref(&self) -> &NetworkTypeOther {
        &self.inner
    }
}

/// A Wi-Fi network
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkTypeWiFi {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for NetworkTypeWiFi {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNetworkType for NetworkTypeWiFi {}

impl NetworkTypeWiFi {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NetworkTypeWiFiBuilder {
        let mut inner = NetworkTypeWiFi::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NetworkTypeWiFiBuilder { inner }
    }
}

#[doc(hidden)]
pub struct NetworkTypeWiFiBuilder {
    inner: NetworkTypeWiFi,
}

#[deprecated]
pub type RTDNetworkTypeWiFiBuilder = NetworkTypeWiFiBuilder;

impl NetworkTypeWiFiBuilder {
    pub fn build(&self) -> NetworkTypeWiFi {
        self.inner.clone()
    }
}

impl AsRef<NetworkTypeWiFi> for NetworkTypeWiFi {
    fn as_ref(&self) -> &NetworkTypeWiFi {
        self
    }
}

impl AsRef<NetworkTypeWiFi> for NetworkTypeWiFiBuilder {
    fn as_ref(&self) -> &NetworkTypeWiFi {
        &self.inner
    }
}
