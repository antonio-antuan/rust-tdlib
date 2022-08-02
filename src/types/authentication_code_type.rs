use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Provides information about the method by which an authentication code is delivered to the user
pub trait TDAuthenticationCodeType: Debug + RObject {}

/// Provides information about the method by which an authentication code is delivered to the user
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AuthenticationCodeType {
    #[doc(hidden)]
    _Default,
    /// An authentication code is delivered via a phone call to the specified phone number
    #[serde(rename = "authenticationCodeTypeCall")]
    Call(AuthenticationCodeTypeCall),
    /// An authentication code is delivered by an immediately canceled call to the specified phone number. The phone number that calls is the code that must be entered automatically
    #[serde(rename = "authenticationCodeTypeFlashCall")]
    FlashCall(AuthenticationCodeTypeFlashCall),
    /// An authentication code is delivered by an immediately canceled call to the specified phone number. The last digits of the phone number that calls are the code that must be entered manually by the user
    #[serde(rename = "authenticationCodeTypeMissedCall")]
    MissedCall(AuthenticationCodeTypeMissedCall),
    /// An authentication code is delivered via an SMS message to the specified phone number
    #[serde(rename = "authenticationCodeTypeSms")]
    Sms(AuthenticationCodeTypeSms),
    /// An authentication code is delivered via a private Telegram message, which can be viewed from another active session
    #[serde(rename = "authenticationCodeTypeTelegramMessage")]
    TelegramMessage(AuthenticationCodeTypeTelegramMessage),
}

impl Default for AuthenticationCodeType {
    fn default() -> Self {
        AuthenticationCodeType::_Default
    }
}

impl RObject for AuthenticationCodeType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            AuthenticationCodeType::Call(t) => t.extra(),
            AuthenticationCodeType::FlashCall(t) => t.extra(),
            AuthenticationCodeType::MissedCall(t) => t.extra(),
            AuthenticationCodeType::Sms(t) => t.extra(),
            AuthenticationCodeType::TelegramMessage(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            AuthenticationCodeType::Call(t) => t.client_id(),
            AuthenticationCodeType::FlashCall(t) => t.client_id(),
            AuthenticationCodeType::MissedCall(t) => t.client_id(),
            AuthenticationCodeType::Sms(t) => t.client_id(),
            AuthenticationCodeType::TelegramMessage(t) => t.client_id(),

            _ => None,
        }
    }
}

impl AuthenticationCodeType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, AuthenticationCodeType::_Default)
    }
}

impl AsRef<AuthenticationCodeType> for AuthenticationCodeType {
    fn as_ref(&self) -> &AuthenticationCodeType {
        self
    }
}

/// An authentication code is delivered via a phone call to the specified phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Length of the code

    #[serde(default)]
    length: i32,
}

impl RObject for AuthenticationCodeTypeCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeCall {}

impl AuthenticationCodeTypeCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthenticationCodeTypeCallBuilder {
        let mut inner = AuthenticationCodeTypeCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthenticationCodeTypeCallBuilder { inner }
    }

    pub fn length(&self) -> i32 {
        self.length
    }
}

#[doc(hidden)]
pub struct AuthenticationCodeTypeCallBuilder {
    inner: AuthenticationCodeTypeCall,
}

#[deprecated]
pub type RTDAuthenticationCodeTypeCallBuilder = AuthenticationCodeTypeCallBuilder;

impl AuthenticationCodeTypeCallBuilder {
    pub fn build(&self) -> AuthenticationCodeTypeCall {
        self.inner.clone()
    }

    pub fn length(&mut self, length: i32) -> &mut Self {
        self.inner.length = length;
        self
    }
}

impl AsRef<AuthenticationCodeTypeCall> for AuthenticationCodeTypeCall {
    fn as_ref(&self) -> &AuthenticationCodeTypeCall {
        self
    }
}

impl AsRef<AuthenticationCodeTypeCall> for AuthenticationCodeTypeCallBuilder {
    fn as_ref(&self) -> &AuthenticationCodeTypeCall {
        &self.inner
    }
}

/// An authentication code is delivered by an immediately canceled call to the specified phone number. The phone number that calls is the code that must be entered automatically
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeFlashCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pattern of the phone number from which the call will be made

    #[serde(default)]
    pattern: String,
}

impl RObject for AuthenticationCodeTypeFlashCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeFlashCall {}

impl AuthenticationCodeTypeFlashCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthenticationCodeTypeFlashCallBuilder {
        let mut inner = AuthenticationCodeTypeFlashCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthenticationCodeTypeFlashCallBuilder { inner }
    }

    pub fn pattern(&self) -> &String {
        &self.pattern
    }
}

#[doc(hidden)]
pub struct AuthenticationCodeTypeFlashCallBuilder {
    inner: AuthenticationCodeTypeFlashCall,
}

#[deprecated]
pub type RTDAuthenticationCodeTypeFlashCallBuilder = AuthenticationCodeTypeFlashCallBuilder;

impl AuthenticationCodeTypeFlashCallBuilder {
    pub fn build(&self) -> AuthenticationCodeTypeFlashCall {
        self.inner.clone()
    }

    pub fn pattern<T: AsRef<str>>(&mut self, pattern: T) -> &mut Self {
        self.inner.pattern = pattern.as_ref().to_string();
        self
    }
}

impl AsRef<AuthenticationCodeTypeFlashCall> for AuthenticationCodeTypeFlashCall {
    fn as_ref(&self) -> &AuthenticationCodeTypeFlashCall {
        self
    }
}

