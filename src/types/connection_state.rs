use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the current state of the connection to Telegram servers
pub trait TDConnectionState: Debug + RObject {}

/// Describes the current state of the connection to Telegram servers
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ConnectionState {
    #[doc(hidden)]
    _Default(()),
    /// Currently establishing a connection to the Telegram servers
    Connecting(ConnectionStateConnecting),
    /// Currently establishing a connection with a proxy server
    ConnectingToProxy(ConnectionStateConnectingToProxy),
    /// There is a working connection to the Telegram servers
    Ready(ConnectionStateReady),
    /// Downloading data received while the application was offline
    Updating(ConnectionStateUpdating),
    /// Currently waiting for the network to become available. Use setNetworkType to change the available network type
    WaitingForNetwork(ConnectionStateWaitingForNetwork),
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState::_Default(())
    }
}

impl<'de> Deserialize<'de> for ConnectionState {
    fn deserialize<D>(deserializer: D) -> Result<ConnectionState, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          ConnectionState,
          (connectionStateConnecting, Connecting);
          (connectionStateConnectingToProxy, ConnectingToProxy);
          (connectionStateReady, Ready);
          (connectionStateUpdating, Updating);
          (connectionStateWaitingForNetwork, WaitingForNetwork);

        )(deserializer)
    }
}

impl RObject for ConnectionState {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            ConnectionState::Connecting(t) => t.td_name(),
            ConnectionState::ConnectingToProxy(t) => t.td_name(),
            ConnectionState::Ready(t) => t.td_name(),
            ConnectionState::Updating(t) => t.td_name(),
            ConnectionState::WaitingForNetwork(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            ConnectionState::Connecting(t) => t.extra(),
            ConnectionState::ConnectingToProxy(t) => t.extra(),
            ConnectionState::Ready(t) => t.extra(),
            ConnectionState::Updating(t) => t.extra(),
            ConnectionState::WaitingForNetwork(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl ConnectionState {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ConnectionState::_Default(_))
    }
}

impl AsRef<ConnectionState> for ConnectionState {
    fn as_ref(&self) -> &ConnectionState {
        self
    }
}

/// Currently establishing a connection to the Telegram servers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionStateConnecting {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ConnectionStateConnecting {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "connectionStateConnecting"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDConnectionState for ConnectionStateConnecting {}

impl ConnectionStateConnecting {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDConnectionStateConnectingBuilder {
        let mut inner = ConnectionStateConnecting::default();
        inner.td_name = "connectionStateConnecting".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDConnectionStateConnectingBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDConnectionStateConnectingBuilder {
    inner: ConnectionStateConnecting,
}

impl RTDConnectionStateConnectingBuilder {
    pub fn build(&self) -> ConnectionStateConnecting {
        self.inner.clone()
    }
}

impl AsRef<ConnectionStateConnecting> for ConnectionStateConnecting {
    fn as_ref(&self) -> &ConnectionStateConnecting {
        self
    }
}

impl AsRef<ConnectionStateConnecting> for RTDConnectionStateConnectingBuilder {
    fn as_ref(&self) -> &ConnectionStateConnecting {
        &self.inner
    }
}

/// Currently establishing a connection with a proxy server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionStateConnectingToProxy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ConnectionStateConnectingToProxy {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "connectionStateConnectingToProxy"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDConnectionState for ConnectionStateConnectingToProxy {}

impl ConnectionStateConnectingToProxy {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDConnectionStateConnectingToProxyBuilder {
        let mut inner = ConnectionStateConnectingToProxy::default();
        inner.td_name = "connectionStateConnectingToProxy".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDConnectionStateConnectingToProxyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDConnectionStateConnectingToProxyBuilder {
    inner: ConnectionStateConnectingToProxy,
}

impl RTDConnectionStateConnectingToProxyBuilder {
    pub fn build(&self) -> ConnectionStateConnectingToProxy {
        self.inner.clone()
    }
}

impl AsRef<ConnectionStateConnectingToProxy> for ConnectionStateConnectingToProxy {
    fn as_ref(&self) -> &ConnectionStateConnectingToProxy {
        self
    }
}

impl AsRef<ConnectionStateConnectingToProxy> for RTDConnectionStateConnectingToProxyBuilder {
    fn as_ref(&self) -> &ConnectionStateConnectingToProxy {
        &self.inner
    }
}

/// There is a working connection to the Telegram servers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionStateReady {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ConnectionStateReady {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "connectionStateReady"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDConnectionState for ConnectionStateReady {}

impl ConnectionStateReady {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDConnectionStateReadyBuilder {
        let mut inner = ConnectionStateReady::default();
        inner.td_name = "connectionStateReady".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDConnectionStateReadyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDConnectionStateReadyBuilder {
    inner: ConnectionStateReady,
}

impl RTDConnectionStateReadyBuilder {
    pub fn build(&self) -> ConnectionStateReady {
        self.inner.clone()
    }
}

impl AsRef<ConnectionStateReady> for ConnectionStateReady {
    fn as_ref(&self) -> &ConnectionStateReady {
        self
    }
}

impl AsRef<ConnectionStateReady> for RTDConnectionStateReadyBuilder {
    fn as_ref(&self) -> &ConnectionStateReady {
        &self.inner
    }
}

/// Downloading data received while the application was offline
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionStateUpdating {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ConnectionStateUpdating {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "connectionStateUpdating"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDConnectionState for ConnectionStateUpdating {}

impl ConnectionStateUpdating {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDConnectionStateUpdatingBuilder {
        let mut inner = ConnectionStateUpdating::default();
        inner.td_name = "connectionStateUpdating".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDConnectionStateUpdatingBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDConnectionStateUpdatingBuilder {
    inner: ConnectionStateUpdating,
}

impl RTDConnectionStateUpdatingBuilder {
    pub fn build(&self) -> ConnectionStateUpdating {
        self.inner.clone()
    }
}

impl AsRef<ConnectionStateUpdating> for ConnectionStateUpdating {
    fn as_ref(&self) -> &ConnectionStateUpdating {
        self
    }
}

impl AsRef<ConnectionStateUpdating> for RTDConnectionStateUpdatingBuilder {
    fn as_ref(&self) -> &ConnectionStateUpdating {
        &self.inner
    }
}

/// Currently waiting for the network to become available. Use setNetworkType to change the available network type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionStateWaitingForNetwork {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ConnectionStateWaitingForNetwork {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "connectionStateWaitingForNetwork"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDConnectionState for ConnectionStateWaitingForNetwork {}

impl ConnectionStateWaitingForNetwork {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDConnectionStateWaitingForNetworkBuilder {
        let mut inner = ConnectionStateWaitingForNetwork::default();
        inner.td_name = "connectionStateWaitingForNetwork".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDConnectionStateWaitingForNetworkBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDConnectionStateWaitingForNetworkBuilder {
    inner: ConnectionStateWaitingForNetwork,
}

impl RTDConnectionStateWaitingForNetworkBuilder {
    pub fn build(&self) -> ConnectionStateWaitingForNetwork {
        self.inner.clone()
    }
}

impl AsRef<ConnectionStateWaitingForNetwork> for ConnectionStateWaitingForNetwork {
    fn as_ref(&self) -> &ConnectionStateWaitingForNetwork {
        self
    }
}

impl AsRef<ConnectionStateWaitingForNetwork> for RTDConnectionStateWaitingForNetworkBuilder {
    fn as_ref(&self) -> &ConnectionStateWaitingForNetwork {
        &self.inner
    }
}
