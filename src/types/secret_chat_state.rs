use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the current secret chat state
pub trait TDSecretChatState: Debug + RObject {}

/// Describes the current secret chat state
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SecretChatState {
    #[doc(hidden)]
    _Default(()),
    /// The secret chat is closed
    Closed(SecretChatStateClosed),
    /// The secret chat is not yet created; waiting for the other user to get online
    Pending(SecretChatStatePending),
    /// The secret chat is ready to use
    Ready(SecretChatStateReady),
}

impl Default for SecretChatState {
    fn default() -> Self {
        SecretChatState::_Default(())
    }
}

impl<'de> Deserialize<'de> for SecretChatState {
    fn deserialize<D>(deserializer: D) -> Result<SecretChatState, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          SecretChatState,
          (secretChatStateClosed, Closed);
          (secretChatStatePending, Pending);
          (secretChatStateReady, Ready);

        )(deserializer)
    }
}

impl RObject for SecretChatState {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            SecretChatState::Closed(t) => t.td_name(),
            SecretChatState::Pending(t) => t.td_name(),
            SecretChatState::Ready(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            SecretChatState::Closed(t) => t.extra(),
            SecretChatState::Pending(t) => t.extra(),
            SecretChatState::Ready(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl SecretChatState {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, SecretChatState::_Default(_))
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for SecretChatStateClosed {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "secretChatStateClosed"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDSecretChatState for SecretChatStateClosed {}

impl SecretChatStateClosed {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSecretChatStateClosedBuilder {
        let mut inner = SecretChatStateClosed::default();
        inner.td_name = "secretChatStateClosed".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDSecretChatStateClosedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSecretChatStateClosedBuilder {
    inner: SecretChatStateClosed,
}

impl RTDSecretChatStateClosedBuilder {
    pub fn build(&self) -> SecretChatStateClosed {
        self.inner.clone()
    }
}

impl AsRef<SecretChatStateClosed> for SecretChatStateClosed {
    fn as_ref(&self) -> &SecretChatStateClosed {
        self
    }
}

impl AsRef<SecretChatStateClosed> for RTDSecretChatStateClosedBuilder {
    fn as_ref(&self) -> &SecretChatStateClosed {
        &self.inner
    }
}

/// The secret chat is not yet created; waiting for the other user to get online
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecretChatStatePending {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for SecretChatStatePending {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "secretChatStatePending"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDSecretChatState for SecretChatStatePending {}

impl SecretChatStatePending {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSecretChatStatePendingBuilder {
        let mut inner = SecretChatStatePending::default();
        inner.td_name = "secretChatStatePending".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDSecretChatStatePendingBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSecretChatStatePendingBuilder {
    inner: SecretChatStatePending,
}

impl RTDSecretChatStatePendingBuilder {
    pub fn build(&self) -> SecretChatStatePending {
        self.inner.clone()
    }
}

impl AsRef<SecretChatStatePending> for SecretChatStatePending {
    fn as_ref(&self) -> &SecretChatStatePending {
        self
    }
}

impl AsRef<SecretChatStatePending> for RTDSecretChatStatePendingBuilder {
    fn as_ref(&self) -> &SecretChatStatePending {
        &self.inner
    }
}

/// The secret chat is ready to use
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecretChatStateReady {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for SecretChatStateReady {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "secretChatStateReady"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDSecretChatState for SecretChatStateReady {}

impl SecretChatStateReady {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSecretChatStateReadyBuilder {
        let mut inner = SecretChatStateReady::default();
        inner.td_name = "secretChatStateReady".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDSecretChatStateReadyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSecretChatStateReadyBuilder {
    inner: SecretChatStateReady,
}

impl RTDSecretChatStateReadyBuilder {
    pub fn build(&self) -> SecretChatStateReady {
        self.inner.clone()
    }
}

impl AsRef<SecretChatStateReady> for SecretChatStateReady {
    fn as_ref(&self) -> &SecretChatStateReady {
        self
    }
}

impl AsRef<SecretChatStateReady> for RTDSecretChatStateReadyBuilder {
    fn as_ref(&self) -> &SecretChatStateReady {
        &self.inner
    }
}
