use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Provides information about the method by which an authentication code is delivered to the user
pub trait TDAuthenticationCodeType: Debug + RObject {}

/// Provides information about the method by which an authentication code is delivered to the user
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum AuthenticationCodeType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// An authentication code is delivered via a phone call to the specified phone number
    #[serde(rename = "authenticationCodeTypeCall")]
    Call(AuthenticationCodeTypeCall),
    /// An authentication code is delivered via Firebase Authentication to the official Android application
    #[serde(rename = "authenticationCodeTypeFirebaseAndroid")]
    FirebaseAndroid(AuthenticationCodeTypeFirebaseAndroid),
    /// An authentication code is delivered via Firebase Authentication to the official iOS application
    #[serde(rename = "authenticationCodeTypeFirebaseIos")]
    FirebaseIos(AuthenticationCodeTypeFirebaseIos),
    /// An authentication code is delivered by an immediately canceled call to the specified phone number. The phone number that calls is the code that must be entered automatically
    #[serde(rename = "authenticationCodeTypeFlashCall")]
    FlashCall(AuthenticationCodeTypeFlashCall),
    /// An authentication code is delivered to https://fragment.com. The user must be logged in there via a wallet owning the phone number's NFT
    #[serde(rename = "authenticationCodeTypeFragment")]
    Fragment(AuthenticationCodeTypeFragment),
    /// An authentication code is delivered by an immediately canceled call to the specified phone number. The last digits of the phone number that calls are the code that must be entered manually by the user
    #[serde(rename = "authenticationCodeTypeMissedCall")]
    MissedCall(AuthenticationCodeTypeMissedCall),
    /// An authentication code is delivered via an SMS message to the specified phone number; applications may not receive this type of code
    #[serde(rename = "authenticationCodeTypeSms")]
    Sms(AuthenticationCodeTypeSms),
    /// An authentication code is delivered via a private Telegram message, which can be viewed from another active session
    #[serde(rename = "authenticationCodeTypeTelegramMessage")]
    TelegramMessage(AuthenticationCodeTypeTelegramMessage),
}

impl RObject for AuthenticationCodeType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            AuthenticationCodeType::Call(t) => t.extra(),
            AuthenticationCodeType::FirebaseAndroid(t) => t.extra(),
            AuthenticationCodeType::FirebaseIos(t) => t.extra(),
            AuthenticationCodeType::FlashCall(t) => t.extra(),
            AuthenticationCodeType::Fragment(t) => t.extra(),
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
            AuthenticationCodeType::FirebaseAndroid(t) => t.client_id(),
            AuthenticationCodeType::FirebaseIos(t) => t.client_id(),
            AuthenticationCodeType::FlashCall(t) => t.client_id(),
            AuthenticationCodeType::Fragment(t) => t.client_id(),
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

/// An authentication code is delivered via Firebase Authentication to the official Android application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeFirebaseAndroid {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Nonce to pass to the SafetyNet Attestation API

    #[serde(default)]
    nonce: String,
    /// Length of the code

    #[serde(default)]
    length: i32,
}

impl RObject for AuthenticationCodeTypeFirebaseAndroid {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeFirebaseAndroid {}

impl AuthenticationCodeTypeFirebaseAndroid {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthenticationCodeTypeFirebaseAndroidBuilder {
        let mut inner = AuthenticationCodeTypeFirebaseAndroid::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthenticationCodeTypeFirebaseAndroidBuilder { inner }
    }

    pub fn nonce(&self) -> &String {
        &self.nonce
    }

    pub fn length(&self) -> i32 {
        self.length
    }
}

#[doc(hidden)]
pub struct AuthenticationCodeTypeFirebaseAndroidBuilder {
    inner: AuthenticationCodeTypeFirebaseAndroid,
}

#[deprecated]
pub type RTDAuthenticationCodeTypeFirebaseAndroidBuilder =
    AuthenticationCodeTypeFirebaseAndroidBuilder;

impl AuthenticationCodeTypeFirebaseAndroidBuilder {
    pub fn build(&self) -> AuthenticationCodeTypeFirebaseAndroid {
        self.inner.clone()
    }

    pub fn nonce<T: AsRef<str>>(&mut self, nonce: T) -> &mut Self {
        self.inner.nonce = nonce.as_ref().to_string();
        self
    }

