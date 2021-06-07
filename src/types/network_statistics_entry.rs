use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains statistics about network usage
pub trait TDNetworkStatisticsEntry: Debug + RObject {}

/// Contains statistics about network usage
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NetworkStatisticsEntry {
    #[doc(hidden)]
    _Default,
    /// Contains information about the total amount of data that was used for calls
    #[serde(rename(
        serialize = "networkStatisticsEntryCall",
        deserialize = "networkStatisticsEntryCall"
    ))]
    Call(NetworkStatisticsEntryCall),
    /// Contains information about the total amount of data that was used to send and receive files
    #[serde(rename(
        serialize = "networkStatisticsEntryFile",
        deserialize = "networkStatisticsEntryFile"
    ))]
    File(NetworkStatisticsEntryFile),
}

impl Default for NetworkStatisticsEntry {
    fn default() -> Self {
        NetworkStatisticsEntry::_Default
    }
}

impl RObject for NetworkStatisticsEntry {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            NetworkStatisticsEntry::Call(t) => t.extra(),
            NetworkStatisticsEntry::File(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            NetworkStatisticsEntry::Call(t) => t.client_id(),
            NetworkStatisticsEntry::File(t) => t.client_id(),

            _ => None,
        }
    }
}

impl NetworkStatisticsEntry {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, NetworkStatisticsEntry::_Default)
    }
}

impl AsRef<NetworkStatisticsEntry> for NetworkStatisticsEntry {
    fn as_ref(&self) -> &NetworkStatisticsEntry {
        self
    }
}

/// Contains information about the total amount of data that was used for calls
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkStatisticsEntryCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the network the data was sent through. Call setNetworkType to maintain the actual network type

    #[serde(skip_serializing_if = "NetworkType::_is_default")]
    network_type: NetworkType,
    /// Total number of bytes sent
    sent_bytes: i64,
    /// Total number of bytes received
    received_bytes: i64,
    /// Total call duration, in seconds
    duration: f32,
}

impl RObject for NetworkStatisticsEntryCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNetworkStatisticsEntry for NetworkStatisticsEntryCall {}

impl NetworkStatisticsEntryCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNetworkStatisticsEntryCallBuilder {
        let mut inner = NetworkStatisticsEntryCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDNetworkStatisticsEntryCallBuilder { inner }
    }

    pub fn network_type(&self) -> &NetworkType {
        &self.network_type
    }

    pub fn sent_bytes(&self) -> i64 {
        self.sent_bytes
    }

    pub fn received_bytes(&self) -> i64 {
        self.received_bytes
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }
}

#[doc(hidden)]
pub struct RTDNetworkStatisticsEntryCallBuilder {
    inner: NetworkStatisticsEntryCall,
}

impl RTDNetworkStatisticsEntryCallBuilder {
    pub fn build(&self) -> NetworkStatisticsEntryCall {
        self.inner.clone()
    }

    pub fn network_type<T: AsRef<NetworkType>>(&mut self, network_type: T) -> &mut Self {
        self.inner.network_type = network_type.as_ref().clone();
        self
    }

    pub fn sent_bytes(&mut self, sent_bytes: i64) -> &mut Self {
        self.inner.sent_bytes = sent_bytes;
        self
    }

    pub fn received_bytes(&mut self, received_bytes: i64) -> &mut Self {
        self.inner.received_bytes = received_bytes;
        self
    }

    pub fn duration(&mut self, duration: f32) -> &mut Self {
        self.inner.duration = duration;
        self
    }
}

impl AsRef<NetworkStatisticsEntryCall> for NetworkStatisticsEntryCall {
    fn as_ref(&self) -> &NetworkStatisticsEntryCall {
        self
    }
}

impl AsRef<NetworkStatisticsEntryCall> for RTDNetworkStatisticsEntryCallBuilder {
    fn as_ref(&self) -> &NetworkStatisticsEntryCall {
        &self.inner
    }
}

/// Contains information about the total amount of data that was used to send and receive files
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkStatisticsEntryFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the file the data is part of

    #[serde(skip_serializing_if = "FileType::_is_default")]
    file_type: FileType,
    /// Type of the network the data was sent through. Call setNetworkType to maintain the actual network type

    #[serde(skip_serializing_if = "NetworkType::_is_default")]
    network_type: NetworkType,
    /// Total number of bytes sent
    sent_bytes: i64,
    /// Total number of bytes received
    received_bytes: i64,
}

impl RObject for NetworkStatisticsEntryFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNetworkStatisticsEntry for NetworkStatisticsEntryFile {}

impl NetworkStatisticsEntryFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNetworkStatisticsEntryFileBuilder {
        let mut inner = NetworkStatisticsEntryFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDNetworkStatisticsEntryFileBuilder { inner }
    }

    pub fn file_type(&self) -> &FileType {
        &self.file_type
    }

    pub fn network_type(&self) -> &NetworkType {
        &self.network_type
    }

    pub fn sent_bytes(&self) -> i64 {
        self.sent_bytes
    }

    pub fn received_bytes(&self) -> i64 {
        self.received_bytes
    }
}

#[doc(hidden)]
pub struct RTDNetworkStatisticsEntryFileBuilder {
    inner: NetworkStatisticsEntryFile,
}

impl RTDNetworkStatisticsEntryFileBuilder {
    pub fn build(&self) -> NetworkStatisticsEntryFile {
        self.inner.clone()
    }

    pub fn file_type<T: AsRef<FileType>>(&mut self, file_type: T) -> &mut Self {
        self.inner.file_type = file_type.as_ref().clone();
        self
    }

    pub fn network_type<T: AsRef<NetworkType>>(&mut self, network_type: T) -> &mut Self {
        self.inner.network_type = network_type.as_ref().clone();
        self
    }

    pub fn sent_bytes(&mut self, sent_bytes: i64) -> &mut Self {
        self.inner.sent_bytes = sent_bytes;
        self
    }

    pub fn received_bytes(&mut self, received_bytes: i64) -> &mut Self {
        self.inner.received_bytes = received_bytes;
        self
    }
}

impl AsRef<NetworkStatisticsEntryFile> for NetworkStatisticsEntryFile {
    fn as_ref(&self) -> &NetworkStatisticsEntryFile {
        self
    }
}

impl AsRef<NetworkStatisticsEntryFile> for RTDNetworkStatisticsEntryFileBuilder {
    fn as_ref(&self) -> &NetworkStatisticsEntryFile {
        &self.inner
    }
}
