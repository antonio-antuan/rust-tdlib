use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the reason why a call was discarded
pub trait TDCallDiscardReason: Debug + RObject {}

/// Describes the reason why a call was discarded
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallDiscardReason {
    #[doc(hidden)]
    _Default,
    /// The call was ended before the conversation started. It was declined by the other party
    #[serde(rename = "callDiscardReasonDeclined")]
    Declined(CallDiscardReasonDeclined),
    /// The call was ended during the conversation because the users were disconnected
    #[serde(rename = "callDiscardReasonDisconnected")]
    Disconnected(CallDiscardReasonDisconnected),
    /// The call wasn't discarded, or the reason is unknown
    #[serde(rename = "callDiscardReasonEmpty")]
    Empty(CallDiscardReasonEmpty),
    /// The call was ended because one of the parties hung up
    #[serde(rename = "callDiscardReasonHungUp")]
    HungUp(CallDiscardReasonHungUp),
    /// The call was ended before the conversation started. It was canceled by the caller or missed by the other party
    #[serde(rename = "callDiscardReasonMissed")]
    Missed(CallDiscardReasonMissed),
}

impl Default for CallDiscardReason {
    fn default() -> Self {
        CallDiscardReason::_Default
    }
}

impl RObject for CallDiscardReason {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            CallDiscardReason::Declined(t) => t.extra(),
            CallDiscardReason::Disconnected(t) => t.extra(),
            CallDiscardReason::Empty(t) => t.extra(),
            CallDiscardReason::HungUp(t) => t.extra(),
            CallDiscardReason::Missed(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            CallDiscardReason::Declined(t) => t.client_id(),
            CallDiscardReason::Disconnected(t) => t.client_id(),
            CallDiscardReason::Empty(t) => t.client_id(),
            CallDiscardReason::HungUp(t) => t.client_id(),
            CallDiscardReason::Missed(t) => t.client_id(),

            _ => None,
        }
    }
}

impl CallDiscardReason {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, CallDiscardReason::_Default)
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallDiscardReasonDeclined {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallDiscardReason for CallDiscardReasonDeclined {}

impl CallDiscardReasonDeclined {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallDiscardReasonDeclinedBuilder {
        let mut inner = CallDiscardReasonDeclined::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallDiscardReasonDeclinedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallDiscardReasonDeclinedBuilder {
    inner: CallDiscardReasonDeclined,
}

#[deprecated]
pub type RTDCallDiscardReasonDeclinedBuilder = CallDiscardReasonDeclinedBuilder;

impl CallDiscardReasonDeclinedBuilder {
    pub fn build(&self) -> CallDiscardReasonDeclined {
        self.inner.clone()
    }
}

impl AsRef<CallDiscardReasonDeclined> for CallDiscardReasonDeclined {
    fn as_ref(&self) -> &CallDiscardReasonDeclined {
        self
    }
}

impl AsRef<CallDiscardReasonDeclined> for CallDiscardReasonDeclinedBuilder {
    fn as_ref(&self) -> &CallDiscardReasonDeclined {
        &self.inner
    }
}

/// The call was ended during the conversation because the users were disconnected
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallDiscardReasonDisconnected {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallDiscardReasonDisconnected {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallDiscardReason for CallDiscardReasonDisconnected {}

impl CallDiscardReasonDisconnected {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallDiscardReasonDisconnectedBuilder {
        let mut inner = CallDiscardReasonDisconnected::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallDiscardReasonDisconnectedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallDiscardReasonDisconnectedBuilder {
    inner: CallDiscardReasonDisconnected,
}

#[deprecated]
pub type RTDCallDiscardReasonDisconnectedBuilder = CallDiscardReasonDisconnectedBuilder;

impl CallDiscardReasonDisconnectedBuilder {
    pub fn build(&self) -> CallDiscardReasonDisconnected {
        self.inner.clone()
    }
}

impl AsRef<CallDiscardReasonDisconnected> for CallDiscardReasonDisconnected {
    fn as_ref(&self) -> &CallDiscardReasonDisconnected {
        self
    }
}

impl AsRef<CallDiscardReasonDisconnected> for CallDiscardReasonDisconnectedBuilder {
    fn as_ref(&self) -> &CallDiscardReasonDisconnected {
        &self.inner
    }
}

/// The call wasn't discarded, or the reason is unknown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallDiscardReasonEmpty {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallDiscardReasonEmpty {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallDiscardReason for CallDiscardReasonEmpty {}

impl CallDiscardReasonEmpty {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallDiscardReasonEmptyBuilder {
        let mut inner = CallDiscardReasonEmpty::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallDiscardReasonEmptyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallDiscardReasonEmptyBuilder {
    inner: CallDiscardReasonEmpty,
}

#[deprecated]
pub type RTDCallDiscardReasonEmptyBuilder = CallDiscardReasonEmptyBuilder;

impl CallDiscardReasonEmptyBuilder {
    pub fn build(&self) -> CallDiscardReasonEmpty {
        self.inner.clone()
    }
}

impl AsRef<CallDiscardReasonEmpty> for CallDiscardReasonEmpty {
    fn as_ref(&self) -> &CallDiscardReasonEmpty {
        self
    }
}

impl AsRef<CallDiscardReasonEmpty> for CallDiscardReasonEmptyBuilder {
    fn as_ref(&self) -> &CallDiscardReasonEmpty {
        &self.inner
    }
}

/// The call was ended because one of the parties hung up
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallDiscardReasonHungUp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallDiscardReasonHungUp {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallDiscardReason for CallDiscardReasonHungUp {}

impl CallDiscardReasonHungUp {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallDiscardReasonHungUpBuilder {
        let mut inner = CallDiscardReasonHungUp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallDiscardReasonHungUpBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallDiscardReasonHungUpBuilder {
    inner: CallDiscardReasonHungUp,
}

#[deprecated]
pub type RTDCallDiscardReasonHungUpBuilder = CallDiscardReasonHungUpBuilder;

impl CallDiscardReasonHungUpBuilder {
    pub fn build(&self) -> CallDiscardReasonHungUp {
        self.inner.clone()
    }
}

impl AsRef<CallDiscardReasonHungUp> for CallDiscardReasonHungUp {
    fn as_ref(&self) -> &CallDiscardReasonHungUp {
        self
    }
}

impl AsRef<CallDiscardReasonHungUp> for CallDiscardReasonHungUpBuilder {
    fn as_ref(&self) -> &CallDiscardReasonHungUp {
        &self.inner
    }
}

/// The call was ended before the conversation started. It was canceled by the caller or missed by the other party
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallDiscardReasonMissed {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallDiscardReasonMissed {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallDiscardReason for CallDiscardReasonMissed {}

impl CallDiscardReasonMissed {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallDiscardReasonMissedBuilder {
        let mut inner = CallDiscardReasonMissed::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallDiscardReasonMissedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallDiscardReasonMissedBuilder {
    inner: CallDiscardReasonMissed,
}

#[deprecated]
pub type RTDCallDiscardReasonMissedBuilder = CallDiscardReasonMissedBuilder;

impl CallDiscardReasonMissedBuilder {
    pub fn build(&self) -> CallDiscardReasonMissed {
        self.inner.clone()
    }
}

impl AsRef<CallDiscardReasonMissed> for CallDiscardReasonMissed {
    fn as_ref(&self) -> &CallDiscardReasonMissed {
        self
    }
}

impl AsRef<CallDiscardReasonMissed> for CallDiscardReasonMissedBuilder {
    fn as_ref(&self) -> &CallDiscardReasonMissed {
        &self.inner
    }
}