impl AsRef<AuthenticationCodeTypeFlashCall> for AuthenticationCodeTypeFlashCallBuilder {
    fn as_ref(&self) -> &AuthenticationCodeTypeFlashCall {
        &self.inner
    }
}

/// An authentication code is delivered by an immediately canceled call to the specified phone number. The last digits of the phone number that calls are the code that must be entered manually by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeMissedCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Prefix of the phone number from which the call will be made

    #[serde(default)]
    phone_number_prefix: String,
    /// Number of digits in the code, excluding the prefix

    #[serde(default)]
    length: i32,
}

impl RObject for AuthenticationCodeTypeMissedCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeMissedCall {}

impl AuthenticationCodeTypeMissedCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthenticationCodeTypeMissedCallBuilder {
        let mut inner = AuthenticationCodeTypeMissedCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthenticationCodeTypeMissedCallBuilder { inner }
    }

    pub fn phone_number_prefix(&self) -> &String {
        &self.phone_number_prefix
    }

    pub fn length(&self) -> i32 {
        self.length
    }
}

#[doc(hidden)]
pub struct AuthenticationCodeTypeMissedCallBuilder {
    inner: AuthenticationCodeTypeMissedCall,
}

#[deprecated]
pub type RTDAuthenticationCodeTypeMissedCallBuilder = AuthenticationCodeTypeMissedCallBuilder;

impl AuthenticationCodeTypeMissedCallBuilder {
    pub fn build(&self) -> AuthenticationCodeTypeMissedCall {
        self.inner.clone()
    }

    pub fn phone_number_prefix<T: AsRef<str>>(&mut self, phone_number_prefix: T) -> &mut Self {
        self.inner.phone_number_prefix = phone_number_prefix.as_ref().to_string();
        self
    }

    pub fn length(&mut self, length: i32) -> &mut Self {
        self.inner.length = length;
        self
    }
}

impl AsRef<AuthenticationCodeTypeMissedCall> for AuthenticationCodeTypeMissedCall {
    fn as_ref(&self) -> &AuthenticationCodeTypeMissedCall {
        self
    }
}

impl AsRef<AuthenticationCodeTypeMissedCall> for AuthenticationCodeTypeMissedCallBuilder {
    fn as_ref(&self) -> &AuthenticationCodeTypeMissedCall {
        &self.inner
    }
}

/// An authentication code is delivered via an SMS message to the specified phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeSms {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Length of the code

    #[serde(default)]
    length: i32,
}

impl RObject for AuthenticationCodeTypeSms {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeSms {}

impl AuthenticationCodeTypeSms {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthenticationCodeTypeSmsBuilder {
        let mut inner = AuthenticationCodeTypeSms::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthenticationCodeTypeSmsBuilder { inner }
    }

    pub fn length(&self) -> i32 {
        self.length
    }
}

#[doc(hidden)]
pub struct AuthenticationCodeTypeSmsBuilder {
    inner: AuthenticationCodeTypeSms,
}

#[deprecated]
pub type RTDAuthenticationCodeTypeSmsBuilder = AuthenticationCodeTypeSmsBuilder;

impl AuthenticationCodeTypeSmsBuilder {
    pub fn build(&self) -> AuthenticationCodeTypeSms {
        self.inner.clone()
    }

    pub fn length(&mut self, length: i32) -> &mut Self {
        self.inner.length = length;
        self
    }
}

impl AsRef<AuthenticationCodeTypeSms> for AuthenticationCodeTypeSms {
    fn as_ref(&self) -> &AuthenticationCodeTypeSms {
        self
    }
}

impl AsRef<AuthenticationCodeTypeSms> for AuthenticationCodeTypeSmsBuilder {
    fn as_ref(&self) -> &AuthenticationCodeTypeSms {
        &self.inner
    }
}

/// An authentication code is delivered via a private Telegram message, which can be viewed from another active session
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeTelegramMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Length of the code

    #[serde(default)]
    length: i32,
}

impl RObject for AuthenticationCodeTypeTelegramMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeTelegramMessage {}

impl AuthenticationCodeTypeTelegramMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthenticationCodeTypeTelegramMessageBuilder {
        let mut inner = AuthenticationCodeTypeTelegramMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthenticationCodeTypeTelegramMessageBuilder { inner }
    }

    pub fn length(&self) -> i32 {
        self.length
    }
}

#[doc(hidden)]
pub struct AuthenticationCodeTypeTelegramMessageBuilder {
    inner: AuthenticationCodeTypeTelegramMessage,
}

#[deprecated]
pub type RTDAuthenticationCodeTypeTelegramMessageBuilder =
    AuthenticationCodeTypeTelegramMessageBuilder;

impl AuthenticationCodeTypeTelegramMessageBuilder {
    pub fn build(&self) -> AuthenticationCodeTypeTelegramMessage {
        self.inner.clone()
    }

    pub fn length(&mut self, length: i32) -> &mut Self {
        self.inner.length = length;
        self
    }
}

impl AsRef<AuthenticationCodeTypeTelegramMessage> for AuthenticationCodeTypeTelegramMessage {
    fn as_ref(&self) -> &AuthenticationCodeTypeTelegramMessage {
        self
    }
}

impl AsRef<AuthenticationCodeTypeTelegramMessage> for AuthenticationCodeTypeTelegramMessageBuilder {
    fn as_ref(&self) -> &AuthenticationCodeTypeTelegramMessage {
        &self.inner
    }
}
