use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents the type of a network
pub trait TDNetworkType: Debug + RObject {}

/// Represents the type of a network
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum NetworkType {
    #[doc(hidden)]
    _Default(()),
    /// A mobile network
    Mobile(NetworkTypeMobile),
    /// A mobile roaming network
    MobileRoaming(NetworkTypeMobileRoaming),
    /// The network is not available
    None(NetworkTypeNone),
    /// A different network type (e.g., Ethernet network)
    Other(NetworkTypeOther),
    /// A Wi-Fi network
    WiFi(NetworkTypeWiFi),
}

impl Default for NetworkType {
    fn default() -> Self {
        NetworkType::_Default(())
    }
}

impl<'de> Deserialize<'de> for NetworkType {
    fn deserialize<D>(deserializer: D) -> Result<NetworkType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          NetworkType,
          (networkTypeMobile, Mobile);
          (networkTypeMobileRoaming, MobileRoaming);
          (networkTypeNone, None);
          (networkTypeOther, Other);
          (networkTypeWiFi, WiFi);

        )(deserializer)
    }
}

impl RObject for NetworkType {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            NetworkType::Mobile(t) => t.td_name(),
            NetworkType::MobileRoaming(t) => t.td_name(),
            NetworkType::None(t) => t.td_name(),
            NetworkType::Other(t) => t.td_name(),
            NetworkType::WiFi(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            NetworkType::Mobile(t) => t.extra(),
            NetworkType::MobileRoaming(t) => t.extra(),
            NetworkType::None(t) => t.extra(),
            NetworkType::Other(t) => t.extra(),
            NetworkType::WiFi(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl NetworkType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, NetworkType::_Default(_))
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for NetworkTypeMobile {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "networkTypeMobile"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDNetworkType for NetworkTypeMobile {}

impl NetworkTypeMobile {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNetworkTypeMobileBuilder {
        let mut inner = NetworkTypeMobile::default();
        inner.td_name = "networkTypeMobile".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDNetworkTypeMobileBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDNetworkTypeMobileBuilder {
    inner: NetworkTypeMobile,
}

impl RTDNetworkTypeMobileBuilder {
    pub fn build(&self) -> NetworkTypeMobile {
        self.inner.clone()
    }
}

impl AsRef<NetworkTypeMobile> for NetworkTypeMobile {
    fn as_ref(&self) -> &NetworkTypeMobile {
        self
    }
}

impl AsRef<NetworkTypeMobile> for RTDNetworkTypeMobileBuilder {
    fn as_ref(&self) -> &NetworkTypeMobile {
        &self.inner
    }
}

/// A mobile roaming network
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkTypeMobileRoaming {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for NetworkTypeMobileRoaming {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "networkTypeMobileRoaming"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDNetworkType for NetworkTypeMobileRoaming {}

impl NetworkTypeMobileRoaming {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNetworkTypeMobileRoamingBuilder {
        let mut inner = NetworkTypeMobileRoaming::default();
        inner.td_name = "networkTypeMobileRoaming".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDNetworkTypeMobileRoamingBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDNetworkTypeMobileRoamingBuilder {
    inner: NetworkTypeMobileRoaming,
}

impl RTDNetworkTypeMobileRoamingBuilder {
    pub fn build(&self) -> NetworkTypeMobileRoaming {
        self.inner.clone()
    }
}

impl AsRef<NetworkTypeMobileRoaming> for NetworkTypeMobileRoaming {
    fn as_ref(&self) -> &NetworkTypeMobileRoaming {
        self
    }
}

impl AsRef<NetworkTypeMobileRoaming> for RTDNetworkTypeMobileRoamingBuilder {
    fn as_ref(&self) -> &NetworkTypeMobileRoaming {
        &self.inner
    }
}

/// The network is not available
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkTypeNone {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for NetworkTypeNone {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "networkTypeNone"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDNetworkType for NetworkTypeNone {}

impl NetworkTypeNone {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNetworkTypeNoneBuilder {
        let mut inner = NetworkTypeNone::default();
        inner.td_name = "networkTypeNone".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDNetworkTypeNoneBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDNetworkTypeNoneBuilder {
    inner: NetworkTypeNone,
}

impl RTDNetworkTypeNoneBuilder {
    pub fn build(&self) -> NetworkTypeNone {
        self.inner.clone()
    }
}

impl AsRef<NetworkTypeNone> for NetworkTypeNone {
    fn as_ref(&self) -> &NetworkTypeNone {
        self
    }
}

impl AsRef<NetworkTypeNone> for RTDNetworkTypeNoneBuilder {
    fn as_ref(&self) -> &NetworkTypeNone {
        &self.inner
    }
}

/// A different network type (e.g., Ethernet network)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkTypeOther {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for NetworkTypeOther {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "networkTypeOther"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDNetworkType for NetworkTypeOther {}

impl NetworkTypeOther {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNetworkTypeOtherBuilder {
        let mut inner = NetworkTypeOther::default();
        inner.td_name = "networkTypeOther".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDNetworkTypeOtherBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDNetworkTypeOtherBuilder {
    inner: NetworkTypeOther,
}

impl RTDNetworkTypeOtherBuilder {
    pub fn build(&self) -> NetworkTypeOther {
        self.inner.clone()
    }
}

impl AsRef<NetworkTypeOther> for NetworkTypeOther {
    fn as_ref(&self) -> &NetworkTypeOther {
        self
    }
}

impl AsRef<NetworkTypeOther> for RTDNetworkTypeOtherBuilder {
    fn as_ref(&self) -> &NetworkTypeOther {
        &self.inner
    }
}

/// A Wi-Fi network
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkTypeWiFi {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for NetworkTypeWiFi {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "networkTypeWiFi"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDNetworkType for NetworkTypeWiFi {}

impl NetworkTypeWiFi {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNetworkTypeWiFiBuilder {
        let mut inner = NetworkTypeWiFi::default();
        inner.td_name = "networkTypeWiFi".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDNetworkTypeWiFiBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDNetworkTypeWiFiBuilder {
    inner: NetworkTypeWiFi,
}

impl RTDNetworkTypeWiFiBuilder {
    pub fn build(&self) -> NetworkTypeWiFi {
        self.inner.clone()
    }
}

impl AsRef<NetworkTypeWiFi> for NetworkTypeWiFi {
    fn as_ref(&self) -> &NetworkTypeWiFi {
        self
    }
}

impl AsRef<NetworkTypeWiFi> for RTDNetworkTypeWiFiBuilder {
    fn as_ref(&self) -> &NetworkTypeWiFi {
        &self.inner
    }
}