    pub fn length(&mut self, length: i32) -> &mut Self {
        self.inner.length = length;
        self
    }
}

impl AsRef<AuthenticationCodeTypeFirebaseAndroid> for AuthenticationCodeTypeFirebaseAndroid {
    fn as_ref(&self) -> &AuthenticationCodeTypeFirebaseAndroid {
        self
    }
}

impl AsRef<AuthenticationCodeTypeFirebaseAndroid> for AuthenticationCodeTypeFirebaseAndroidBuilder {
    fn as_ref(&self) -> &AuthenticationCodeTypeFirebaseAndroid {
        &self.inner
    }
}

/// An authentication code is delivered via Firebase Authentication to the official iOS application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeFirebaseIos {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Receipt of successful application token validation to compare with receipt from push notification

    #[serde(default)]
    receipt: String,
    /// Time after the next authentication method is supposed to be used if verification push notification isn't received, in seconds

    #[serde(default)]
    push_timeout: i32,
    /// Length of the code

    #[serde(default)]
    length: i32,
}

impl RObject for AuthenticationCodeTypeFirebaseIos {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeFirebaseIos {}

impl AuthenticationCodeTypeFirebaseIos {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthenticationCodeTypeFirebaseIosBuilder {
        let mut inner = AuthenticationCodeTypeFirebaseIos::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthenticationCodeTypeFirebaseIosBuilder { inner }
    }

    pub fn receipt(&self) -> &String {
        &self.receipt
    }

    pub fn push_timeout(&self) -> i32 {
        self.push_timeout
    }

    pub fn length(&self) -> i32 {
        self.length
    }
}

#[doc(hidden)]
pub struct AuthenticationCodeTypeFirebaseIosBuilder {
    inner: AuthenticationCodeTypeFirebaseIos,
}

#[deprecated]
pub type RTDAuthenticationCodeTypeFirebaseIosBuilder = AuthenticationCodeTypeFirebaseIosBuilder;

impl AuthenticationCodeTypeFirebaseIosBuilder {
    pub fn build(&self) -> AuthenticationCodeTypeFirebaseIos {
        self.inner.clone()
    }

    pub fn receipt<T: AsRef<str>>(&mut self, receipt: T) -> &mut Self {
        self.inner.receipt = receipt.as_ref().to_string();
        self
    }

    pub fn push_timeout(&mut self, push_timeout: i32) -> &mut Self {
        self.inner.push_timeout = push_timeout;
        self
    }

    pub fn length(&mut self, length: i32) -> &mut Self {
        self.inner.length = length;
        self
    }
}

impl AsRef<AuthenticationCodeTypeFirebaseIos> for AuthenticationCodeTypeFirebaseIos {
    fn as_ref(&self) -> &AuthenticationCodeTypeFirebaseIos {
        self
    }
}

impl AsRef<AuthenticationCodeTypeFirebaseIos> for AuthenticationCodeTypeFirebaseIosBuilder {
    fn as_ref(&self) -> &AuthenticationCodeTypeFirebaseIos {
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

/// An authentication code is delivered to https://fragment.com. The user must be logged in there via a wallet owning the phone number's NFT
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeFragment {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// URL to open to receive the code

    #[serde(default)]
    url: String,
    /// Length of the code

    #[serde(default)]
    length: i32,
}

impl RObject for AuthenticationCodeTypeFragment {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeFragment {}

impl AuthenticationCodeTypeFragment {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthenticationCodeTypeFragmentBuilder {
        let mut inner = AuthenticationCodeTypeFragment::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthenticationCodeTypeFragmentBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn length(&self) -> i32 {
        self.length
    }
}

#[doc(hidden)]
pub struct AuthenticationCodeTypeFragmentBuilder {
    inner: AuthenticationCodeTypeFragment,
}

#[deprecated]
pub type RTDAuthenticationCodeTypeFragmentBuilder = AuthenticationCodeTypeFragmentBuilder;

impl AuthenticationCodeTypeFragmentBuilder {
    pub fn build(&self) -> AuthenticationCodeTypeFragment {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn length(&mut self, length: i32) -> &mut Self {
        self.inner.length = length;
        self
    }
}

impl AsRef<AuthenticationCodeTypeFragment> for AuthenticationCodeTypeFragment {
    fn as_ref(&self) -> &AuthenticationCodeTypeFragment {
        self
    }
}

impl AsRef<AuthenticationCodeTypeFragment> for AuthenticationCodeTypeFragmentBuilder {
    fn as_ref(&self) -> &AuthenticationCodeTypeFragment {
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

/// An authentication code is delivered via an SMS message to the specified phone number; applications may not receive this type of code
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
