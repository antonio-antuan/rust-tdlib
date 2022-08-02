use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the current call state
pub trait TDCallState: Debug + RObject {}

/// Describes the current call state
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallState {
    #[doc(hidden)]
    _Default,
    /// The call has ended successfully
    #[serde(rename = "callStateDiscarded")]
    Discarded(CallStateDiscarded),
    /// The call has ended with an error
    #[serde(rename = "callStateError")]
    Error(CallStateError),
    /// The call has been answered and encryption keys are being exchanged
    #[serde(rename = "callStateExchangingKeys")]
    ExchangingKeys(CallStateExchangingKeys),
    /// The call is hanging up after discardCall has been called
    #[serde(rename = "callStateHangingUp")]
    HangingUp(CallStateHangingUp),
    /// The call is pending, waiting to be accepted by a user
    #[serde(rename = "callStatePending")]
    Pending(CallStatePending),
    /// The call is ready to use
    #[serde(rename = "callStateReady")]
    Ready(CallStateReady),
}

impl Default for CallState {
    fn default() -> Self {
        CallState::_Default
    }
}

impl RObject for CallState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            CallState::Discarded(t) => t.extra(),
            CallState::Error(t) => t.extra(),
            CallState::ExchangingKeys(t) => t.extra(),
            CallState::HangingUp(t) => t.extra(),
            CallState::Pending(t) => t.extra(),
            CallState::Ready(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            CallState::Discarded(t) => t.client_id(),
            CallState::Error(t) => t.client_id(),
            CallState::ExchangingKeys(t) => t.client_id(),
            CallState::HangingUp(t) => t.client_id(),
            CallState::Pending(t) => t.client_id(),
            CallState::Ready(t) => t.client_id(),

            _ => None,
        }
    }
}

impl CallState {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, CallState::_Default)
    }
}

impl AsRef<CallState> for CallState {
    fn as_ref(&self) -> &CallState {
        self
    }
}

/// The call has ended successfully
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStateDiscarded {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The reason, why the call has ended

    #[serde(skip_serializing_if = "CallDiscardReason::_is_default")]
    reason: CallDiscardReason,
    /// True, if the call rating must be sent to the server

    #[serde(default)]
    need_rating: bool,
    /// True, if the call debug information must be sent to the server

    #[serde(default)]
    need_debug_information: bool,
}

impl RObject for CallStateDiscarded {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallState for CallStateDiscarded {}

impl CallStateDiscarded {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallStateDiscardedBuilder {
        let mut inner = CallStateDiscarded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallStateDiscardedBuilder { inner }
    }

    pub fn reason(&self) -> &CallDiscardReason {
        &self.reason
    }

    pub fn need_rating(&self) -> bool {
        self.need_rating
    }

    pub fn need_debug_information(&self) -> bool {
        self.need_debug_information
    }
}

#[doc(hidden)]
pub struct CallStateDiscardedBuilder {
    inner: CallStateDiscarded,
}

#[deprecated]
pub type RTDCallStateDiscardedBuilder = CallStateDiscardedBuilder;

impl CallStateDiscardedBuilder {
    pub fn build(&self) -> CallStateDiscarded {
        self.inner.clone()
    }

    pub fn reason<T: AsRef<CallDiscardReason>>(&mut self, reason: T) -> &mut Self {
        self.inner.reason = reason.as_ref().clone();
        self
    }

    pub fn need_rating(&mut self, need_rating: bool) -> &mut Self {
        self.inner.need_rating = need_rating;
        self
    }

    pub fn need_debug_information(&mut self, need_debug_information: bool) -> &mut Self {
        self.inner.need_debug_information = need_debug_information;
        self
    }
}

impl AsRef<CallStateDiscarded> for CallStateDiscarded {
    fn as_ref(&self) -> &CallStateDiscarded {
        self
    }
}

impl AsRef<CallStateDiscarded> for CallStateDiscardedBuilder {
    fn as_ref(&self) -> &CallStateDiscarded {
        &self.inner
    }
}

/// The call has ended with an error
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStateError {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Error. An error with the code 4005000 will be returned if an outgoing call is missed because of an expired timeout
    error: Error,
}

impl RObject for CallStateError {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallState for CallStateError {}

impl CallStateError {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallStateErrorBuilder {
        let mut inner = CallStateError::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallStateErrorBuilder { inner }
    }

    pub fn error(&self) -> &Error {
        &self.error
    }
}

#[doc(hidden)]
pub struct CallStateErrorBuilder {
    inner: CallStateError,
}

#[deprecated]
pub type RTDCallStateErrorBuilder = CallStateErrorBuilder;

impl CallStateErrorBuilder {
    pub fn build(&self) -> CallStateError {
        self.inner.clone()
    }

