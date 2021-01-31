use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the current call state
pub trait TDCallState: Debug + RObject {}

/// Describes the current call state
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CallState {
    #[doc(hidden)]
    _Default(()),
    /// The call has ended successfully
    Discarded(CallStateDiscarded),
    /// The call has ended with an error
    Error(CallStateError),
    /// The call has been answered and encryption keys are being exchanged
    ExchangingKeys(CallStateExchangingKeys),
    /// The call is hanging up after discardCall has been called
    HangingUp(CallStateHangingUp),
    /// The call is pending, waiting to be accepted by a user
    Pending(CallStatePending),
    /// The call is ready to use
    Ready(CallStateReady),
}

impl Default for CallState {
    fn default() -> Self {
        CallState::_Default(())
    }
}

impl<'de> Deserialize<'de> for CallState {
    fn deserialize<D>(deserializer: D) -> Result<CallState, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          CallState,
          (callStateDiscarded, Discarded);
          (callStateError, Error);
          (callStateExchangingKeys, ExchangingKeys);
          (callStateHangingUp, HangingUp);
          (callStatePending, Pending);
          (callStateReady, Ready);

        )(deserializer)
    }
}

impl RObject for CallState {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            CallState::Discarded(t) => t.td_name(),
            CallState::Error(t) => t.td_name(),
            CallState::ExchangingKeys(t) => t.td_name(),
            CallState::HangingUp(t) => t.td_name(),
            CallState::Pending(t) => t.td_name(),
            CallState::Ready(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
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
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl CallState {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, CallState::_Default(_))
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// The reason, why the call has ended
    reason: CallDiscardReason,
    /// True, if the call rating should be sent to the server
    need_rating: bool,
    /// True, if the call debug information should be sent to the server
    need_debug_information: bool,
}

impl RObject for CallStateDiscarded {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callStateDiscarded"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallState for CallStateDiscarded {}

impl CallStateDiscarded {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallStateDiscardedBuilder {
        let mut inner = CallStateDiscarded::default();
        inner.td_name = "callStateDiscarded".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallStateDiscardedBuilder { inner }
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
pub struct RTDCallStateDiscardedBuilder {
    inner: CallStateDiscarded,
}

impl RTDCallStateDiscardedBuilder {
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

impl AsRef<CallStateDiscarded> for RTDCallStateDiscardedBuilder {
    fn as_ref(&self) -> &CallStateDiscarded {
        &self.inner
    }
}

/// The call has ended with an error
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStateError {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Error. An error with the code 4005000 will be returned if an outgoing call is missed because of an expired timeout
    error: Error,
}

impl RObject for CallStateError {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callStateError"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallState for CallStateError {}

impl CallStateError {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallStateErrorBuilder {
        let mut inner = CallStateError::default();
        inner.td_name = "callStateError".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallStateErrorBuilder { inner }
    }

    pub fn error(&self) -> &Error {
        &self.error
    }
}

#[doc(hidden)]
pub struct RTDCallStateErrorBuilder {
    inner: CallStateError,
}

impl RTDCallStateErrorBuilder {
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

impl AsRef<CallStateError> for RTDCallStateErrorBuilder {
    fn as_ref(&self) -> &CallStateError {
        &self.inner
    }
}

/// The call has been answered and encryption keys are being exchanged
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStateExchangingKeys {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for CallStateExchangingKeys {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callStateExchangingKeys"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallState for CallStateExchangingKeys {}

impl CallStateExchangingKeys {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallStateExchangingKeysBuilder {
        let mut inner = CallStateExchangingKeys::default();
        inner.td_name = "callStateExchangingKeys".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallStateExchangingKeysBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallStateExchangingKeysBuilder {
    inner: CallStateExchangingKeys,
}

impl RTDCallStateExchangingKeysBuilder {
    pub fn build(&self) -> CallStateExchangingKeys {
        self.inner.clone()
    }
}

impl AsRef<CallStateExchangingKeys> for CallStateExchangingKeys {
    fn as_ref(&self) -> &CallStateExchangingKeys {
        self
    }
}

impl AsRef<CallStateExchangingKeys> for RTDCallStateExchangingKeysBuilder {
    fn as_ref(&self) -> &CallStateExchangingKeys {
        &self.inner
    }
}

/// The call is hanging up after discardCall has been called
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStateHangingUp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for CallStateHangingUp {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callStateHangingUp"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallState for CallStateHangingUp {}

impl CallStateHangingUp {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallStateHangingUpBuilder {
        let mut inner = CallStateHangingUp::default();
        inner.td_name = "callStateHangingUp".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallStateHangingUpBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallStateHangingUpBuilder {
    inner: CallStateHangingUp,
}

impl RTDCallStateHangingUpBuilder {
    pub fn build(&self) -> CallStateHangingUp {
        self.inner.clone()
    }
}

impl AsRef<CallStateHangingUp> for CallStateHangingUp {
    fn as_ref(&self) -> &CallStateHangingUp {
        self
    }
}

impl AsRef<CallStateHangingUp> for RTDCallStateHangingUpBuilder {
    fn as_ref(&self) -> &CallStateHangingUp {
        &self.inner
    }
}

/// The call is pending, waiting to be accepted by a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStatePending {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// True, if the call has already been created by the server
    is_created: bool,
    /// True, if the call has already been received by the other party
    is_received: bool,
}

impl RObject for CallStatePending {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callStatePending"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallState for CallStatePending {}

impl CallStatePending {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallStatePendingBuilder {
        let mut inner = CallStatePending::default();
        inner.td_name = "callStatePending".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallStatePendingBuilder { inner }
    }

    pub fn is_created(&self) -> bool {
        self.is_created
    }

    pub fn is_received(&self) -> bool {
        self.is_received
    }
}

#[doc(hidden)]
pub struct RTDCallStatePendingBuilder {
    inner: CallStatePending,
}

impl RTDCallStatePendingBuilder {
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

impl AsRef<CallStatePending> for RTDCallStatePendingBuilder {
    fn as_ref(&self) -> &CallStatePending {
        &self.inner
    }
}

/// The call is ready to use
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStateReady {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Call protocols supported by the peer
    protocol: CallProtocol,
    /// List of available call servers
    servers: Vec<CallServer>,
    /// A JSON-encoded call config
    config: String,
    /// Call encryption key
    encryption_key: String,
    /// Encryption key emojis fingerprint
    emojis: Vec<String>,
    /// True, if peer-to-peer connection is allowed by users privacy settings
    allow_p2p: bool,
}

impl RObject for CallStateReady {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callStateReady"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallState for CallStateReady {}

impl CallStateReady {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallStateReadyBuilder {
        let mut inner = CallStateReady::default();
        inner.td_name = "callStateReady".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallStateReadyBuilder { inner }
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
pub struct RTDCallStateReadyBuilder {
    inner: CallStateReady,
}

impl RTDCallStateReadyBuilder {
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

impl AsRef<CallStateReady> for RTDCallStateReadyBuilder {
    fn as_ref(&self) -> &CallStateReady {
        &self.inner
    }
}
