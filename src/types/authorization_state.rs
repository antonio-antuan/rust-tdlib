use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents the current authorization state of the TDLib client
pub trait TDAuthorizationState: Debug + RObject {}

/// Represents the current authorization state of the TDLib client
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AuthorizationState {
    #[doc(hidden)]
    _Default,
    /// TDLib client is in its final state. All databases are closed and all resources are released. No other updates will be received after this. All queries will be responded to with error code 500. To continue working, one must create a new instance of the TDLib client
    #[serde(rename = "authorizationStateClosed")]
    Closed(AuthorizationStateClosed),
    /// TDLib is closing, all subsequent queries will be answered with the error 500. Note that closing TDLib can take a while. All resources will be freed only after authorizationStateClosed has been received
    #[serde(rename = "authorizationStateClosing")]
    Closing(AuthorizationStateClosing),
    /// The user is currently logging out
    #[serde(rename = "authorizationStateLoggingOut")]
    LoggingOut(AuthorizationStateLoggingOut),
    /// The user has been successfully authorized. TDLib is now ready to answer queries
    #[serde(rename = "authorizationStateReady")]
    Ready(AuthorizationStateReady),
    /// TDLib needs the user's authentication code to authorize
    #[serde(rename = "authorizationStateWaitCode")]
    WaitCode(AuthorizationStateWaitCode),
    /// TDLib needs an encryption key to decrypt the local database
    #[serde(rename = "authorizationStateWaitEncryptionKey")]
    WaitEncryptionKey(AuthorizationStateWaitEncryptionKey),
    /// The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link
    #[serde(rename = "authorizationStateWaitOtherDeviceConfirmation")]
    WaitOtherDeviceConfirmation(AuthorizationStateWaitOtherDeviceConfirmation),
    /// The user has been authorized, but needs to enter a password to start using the application
    #[serde(rename = "authorizationStateWaitPassword")]
    WaitPassword(AuthorizationStateWaitPassword),
    /// TDLib needs the user's phone number to authorize. Call `setAuthenticationPhoneNumber` to provide the phone number, or use `requestQrCodeAuthentication`, or `checkAuthenticationBotToken` for other authentication options
    #[serde(rename = "authorizationStateWaitPhoneNumber")]
    WaitPhoneNumber(AuthorizationStateWaitPhoneNumber),
    /// The user is unregistered and need to accept terms of service and enter their first name and last name to finish registration
    #[serde(rename = "authorizationStateWaitRegistration")]
    WaitRegistration(AuthorizationStateWaitRegistration),
    /// TDLib needs TdlibParameters for initialization
    #[serde(rename = "authorizationStateWaitTdlibParameters")]
    WaitTdlibParameters(AuthorizationStateWaitTdlibParameters),
    /// Returns the current authorization state; this is an offline request. For informational purposes only. Use updateAuthorizationState instead to maintain the current authorization state. Can be called before initialization
    #[serde(rename = "getAuthorizationState")]
    GetAuthorizationState(GetAuthorizationState),
}

impl Default for AuthorizationState {
    fn default() -> Self {
        AuthorizationState::_Default
    }
}

impl RObject for AuthorizationState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            AuthorizationState::Closed(t) => t.extra(),
            AuthorizationState::Closing(t) => t.extra(),
            AuthorizationState::LoggingOut(t) => t.extra(),
            AuthorizationState::Ready(t) => t.extra(),
            AuthorizationState::WaitCode(t) => t.extra(),
            AuthorizationState::WaitEncryptionKey(t) => t.extra(),
            AuthorizationState::WaitOtherDeviceConfirmation(t) => t.extra(),
            AuthorizationState::WaitPassword(t) => t.extra(),
            AuthorizationState::WaitPhoneNumber(t) => t.extra(),
            AuthorizationState::WaitRegistration(t) => t.extra(),
            AuthorizationState::WaitTdlibParameters(t) => t.extra(),
            AuthorizationState::GetAuthorizationState(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            AuthorizationState::Closed(t) => t.client_id(),
            AuthorizationState::Closing(t) => t.client_id(),
            AuthorizationState::LoggingOut(t) => t.client_id(),
            AuthorizationState::Ready(t) => t.client_id(),
            AuthorizationState::WaitCode(t) => t.client_id(),
            AuthorizationState::WaitEncryptionKey(t) => t.client_id(),
            AuthorizationState::WaitOtherDeviceConfirmation(t) => t.client_id(),
            AuthorizationState::WaitPassword(t) => t.client_id(),
            AuthorizationState::WaitPhoneNumber(t) => t.client_id(),
            AuthorizationState::WaitRegistration(t) => t.client_id(),
            AuthorizationState::WaitTdlibParameters(t) => t.client_id(),
            AuthorizationState::GetAuthorizationState(t) => t.client_id(),

            _ => None,
        }
    }
}