    pub fn error<T: AsRef<Error>>(&mut self, error: T) -> &mut Self {
        self.inner.error = error.as_ref().clone();
        self
    }
}

impl AsRef<CallStateError> for CallStateError {
    fn as_ref(&self) -> &CallStateError {
        self
    }
}

impl AsRef<CallStateError> for CallStateErrorBuilder {
    fn as_ref(&self) -> &CallStateError {
        &self.inner
    }
}

/// The call has been answered and encryption keys are being exchanged
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStateExchangingKeys {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallStateExchangingKeys {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallState for CallStateExchangingKeys {}

impl CallStateExchangingKeys {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallStateExchangingKeysBuilder {
        let mut inner = CallStateExchangingKeys::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallStateExchangingKeysBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallStateExchangingKeysBuilder {
    inner: CallStateExchangingKeys,
}

#[deprecated]
pub type RTDCallStateExchangingKeysBuilder = CallStateExchangingKeysBuilder;

impl CallStateExchangingKeysBuilder {
    pub fn build(&self) -> CallStateExchangingKeys {
        self.inner.clone()
    }
}

impl AsRef<CallStateExchangingKeys> for CallStateExchangingKeys {
    fn as_ref(&self) -> &CallStateExchangingKeys {
        self
    }
}

impl AsRef<CallStateExchangingKeys> for CallStateExchangingKeysBuilder {
    fn as_ref(&self) -> &CallStateExchangingKeys {
        &self.inner
    }
}

/// The call is hanging up after discardCall has been called
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStateHangingUp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallStateHangingUp {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallState for CallStateHangingUp {}

impl CallStateHangingUp {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallStateHangingUpBuilder {
        let mut inner = CallStateHangingUp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallStateHangingUpBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallStateHangingUpBuilder {
    inner: CallStateHangingUp,
}

#[deprecated]
pub type RTDCallStateHangingUpBuilder = CallStateHangingUpBuilder;

impl CallStateHangingUpBuilder {
    pub fn build(&self) -> CallStateHangingUp {
        self.inner.clone()
    }
}

impl AsRef<CallStateHangingUp> for CallStateHangingUp {
    fn as_ref(&self) -> &CallStateHangingUp {
        self
    }
}

impl AsRef<CallStateHangingUp> for CallStateHangingUpBuilder {
    fn as_ref(&self) -> &CallStateHangingUp {
        &self.inner
    }
}

/// The call is pending, waiting to be accepted by a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStatePending {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the call has already been created by the server

    #[serde(default)]
    is_created: bool,
    /// True, if the call has already been received by the other party

    #[serde(default)]
    is_received: bool,
}

impl RObject for CallStatePending {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallState for CallStatePending {}

impl CallStatePending {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallStatePendingBuilder {
        let mut inner = CallStatePending::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallStatePendingBuilder { inner }
    }

    pub fn is_created(&self) -> bool {
        self.is_created
    }

    pub fn is_received(&self) -> bool {
        self.is_received
    }
}

#[doc(hidden)]
pub struct CallStatePendingBuilder {
    inner: CallStatePending,
}

#[deprecated]
pub type RTDCallStatePendingBuilder = CallStatePendingBuilder;

impl CallStatePendingBuilder {
    pub fn build(&self) -> CallStatePending {
        self.inner.clone()
    }

    pub fn is_created(&mut self, is_created: bool) -> &mut Self {
        self.inner.is_created = is_created;
        self
    }

    pub fn is_received(&mut self, is_received: bool) -> &mut Self {
        self.inner.is_received = is_received;
        self
    }
}

impl AsRef<CallStatePending> for CallStatePending {
    fn as_ref(&self) -> &CallStatePending {
        self
    }
}

impl AsRef<CallStatePending> for CallStatePendingBuilder {
    fn as_ref(&self) -> &CallStatePending {
        &self.inner
    }
}

/// The call is ready to use
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStateReady {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Call protocols supported by the peer
    protocol: CallProtocol,
    /// List of available call servers

    #[serde(default)]
    servers: Vec<CallServer>,
    /// A JSON-encoded call config

    #[serde(default)]
    config: String,
    /// Call encryption key

    #[serde(default)]
    encryption_key: String,
    /// Encryption key emojis fingerprint

    #[serde(default)]
    emojis: Vec<String>,
    /// True, if peer-to-peer connection is allowed by users privacy settings

    #[serde(default)]
    allow_p2p: bool,
}

impl RObject for CallStateReady {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallState for CallStateReady {}

impl CallStateReady {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallStateReadyBuilder {
        let mut inner = CallStateReady::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallStateReadyBuilder { inner }
    }

    pub fn protocol(&self) -> &CallProtocol {
        &self.protocol
    }

    pub fn servers(&self) -> &Vec<CallServer> {
        &self.servers
    }

    pub fn config(&self) -> &String {
        &self.config
    }

    pub fn encryption_key(&self) -> &String {
        &self.encryption_key
    }

    pub fn emojis(&self) -> &Vec<String> {
        &self.emojis
    }

    pub fn allow_p2p(&self) -> bool {
        self.allow_p2p
    }
}

#[doc(hidden)]
pub struct CallStateReadyBuilder {
    inner: CallStateReady,
}

#[deprecated]
pub type RTDCallStateReadyBuilder = CallStateReadyBuilder;

impl CallStateReadyBuilder {
    pub fn build(&self) -> CallStateReady {
        self.inner.clone()
    }

    pub fn protocol<T: AsRef<CallProtocol>>(&mut self, protocol: T) -> &mut Self {
        self.inner.protocol = protocol.as_ref().clone();
        self
    }

    pub fn servers(&mut self, servers: Vec<CallServer>) -> &mut Self {
        self.inner.servers = servers;
        self
    }

    pub fn config<T: AsRef<str>>(&mut self, config: T) -> &mut Self {
        self.inner.config = config.as_ref().to_string();
        self
    }

    pub fn encryption_key<T: AsRef<str>>(&mut self, encryption_key: T) -> &mut Self {
        self.inner.encryption_key = encryption_key.as_ref().to_string();
        self
    }

    pub fn emojis(&mut self, emojis: Vec<String>) -> &mut Self {
        self.inner.emojis = emojis;
        self
    }

    pub fn allow_p2p(&mut self, allow_p2p: bool) -> &mut Self {
        self.inner.allow_p2p = allow_p2p;
        self
    }
}

impl AsRef<CallStateReady> for CallStateReady {
    fn as_ref(&self) -> &CallStateReady {
        self
    }
}

impl AsRef<CallStateReady> for CallStateReadyBuilder {
    fn as_ref(&self) -> &CallStateReady {
        &self.inner
    }
}
