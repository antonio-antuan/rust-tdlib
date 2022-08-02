use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the current secret chat state
pub trait TDSecretChatState: Debug + RObject {}

/// Describes the current secret chat state
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SecretChatState {
    #[doc(hidden)]
    _Default,
    /// The secret chat is closed
    #[serde(rename = "secretChatStateClosed")]
    Closed(SecretChatStateClosed),
    /// The secret chat is not yet created; waiting for the other user to get online
    #[serde(rename = "secretChatStatePending")]
    Pending(SecretChatStatePending),
    /// The secret chat is ready to use
    #[serde(rename = "secretChatStateReady")]
    Ready(SecretChatStateReady),
}

impl Default for SecretChatState {
    fn default() -> Self {
        SecretChatState::_Default
    }
}

impl RObject for SecretChatState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            SecretChatState::Closed(t) => t.extra(),
            SecretChatState::Pending(t) => t.extra(),
            SecretChatState::Ready(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            SecretChatState::Closed(t) => t.client_id(),
            SecretChatState::Pending(t) => t.client_id(),
            SecretChatState::Ready(t) => t.client_id(),

            _ => None,
        }
    }
}

impl SecretChatState {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, SecretChatState::_Default)
    }
}

impl AsRef<SecretChatState> for SecretChatState {
    fn as_ref(&self) -> &SecretChatState {
        self
    }
}

/// The secret chat is closed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecretChatStateClosed {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SecretChatStateClosed {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSecretChatState for SecretChatStateClosed {}

impl SecretChatStateClosed {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SecretChatStateClosedBuilder {
        let mut inner = SecretChatStateClosed::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SecretChatStateClosedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SecretChatStateClosedBuilder {
    inner: SecretChatStateClosed,
}

#[deprecated]
pub type RTDSecretChatStateClosedBuilder = SecretChatStateClosedBuilder;

impl SecretChatStateClosedBuilder {
    pub fn build(&self) -> SecretChatStateClosed {
        self.inner.clone()
    }
}

impl AsRef<SecretChatStateClosed> for SecretChatStateClosed {
    fn as_ref(&self) -> &SecretChatStateClosed {
        self
    }
}

impl AsRef<SecretChatStateClosed> for SecretChatStateClosedBuilder {
    fn as_ref(&self) -> &SecretChatStateClosed {
        &self.inner
    }
}

/// The secret chat is not yet created; waiting for the other user to get online
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecretChatStatePending {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SecretChatStatePending {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSecretChatState for SecretChatStatePending {}

impl SecretChatStatePending {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SecretChatStatePendingBuilder {
        let mut inner = SecretChatStatePending::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SecretChatStatePendingBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SecretChatStatePendingBuilder {
    inner: SecretChatStatePending,
}

#[deprecated]
pub type RTDSecretChatStatePendingBuilder = SecretChatStatePendingBuilder;

impl SecretChatStatePendingBuilder {
    pub fn build(&self) -> SecretChatStatePending {
        self.inner.clone()
    }
}

impl AsRef<SecretChatStatePending> for SecretChatStatePending {
    fn as_ref(&self) -> &SecretChatStatePending {
        self
    }
}

impl AsRef<SecretChatStatePending> for SecretChatStatePendingBuilder {
    fn as_ref(&self) -> &SecretChatStatePending {
        &self.inner
    }
}

/// The secret chat is ready to use
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecretChatStateReady {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SecretChatStateReady {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSecretChatState for SecretChatStateReady {}

impl SecretChatStateReady {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SecretChatStateReadyBuilder {
        let mut inner = SecretChatStateReady::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SecretChatStateReadyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SecretChatStateReadyBuilder {
    inner: SecretChatStateReady,
}

#[deprecated]
pub type RTDSecretChatStateReadyBuilder = SecretChatStateReadyBuilder;

impl SecretChatStateReadyBuilder {
    pub fn build(&self) -> SecretChatStateReady {
        self.inner.clone()
    }
}

impl AsRef<SecretChatStateReady> for SecretChatStateReady {
    fn as_ref(&self) -> &SecretChatStateReady {
        self
    }
}

impl AsRef<SecretChatStateReady> for SecretChatStateReadyBuilder {
    fn as_ref(&self) -> &SecretChatStateReady {
        &self.inner
    }
}
