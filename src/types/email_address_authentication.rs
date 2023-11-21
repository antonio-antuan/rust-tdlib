use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains authentication data for a email address
pub trait TDEmailAddressAuthentication: Debug + RObject {}

/// Contains authentication data for a email address
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum EmailAddressAuthentication {
    #[doc(hidden)]
    #[default]
    _Default,
    /// An authentication token received through Apple ID
    #[serde(rename = "emailAddressAuthenticationAppleId")]
    AppleId(EmailAddressAuthenticationAppleId),
    /// An authentication code delivered to a user's email address
    #[serde(rename = "emailAddressAuthenticationCode")]
    Code(EmailAddressAuthenticationCode),
    /// An authentication token received through Google ID
    #[serde(rename = "emailAddressAuthenticationGoogleId")]
    GoogleId(EmailAddressAuthenticationGoogleId),
}

impl RObject for EmailAddressAuthentication {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            EmailAddressAuthentication::AppleId(t) => t.extra(),
            EmailAddressAuthentication::Code(t) => t.extra(),
            EmailAddressAuthentication::GoogleId(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            EmailAddressAuthentication::AppleId(t) => t.client_id(),
            EmailAddressAuthentication::Code(t) => t.client_id(),
            EmailAddressAuthentication::GoogleId(t) => t.client_id(),

            _ => None,
        }
    }
}

impl EmailAddressAuthentication {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, EmailAddressAuthentication::_Default)
    }
}

impl AsRef<EmailAddressAuthentication> for EmailAddressAuthentication {
    fn as_ref(&self) -> &EmailAddressAuthentication {
        self
    }
}

/// An authentication token received through Apple ID
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailAddressAuthenticationAppleId {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The token

    #[serde(default)]
    token: String,
}

impl RObject for EmailAddressAuthenticationAppleId {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDEmailAddressAuthentication for EmailAddressAuthenticationAppleId {}

impl EmailAddressAuthenticationAppleId {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EmailAddressAuthenticationAppleIdBuilder {
        let mut inner = EmailAddressAuthenticationAppleId::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        EmailAddressAuthenticationAppleIdBuilder { inner }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[doc(hidden)]
pub struct EmailAddressAuthenticationAppleIdBuilder {
    inner: EmailAddressAuthenticationAppleId,
}

#[deprecated]
pub type RTDEmailAddressAuthenticationAppleIdBuilder = EmailAddressAuthenticationAppleIdBuilder;

impl EmailAddressAuthenticationAppleIdBuilder {
    pub fn build(&self) -> EmailAddressAuthenticationAppleId {
        self.inner.clone()
    }

    pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
        self.inner.token = token.as_ref().to_string();
        self
    }
}

impl AsRef<EmailAddressAuthenticationAppleId> for EmailAddressAuthenticationAppleId {
    fn as_ref(&self) -> &EmailAddressAuthenticationAppleId {
        self
    }
}

impl AsRef<EmailAddressAuthenticationAppleId> for EmailAddressAuthenticationAppleIdBuilder {
    fn as_ref(&self) -> &EmailAddressAuthenticationAppleId {
        &self.inner
    }
}

/// An authentication code delivered to a user's email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailAddressAuthenticationCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The code

    #[serde(default)]
    code: String,
}

impl RObject for EmailAddressAuthenticationCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDEmailAddressAuthentication for EmailAddressAuthenticationCode {}

impl EmailAddressAuthenticationCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EmailAddressAuthenticationCodeBuilder {
        let mut inner = EmailAddressAuthenticationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        EmailAddressAuthenticationCodeBuilder { inner }
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}

#[doc(hidden)]
pub struct EmailAddressAuthenticationCodeBuilder {
    inner: EmailAddressAuthenticationCode,
}

#[deprecated]
pub type RTDEmailAddressAuthenticationCodeBuilder = EmailAddressAuthenticationCodeBuilder;

impl EmailAddressAuthenticationCodeBuilder {
    pub fn build(&self) -> EmailAddressAuthenticationCode {
        self.inner.clone()
    }

    pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
        self.inner.code = code.as_ref().to_string();
        self
    }
}

impl AsRef<EmailAddressAuthenticationCode> for EmailAddressAuthenticationCode {
    fn as_ref(&self) -> &EmailAddressAuthenticationCode {
        self
    }
}

impl AsRef<EmailAddressAuthenticationCode> for EmailAddressAuthenticationCodeBuilder {
    fn as_ref(&self) -> &EmailAddressAuthenticationCode {
        &self.inner
    }
}

/// An authentication token received through Google ID
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailAddressAuthenticationGoogleId {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The token

    #[serde(default)]
    token: String,
}

impl RObject for EmailAddressAuthenticationGoogleId {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDEmailAddressAuthentication for EmailAddressAuthenticationGoogleId {}

impl EmailAddressAuthenticationGoogleId {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EmailAddressAuthenticationGoogleIdBuilder {
        let mut inner = EmailAddressAuthenticationGoogleId::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        EmailAddressAuthenticationGoogleIdBuilder { inner }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[doc(hidden)]
pub struct EmailAddressAuthenticationGoogleIdBuilder {
    inner: EmailAddressAuthenticationGoogleId,
}

#[deprecated]
pub type RTDEmailAddressAuthenticationGoogleIdBuilder = EmailAddressAuthenticationGoogleIdBuilder;

impl EmailAddressAuthenticationGoogleIdBuilder {
    pub fn build(&self) -> EmailAddressAuthenticationGoogleId {
        self.inner.clone()
    }

    pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
        self.inner.token = token.as_ref().to_string();
        self
    }
}

impl AsRef<EmailAddressAuthenticationGoogleId> for EmailAddressAuthenticationGoogleId {
    fn as_ref(&self) -> &EmailAddressAuthenticationGoogleId {
        self
    }
}

impl AsRef<EmailAddressAuthenticationGoogleId> for EmailAddressAuthenticationGoogleIdBuilder {
    fn as_ref(&self) -> &EmailAddressAuthenticationGoogleId {
        &self.inner
    }
}
