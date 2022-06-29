use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents result of 2-step verification password reset
pub trait TDResetPasswordResult: Debug + RObject {}

/// Represents result of 2-step verification password reset
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ResetPasswordResult {
    #[doc(hidden)]
    _Default,
    /// Removes 2-step verification password without previous password and access to recovery email address. The password can't be reset immediately and the request needs to be repeated after the specified time
    #[serde(rename(deserialize = "resetPassword"))]
    ResetPassword(ResetPassword),
    /// The password reset request was declined
    #[serde(rename(deserialize = "resetPasswordResultDeclined"))]
    Declined(ResetPasswordResultDeclined),
    /// The password was reset
    #[serde(rename(deserialize = "resetPasswordResultOk"))]
    Ok(ResetPasswordResultOk),
    /// The password reset request is pending
    #[serde(rename(deserialize = "resetPasswordResultPending"))]
    Pending(ResetPasswordResultPending),
}

impl Default for ResetPasswordResult {
    fn default() -> Self {
        ResetPasswordResult::_Default
    }
}

impl RObject for ResetPasswordResult {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ResetPasswordResult::ResetPassword(t) => t.extra(),
            ResetPasswordResult::Declined(t) => t.extra(),
            ResetPasswordResult::Ok(t) => t.extra(),
            ResetPasswordResult::Pending(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ResetPasswordResult::ResetPassword(t) => t.client_id(),
            ResetPasswordResult::Declined(t) => t.client_id(),
            ResetPasswordResult::Ok(t) => t.client_id(),
            ResetPasswordResult::Pending(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ResetPasswordResult {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ResetPasswordResult::_Default)
    }
}

impl AsRef<ResetPasswordResult> for ResetPasswordResult {
    fn as_ref(&self) -> &ResetPasswordResult {
        self
    }
}

/// The password reset request was declined
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetPasswordResultDeclined {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Point in time (Unix timestamp) when the password reset can be retried

    #[serde(default)]
    retry_date: i32,
}

impl RObject for ResetPasswordResultDeclined {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDResetPasswordResult for ResetPasswordResultDeclined {}

impl ResetPasswordResultDeclined {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDResetPasswordResultDeclinedBuilder {
        let mut inner = ResetPasswordResultDeclined::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDResetPasswordResultDeclinedBuilder { inner }
    }

    pub fn retry_date(&self) -> i32 {
        self.retry_date
    }
}

#[doc(hidden)]
pub struct RTDResetPasswordResultDeclinedBuilder {
    inner: ResetPasswordResultDeclined,
}

impl RTDResetPasswordResultDeclinedBuilder {
    pub fn build(&self) -> ResetPasswordResultDeclined {
        self.inner.clone()
    }

    pub fn retry_date(&mut self, retry_date: i32) -> &mut Self {
        self.inner.retry_date = retry_date;
        self
    }
}

impl AsRef<ResetPasswordResultDeclined> for ResetPasswordResultDeclined {
    fn as_ref(&self) -> &ResetPasswordResultDeclined {
        self
    }
}

impl AsRef<ResetPasswordResultDeclined> for RTDResetPasswordResultDeclinedBuilder {
    fn as_ref(&self) -> &ResetPasswordResultDeclined {
        &self.inner
    }
}

/// The password was reset
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetPasswordResultOk {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ResetPasswordResultOk {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDResetPasswordResult for ResetPasswordResultOk {}

impl ResetPasswordResultOk {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDResetPasswordResultOkBuilder {
        let mut inner = ResetPasswordResultOk::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDResetPasswordResultOkBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDResetPasswordResultOkBuilder {
    inner: ResetPasswordResultOk,
}

impl RTDResetPasswordResultOkBuilder {
    pub fn build(&self) -> ResetPasswordResultOk {
        self.inner.clone()
    }
}

impl AsRef<ResetPasswordResultOk> for ResetPasswordResultOk {
    fn as_ref(&self) -> &ResetPasswordResultOk {
        self
    }
}

impl AsRef<ResetPasswordResultOk> for RTDResetPasswordResultOkBuilder {
    fn as_ref(&self) -> &ResetPasswordResultOk {
        &self.inner
    }
}

/// The password reset request is pending
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetPasswordResultPending {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Point in time (Unix timestamp) after which the password can be reset immediately using resetPassword

    #[serde(default)]
    pending_reset_date: i32,
}

impl RObject for ResetPasswordResultPending {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDResetPasswordResult for ResetPasswordResultPending {}

impl ResetPasswordResultPending {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDResetPasswordResultPendingBuilder {
        let mut inner = ResetPasswordResultPending::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDResetPasswordResultPendingBuilder { inner }
    }

    pub fn pending_reset_date(&self) -> i32 {
        self.pending_reset_date
    }
}

#[doc(hidden)]
pub struct RTDResetPasswordResultPendingBuilder {
    inner: ResetPasswordResultPending,
}

impl RTDResetPasswordResultPendingBuilder {
    pub fn build(&self) -> ResetPasswordResultPending {
        self.inner.clone()
    }

    pub fn pending_reset_date(&mut self, pending_reset_date: i32) -> &mut Self {
        self.inner.pending_reset_date = pending_reset_date;
        self
    }
}

impl AsRef<ResetPasswordResultPending> for ResetPasswordResultPending {
    fn as_ref(&self) -> &ResetPasswordResultPending {
        self
    }
}

impl AsRef<ResetPasswordResultPending> for RTDResetPasswordResultPendingBuilder {
    fn as_ref(&self) -> &ResetPasswordResultPending {
        &self.inner
    }
}