impl AuthorizationState {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, AuthorizationState::_Default)
    }
}

impl AsRef<AuthorizationState> for AuthorizationState {
    fn as_ref(&self) -> &AuthorizationState {
        self
    }
}

/// TDLib client is in its final state. All databases are closed and all resources are released. No other updates will be received after this. All queries will be responded to with error code 500. To continue working, one must create a new instance of the TDLib client
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateClosed {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for AuthorizationStateClosed {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthorizationState for AuthorizationStateClosed {}

impl AuthorizationStateClosed {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthorizationStateClosedBuilder {
        let mut inner = AuthorizationStateClosed::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthorizationStateClosedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct AuthorizationStateClosedBuilder {
    inner: AuthorizationStateClosed,
}

#[deprecated]
pub type RTDAuthorizationStateClosedBuilder = AuthorizationStateClosedBuilder;

impl AuthorizationStateClosedBuilder {
    pub fn build(&self) -> AuthorizationStateClosed {
        self.inner.clone()
    }
}

impl AsRef<AuthorizationStateClosed> for AuthorizationStateClosed {
    fn as_ref(&self) -> &AuthorizationStateClosed {
        self
    }
}

impl AsRef<AuthorizationStateClosed> for AuthorizationStateClosedBuilder {
    fn as_ref(&self) -> &AuthorizationStateClosed {
        &self.inner
    }
}

/// TDLib is closing, all subsequent queries will be answered with the error 500. Note that closing TDLib can take a while. All resources will be freed only after authorizationStateClosed has been received
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateClosing {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for AuthorizationStateClosing {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthorizationState for AuthorizationStateClosing {}

impl AuthorizationStateClosing {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthorizationStateClosingBuilder {
        let mut inner = AuthorizationStateClosing::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthorizationStateClosingBuilder { inner }
    }
}

#[doc(hidden)]
pub struct AuthorizationStateClosingBuilder {
    inner: AuthorizationStateClosing,
}

#[deprecated]
pub type RTDAuthorizationStateClosingBuilder = AuthorizationStateClosingBuilder;

impl AuthorizationStateClosingBuilder {
    pub fn build(&self) -> AuthorizationStateClosing {
        self.inner.clone()
    }
}

impl AsRef<AuthorizationStateClosing> for AuthorizationStateClosing {
    fn as_ref(&self) -> &AuthorizationStateClosing {
        self
    }
}

impl AsRef<AuthorizationStateClosing> for AuthorizationStateClosingBuilder {
    fn as_ref(&self) -> &AuthorizationStateClosing {
        &self.inner
    }
}

/// The user is currently logging out
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateLoggingOut {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for AuthorizationStateLoggingOut {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthorizationState for AuthorizationStateLoggingOut {}

impl AuthorizationStateLoggingOut {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthorizationStateLoggingOutBuilder {
        let mut inner = AuthorizationStateLoggingOut::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthorizationStateLoggingOutBuilder { inner }
    }
}

#[doc(hidden)]
pub struct AuthorizationStateLoggingOutBuilder {
    inner: AuthorizationStateLoggingOut,
}

#[deprecated]
pub type RTDAuthorizationStateLoggingOutBuilder = AuthorizationStateLoggingOutBuilder;

impl AuthorizationStateLoggingOutBuilder {
    pub fn build(&self) -> AuthorizationStateLoggingOut {
        self.inner.clone()
    }
}

impl AsRef<AuthorizationStateLoggingOut> for AuthorizationStateLoggingOut {
    fn as_ref(&self) -> &AuthorizationStateLoggingOut {
        self
    }
}

impl AsRef<AuthorizationStateLoggingOut> for AuthorizationStateLoggingOutBuilder {
    fn as_ref(&self) -> &AuthorizationStateLoggingOut {
        &self.inner
    }
}

/// The user has been successfully authorized. TDLib is now ready to answer queries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateReady {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for AuthorizationStateReady {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthorizationState for AuthorizationStateReady {}

impl AuthorizationStateReady {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthorizationStateReadyBuilder {
        let mut inner = AuthorizationStateReady::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthorizationStateReadyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct AuthorizationStateReadyBuilder {
    inner: AuthorizationStateReady,
}

#[deprecated]
pub type RTDAuthorizationStateReadyBuilder = AuthorizationStateReadyBuilder;

impl AuthorizationStateReadyBuilder {
    pub fn build(&self) -> AuthorizationStateReady {
        self.inner.clone()
    }
}

impl AsRef<AuthorizationStateReady> for AuthorizationStateReady {
    fn as_ref(&self) -> &AuthorizationStateReady {
        self
    }
}

impl AsRef<AuthorizationStateReady> for AuthorizationStateReadyBuilder {
    fn as_ref(&self) -> &AuthorizationStateReady {
        &self.inner
    }
}

/// TDLib needs the user's authentication code to authorize
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Information about the authorization code that was sent
    code_info: AuthenticationCodeInfo,
}

impl RObject for AuthorizationStateWaitCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthorizationState for AuthorizationStateWaitCode {}

impl AuthorizationStateWaitCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthorizationStateWaitCodeBuilder {
        let mut inner = AuthorizationStateWaitCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthorizationStateWaitCodeBuilder { inner }
    }

    pub fn code_info(&self) -> &AuthenticationCodeInfo {
        &self.code_info
    }
}

#[doc(hidden)]
pub struct AuthorizationStateWaitCodeBuilder {
    inner: AuthorizationStateWaitCode,
}

#[deprecated]
pub type RTDAuthorizationStateWaitCodeBuilder = AuthorizationStateWaitCodeBuilder;

impl AuthorizationStateWaitCodeBuilder {
    pub fn build(&self) -> AuthorizationStateWaitCode {
        self.inner.clone()
    }

    pub fn code_info<T: AsRef<AuthenticationCodeInfo>>(&mut self, code_info: T) -> &mut Self {
        self.inner.code_info = code_info.as_ref().clone();
        self
    }
}

impl AsRef<AuthorizationStateWaitCode> for AuthorizationStateWaitCode {
    fn as_ref(&self) -> &AuthorizationStateWaitCode {
        self
    }
}

impl AsRef<AuthorizationStateWaitCode> for AuthorizationStateWaitCodeBuilder {
    fn as_ref(&self) -> &AuthorizationStateWaitCode {
        &self.inner
    }
}

/// TDLib needs an encryption key to decrypt the local database
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitEncryptionKey {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the database is currently encrypted

    #[serde(default)]
    is_encrypted: bool,
}

impl RObject for AuthorizationStateWaitEncryptionKey {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthorizationState for AuthorizationStateWaitEncryptionKey {}

impl AuthorizationStateWaitEncryptionKey {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthorizationStateWaitEncryptionKeyBuilder {
        let mut inner = AuthorizationStateWaitEncryptionKey::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthorizationStateWaitEncryptionKeyBuilder { inner }
    }

    pub fn is_encrypted(&self) -> bool {
        self.is_encrypted
    }
}

#[doc(hidden)]
pub struct AuthorizationStateWaitEncryptionKeyBuilder {
    inner: AuthorizationStateWaitEncryptionKey,
}

#[deprecated]
pub type RTDAuthorizationStateWaitEncryptionKeyBuilder = AuthorizationStateWaitEncryptionKeyBuilder;

impl AuthorizationStateWaitEncryptionKeyBuilder {
    pub fn build(&self) -> AuthorizationStateWaitEncryptionKey {
        self.inner.clone()
    }

    pub fn is_encrypted(&mut self, is_encrypted: bool) -> &mut Self {
        self.inner.is_encrypted = is_encrypted;
        self
    }
}

impl AsRef<AuthorizationStateWaitEncryptionKey> for AuthorizationStateWaitEncryptionKey {
    fn as_ref(&self) -> &AuthorizationStateWaitEncryptionKey {
        self
    }
}

impl AsRef<AuthorizationStateWaitEncryptionKey> for AuthorizationStateWaitEncryptionKeyBuilder {
    fn as_ref(&self) -> &AuthorizationStateWaitEncryptionKey {
        &self.inner
    }
}

/// The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitOtherDeviceConfirmation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A tg:// URL for the QR code. The link will be updated frequently

    #[serde(default)]
    link: String,
}

impl RObject for AuthorizationStateWaitOtherDeviceConfirmation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthorizationState for AuthorizationStateWaitOtherDeviceConfirmation {}

impl AuthorizationStateWaitOtherDeviceConfirmation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthorizationStateWaitOtherDeviceConfirmationBuilder {
        let mut inner = AuthorizationStateWaitOtherDeviceConfirmation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthorizationStateWaitOtherDeviceConfirmationBuilder { inner }
    }

    pub fn link(&self) -> &String {
        &self.link
    }
}

#[doc(hidden)]
pub struct AuthorizationStateWaitOtherDeviceConfirmationBuilder {
    inner: AuthorizationStateWaitOtherDeviceConfirmation,
}

#[deprecated]
pub type RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder =
    AuthorizationStateWaitOtherDeviceConfirmationBuilder;

impl AuthorizationStateWaitOtherDeviceConfirmationBuilder {
    pub fn build(&self) -> AuthorizationStateWaitOtherDeviceConfirmation {
        self.inner.clone()
    }

    pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
        self.inner.link = link.as_ref().to_string();
        self
    }
}

impl AsRef<AuthorizationStateWaitOtherDeviceConfirmation>
    for AuthorizationStateWaitOtherDeviceConfirmation
{
    fn as_ref(&self) -> &AuthorizationStateWaitOtherDeviceConfirmation {
        self
    }
}

impl AsRef<AuthorizationStateWaitOtherDeviceConfirmation>
    for AuthorizationStateWaitOtherDeviceConfirmationBuilder
{
    fn as_ref(&self) -> &AuthorizationStateWaitOtherDeviceConfirmation {
        &self.inner
    }
}

/// The user has been authorized, but needs to enter a password to start using the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitPassword {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Hint for the password; may be empty

    #[serde(default)]
    password_hint: String,
    /// True, if a recovery email address has been set up

    #[serde(default)]
    has_recovery_email_address: bool,
    /// Pattern of the email address to which the recovery email was sent; empty until a recovery email has been sent

    #[serde(default)]
    recovery_email_address_pattern: String,
}

impl RObject for AuthorizationStateWaitPassword {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthorizationState for AuthorizationStateWaitPassword {}

impl AuthorizationStateWaitPassword {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthorizationStateWaitPasswordBuilder {
        let mut inner = AuthorizationStateWaitPassword::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthorizationStateWaitPasswordBuilder { inner }
    }

    pub fn password_hint(&self) -> &String {
        &self.password_hint
    }

    pub fn has_recovery_email_address(&self) -> bool {
        self.has_recovery_email_address
    }

    pub fn recovery_email_address_pattern(&self) -> &String {
        &self.recovery_email_address_pattern
    }
}

#[doc(hidden)]
pub struct AuthorizationStateWaitPasswordBuilder {
    inner: AuthorizationStateWaitPassword,
}

#[deprecated]
pub type RTDAuthorizationStateWaitPasswordBuilder = AuthorizationStateWaitPasswordBuilder;

impl AuthorizationStateWaitPasswordBuilder {
    pub fn build(&self) -> AuthorizationStateWaitPassword {
        self.inner.clone()
    }

    pub fn password_hint<T: AsRef<str>>(&mut self, password_hint: T) -> &mut Self {
        self.inner.password_hint = password_hint.as_ref().to_string();
        self
    }

    pub fn has_recovery_email_address(&mut self, has_recovery_email_address: bool) -> &mut Self {
        self.inner.has_recovery_email_address = has_recovery_email_address;
        self
    }

    pub fn recovery_email_address_pattern<T: AsRef<str>>(
        &mut self,
        recovery_email_address_pattern: T,
    ) -> &mut Self {
        self.inner.recovery_email_address_pattern =
            recovery_email_address_pattern.as_ref().to_string();
        self
    }
}

impl AsRef<AuthorizationStateWaitPassword> for AuthorizationStateWaitPassword {
    fn as_ref(&self) -> &AuthorizationStateWaitPassword {
        self
    }
}

impl AsRef<AuthorizationStateWaitPassword> for AuthorizationStateWaitPasswordBuilder {
    fn as_ref(&self) -> &AuthorizationStateWaitPassword {
        &self.inner
    }
}

/// TDLib needs the user's phone number to authorize. Call `setAuthenticationPhoneNumber` to provide the phone number, or use `requestQrCodeAuthentication`, or `checkAuthenticationBotToken` for other authentication options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitPhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for AuthorizationStateWaitPhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthorizationState for AuthorizationStateWaitPhoneNumber {}

impl AuthorizationStateWaitPhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthorizationStateWaitPhoneNumberBuilder {
        let mut inner = AuthorizationStateWaitPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthorizationStateWaitPhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct AuthorizationStateWaitPhoneNumberBuilder {
    inner: AuthorizationStateWaitPhoneNumber,
}

#[deprecated]
pub type RTDAuthorizationStateWaitPhoneNumberBuilder = AuthorizationStateWaitPhoneNumberBuilder;

impl AuthorizationStateWaitPhoneNumberBuilder {
    pub fn build(&self) -> AuthorizationStateWaitPhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<AuthorizationStateWaitPhoneNumber> for AuthorizationStateWaitPhoneNumber {
    fn as_ref(&self) -> &AuthorizationStateWaitPhoneNumber {
        self
    }
}

impl AsRef<AuthorizationStateWaitPhoneNumber> for AuthorizationStateWaitPhoneNumberBuilder {
    fn as_ref(&self) -> &AuthorizationStateWaitPhoneNumber {
        &self.inner
    }
}

/// The user is unregistered and need to accept terms of service and enter their first name and last name to finish registration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitRegistration {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Telegram terms of service
    terms_of_service: TermsOfService,
}

impl RObject for AuthorizationStateWaitRegistration {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthorizationState for AuthorizationStateWaitRegistration {}

impl AuthorizationStateWaitRegistration {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthorizationStateWaitRegistrationBuilder {
        let mut inner = AuthorizationStateWaitRegistration::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthorizationStateWaitRegistrationBuilder { inner }
    }

    pub fn terms_of_service(&self) -> &TermsOfService {
        &self.terms_of_service
    }
}

#[doc(hidden)]
pub struct AuthorizationStateWaitRegistrationBuilder {
    inner: AuthorizationStateWaitRegistration,
}

#[deprecated]
pub type RTDAuthorizationStateWaitRegistrationBuilder = AuthorizationStateWaitRegistrationBuilder;

impl AuthorizationStateWaitRegistrationBuilder {
    pub fn build(&self) -> AuthorizationStateWaitRegistration {
        self.inner.clone()
    }

    pub fn terms_of_service<T: AsRef<TermsOfService>>(&mut self, terms_of_service: T) -> &mut Self {
        self.inner.terms_of_service = terms_of_service.as_ref().clone();
        self
    }
}

impl AsRef<AuthorizationStateWaitRegistration> for AuthorizationStateWaitRegistration {
    fn as_ref(&self) -> &AuthorizationStateWaitRegistration {
        self
    }
}

impl AsRef<AuthorizationStateWaitRegistration> for AuthorizationStateWaitRegistrationBuilder {
    fn as_ref(&self) -> &AuthorizationStateWaitRegistration {
        &self.inner
    }
}

/// TDLib needs TdlibParameters for initialization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitTdlibParameters {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for AuthorizationStateWaitTdlibParameters {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthorizationState for AuthorizationStateWaitTdlibParameters {}

impl AuthorizationStateWaitTdlibParameters {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AuthorizationStateWaitTdlibParametersBuilder {
        let mut inner = AuthorizationStateWaitTdlibParameters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AuthorizationStateWaitTdlibParametersBuilder { inner }
    }
}

#[doc(hidden)]
pub struct AuthorizationStateWaitTdlibParametersBuilder {
    inner: AuthorizationStateWaitTdlibParameters,
}

#[deprecated]
pub type RTDAuthorizationStateWaitTdlibParametersBuilder =
    AuthorizationStateWaitTdlibParametersBuilder;

impl AuthorizationStateWaitTdlibParametersBuilder {
    pub fn build(&self) -> AuthorizationStateWaitTdlibParameters {
        self.inner.clone()
    }
}

impl AsRef<AuthorizationStateWaitTdlibParameters> for AuthorizationStateWaitTdlibParameters {
    fn as_ref(&self) -> &AuthorizationStateWaitTdlibParameters {
        self
    }
}

impl AsRef<AuthorizationStateWaitTdlibParameters> for AuthorizationStateWaitTdlibParametersBuilder {
    fn as_ref(&self) -> &AuthorizationStateWaitTdlibParameters {
        &self.inner
    }
}
