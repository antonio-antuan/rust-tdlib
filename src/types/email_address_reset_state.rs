use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes reset state of a email address
pub trait TDEmailAddressResetState: Debug + RObject {}

/// Describes reset state of a email address
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum EmailAddressResetState {
    #[doc(hidden)]
    #[default]
    _Default,
    /// Email address can be reset after the given period. Call resetAuthenticationEmailAddress to reset it and allow the user to authorize with a code sent to the user's phone number
    #[serde(rename = "emailAddressResetStateAvailable")]
    Available(EmailAddressResetStateAvailable),
    /// Email address reset has already been requested. Call resetAuthenticationEmailAddress to check whether immediate reset is possible
    #[serde(rename = "emailAddressResetStatePending")]
    Pending(EmailAddressResetStatePending),
}

impl RObject for EmailAddressResetState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            EmailAddressResetState::Available(t) => t.extra(),
            EmailAddressResetState::Pending(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            EmailAddressResetState::Available(t) => t.client_id(),
            EmailAddressResetState::Pending(t) => t.client_id(),

            _ => None,
        }
    }
}

impl EmailAddressResetState {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, EmailAddressResetState::_Default)
    }
}

impl AsRef<EmailAddressResetState> for EmailAddressResetState {
    fn as_ref(&self) -> &EmailAddressResetState {
        self
    }
}

/// Email address can be reset after the given period. Call resetAuthenticationEmailAddress to reset it and allow the user to authorize with a code sent to the user's phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailAddressResetStateAvailable {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Time required to wait before the email address can be reset; 0 if the user is subscribed to Telegram Premium

    #[serde(default)]
    wait_period: i32,
}

impl RObject for EmailAddressResetStateAvailable {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDEmailAddressResetState for EmailAddressResetStateAvailable {}

impl EmailAddressResetStateAvailable {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EmailAddressResetStateAvailableBuilder {
        let mut inner = EmailAddressResetStateAvailable::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        EmailAddressResetStateAvailableBuilder { inner }
    }

    pub fn wait_period(&self) -> i32 {
        self.wait_period
    }
}

#[doc(hidden)]
pub struct EmailAddressResetStateAvailableBuilder {
    inner: EmailAddressResetStateAvailable,
}

#[deprecated]
pub type RTDEmailAddressResetStateAvailableBuilder = EmailAddressResetStateAvailableBuilder;

impl EmailAddressResetStateAvailableBuilder {
    pub fn build(&self) -> EmailAddressResetStateAvailable {
        self.inner.clone()
    }

    pub fn wait_period(&mut self, wait_period: i32) -> &mut Self {
        self.inner.wait_period = wait_period;
        self
    }
}

impl AsRef<EmailAddressResetStateAvailable> for EmailAddressResetStateAvailable {
    fn as_ref(&self) -> &EmailAddressResetStateAvailable {
        self
    }
}

impl AsRef<EmailAddressResetStateAvailable> for EmailAddressResetStateAvailableBuilder {
    fn as_ref(&self) -> &EmailAddressResetStateAvailable {
        &self.inner
    }
}

/// Email address reset has already been requested. Call resetAuthenticationEmailAddress to check whether immediate reset is possible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailAddressResetStatePending {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Left time before the email address will be reset, in seconds. updateAuthorizationState is not sent when this field changes

    #[serde(default)]
    reset_in: i32,
}

impl RObject for EmailAddressResetStatePending {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDEmailAddressResetState for EmailAddressResetStatePending {}

impl EmailAddressResetStatePending {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EmailAddressResetStatePendingBuilder {
        let mut inner = EmailAddressResetStatePending::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        EmailAddressResetStatePendingBuilder { inner }
    }

    pub fn reset_in(&self) -> i32 {
        self.reset_in
    }
}

#[doc(hidden)]
pub struct EmailAddressResetStatePendingBuilder {
    inner: EmailAddressResetStatePending,
}

#[deprecated]
pub type RTDEmailAddressResetStatePendingBuilder = EmailAddressResetStatePendingBuilder;

impl EmailAddressResetStatePendingBuilder {
    pub fn build(&self) -> EmailAddressResetStatePending {
        self.inner.clone()
    }

    pub fn reset_in(&mut self, reset_in: i32) -> &mut Self {
        self.inner.reset_in = reset_in;
        self
    }
}

impl AsRef<EmailAddressResetStatePending> for EmailAddressResetStatePending {
    fn as_ref(&self) -> &EmailAddressResetStatePending {
        self
    }
}

impl AsRef<EmailAddressResetStatePending> for EmailAddressResetStatePendingBuilder {
    fn as_ref(&self) -> &EmailAddressResetStatePending {
        &self.inner
    }
}
