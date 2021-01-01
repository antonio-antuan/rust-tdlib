use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the reason why a call was discarded
pub trait TDCallDiscardReason: Debug + RObject {}

/// Describes the reason why a call was discarded
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CallDiscardReason {
    #[doc(hidden)]
    _Default(()),
    /// The call was ended before the conversation started. It was declined by the other party
    Declined(CallDiscardReasonDeclined),
    /// The call was ended during the conversation because the users were disconnected
    Disconnected(CallDiscardReasonDisconnected),
    /// The call wasn't discarded, or the reason is unknown
    Empty(CallDiscardReasonEmpty),
    /// The call was ended because one of the parties hung up
    HungUp(CallDiscardReasonHungUp),
    /// The call was ended before the conversation started. It was cancelled by the caller or missed by the other party
    Missed(CallDiscardReasonMissed),
}

impl Default for CallDiscardReason {
    fn default() -> Self {
        CallDiscardReason::_Default(())
    }
}

impl<'de> Deserialize<'de> for CallDiscardReason {
    fn deserialize<D>(deserializer: D) -> Result<CallDiscardReason, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          CallDiscardReason,
          (callDiscardReasonDeclined, Declined);
          (callDiscardReasonDisconnected, Disconnected);
          (callDiscardReasonEmpty, Empty);
          (callDiscardReasonHungUp, HungUp);
          (callDiscardReasonMissed, Missed);

        )(deserializer)
    }
}

impl RObject for CallDiscardReason {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            CallDiscardReason::Declined(t) => t.td_name(),
            CallDiscardReason::Disconnected(t) => t.td_name(),
            CallDiscardReason::Empty(t) => t.td_name(),
            CallDiscardReason::HungUp(t) => t.td_name(),
            CallDiscardReason::Missed(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            CallDiscardReason::Declined(t) => t.extra(),
            CallDiscardReason::Disconnected(t) => t.extra(),
            CallDiscardReason::Empty(t) => t.extra(),
            CallDiscardReason::HungUp(t) => t.extra(),
            CallDiscardReason::Missed(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl CallDiscardReason {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, CallDiscardReason::_Default(_))
    }
}

impl AsRef<CallDiscardReason> for CallDiscardReason {
    fn as_ref(&self) -> &CallDiscardReason {
        self
    }
}

/// The call was ended before the conversation started. It was declined by the other party
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallDiscardReasonDeclined {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for CallDiscardReasonDeclined {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callDiscardReasonDeclined"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallDiscardReason for CallDiscardReasonDeclined {}

impl CallDiscardReasonDeclined {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallDiscardReasonDeclinedBuilder {
        let mut inner = CallDiscardReasonDeclined::default();
        inner.td_name = "callDiscardReasonDeclined".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallDiscardReasonDeclinedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallDiscardReasonDeclinedBuilder {
    inner: CallDiscardReasonDeclined,
}

impl RTDCallDiscardReasonDeclinedBuilder {
    pub fn build(&self) -> CallDiscardReasonDeclined {
        self.inner.clone()
    }
}

impl AsRef<CallDiscardReasonDeclined> for CallDiscardReasonDeclined {
    fn as_ref(&self) -> &CallDiscardReasonDeclined {
        self
    }
}

impl AsRef<CallDiscardReasonDeclined> for RTDCallDiscardReasonDeclinedBuilder {
    fn as_ref(&self) -> &CallDiscardReasonDeclined {
        &self.inner
    }
}

/// The call was ended during the conversation because the users were disconnected
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallDiscardReasonDisconnected {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for CallDiscardReasonDisconnected {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callDiscardReasonDisconnected"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallDiscardReason for CallDiscardReasonDisconnected {}

impl CallDiscardReasonDisconnected {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallDiscardReasonDisconnectedBuilder {
        let mut inner = CallDiscardReasonDisconnected::default();
        inner.td_name = "callDiscardReasonDisconnected".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallDiscardReasonDisconnectedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallDiscardReasonDisconnectedBuilder {
    inner: CallDiscardReasonDisconnected,
}

impl RTDCallDiscardReasonDisconnectedBuilder {
    pub fn build(&self) -> CallDiscardReasonDisconnected {
        self.inner.clone()
    }
}

impl AsRef<CallDiscardReasonDisconnected> for CallDiscardReasonDisconnected {
    fn as_ref(&self) -> &CallDiscardReasonDisconnected {
        self
    }
}

impl AsRef<CallDiscardReasonDisconnected> for RTDCallDiscardReasonDisconnectedBuilder {
    fn as_ref(&self) -> &CallDiscardReasonDisconnected {
        &self.inner
    }
}

/// The call wasn't discarded, or the reason is unknown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallDiscardReasonEmpty {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for CallDiscardReasonEmpty {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callDiscardReasonEmpty"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallDiscardReason for CallDiscardReasonEmpty {}

impl CallDiscardReasonEmpty {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallDiscardReasonEmptyBuilder {
        let mut inner = CallDiscardReasonEmpty::default();
        inner.td_name = "callDiscardReasonEmpty".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallDiscardReasonEmptyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallDiscardReasonEmptyBuilder {
    inner: CallDiscardReasonEmpty,
}

impl RTDCallDiscardReasonEmptyBuilder {
    pub fn build(&self) -> CallDiscardReasonEmpty {
        self.inner.clone()
    }
}

impl AsRef<CallDiscardReasonEmpty> for CallDiscardReasonEmpty {
    fn as_ref(&self) -> &CallDiscardReasonEmpty {
        self
    }
}

impl AsRef<CallDiscardReasonEmpty> for RTDCallDiscardReasonEmptyBuilder {
    fn as_ref(&self) -> &CallDiscardReasonEmpty {
        &self.inner
    }
}

/// The call was ended because one of the parties hung up
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallDiscardReasonHungUp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for CallDiscardReasonHungUp {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callDiscardReasonHungUp"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallDiscardReason for CallDiscardReasonHungUp {}

impl CallDiscardReasonHungUp {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallDiscardReasonHungUpBuilder {
        let mut inner = CallDiscardReasonHungUp::default();
        inner.td_name = "callDiscardReasonHungUp".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallDiscardReasonHungUpBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallDiscardReasonHungUpBuilder {
    inner: CallDiscardReasonHungUp,
}

impl RTDCallDiscardReasonHungUpBuilder {
    pub fn build(&self) -> CallDiscardReasonHungUp {
        self.inner.clone()
    }
}

impl AsRef<CallDiscardReasonHungUp> for CallDiscardReasonHungUp {
    fn as_ref(&self) -> &CallDiscardReasonHungUp {
        self
    }
}

impl AsRef<CallDiscardReasonHungUp> for RTDCallDiscardReasonHungUpBuilder {
    fn as_ref(&self) -> &CallDiscardReasonHungUp {
        &self.inner
    }
}

/// The call was ended before the conversation started. It was cancelled by the caller or missed by the other party
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallDiscardReasonMissed {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for CallDiscardReasonMissed {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callDiscardReasonMissed"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallDiscardReason for CallDiscardReasonMissed {}

impl CallDiscardReasonMissed {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallDiscardReasonMissedBuilder {
        let mut inner = CallDiscardReasonMissed::default();
        inner.td_name = "callDiscardReasonMissed".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallDiscardReasonMissedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallDiscardReasonMissedBuilder {
    inner: CallDiscardReasonMissed,
}

impl RTDCallDiscardReasonMissedBuilder {
    pub fn build(&self) -> CallDiscardReasonMissed {
        self.inner.clone()
    }
}

impl AsRef<CallDiscardReasonMissed> for CallDiscardReasonMissed {
    fn as_ref(&self) -> &CallDiscardReasonMissed {
        self
    }
}

impl AsRef<CallDiscardReasonMissed> for RTDCallDiscardReasonMissedBuilder {
    fn as_ref(&self) -> &CallDiscardReasonMissed {
        &self.inner
    }
}
