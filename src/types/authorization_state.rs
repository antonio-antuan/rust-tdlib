
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents the current authorization state of the client
pub trait TDAuthorizationState: Debug + RObject {}

/// Represents the current authorization state of the client
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AuthorizationState {
  #[doc(hidden)] _Default(()),
  /// TDLib client is in its final state. All databases are closed and all resources are released. No other updates will be received after this. All queries will be responded to with error code 500. To continue working, one should create a new instance of the TDLib client
  Closed(AuthorizationStateClosed),
  /// TDLib is closing, all subsequent queries will be answered with the error 500. Note that closing TDLib can take a while. All resources will be freed only after authorizationStateClosed has been received
  Closing(AuthorizationStateClosing),
  /// The user is currently logging out
  LoggingOut(AuthorizationStateLoggingOut),
  /// The user has been successfully authorized. TDLib is now ready to answer queries
  Ready(AuthorizationStateReady),
  /// TDLib needs the user's authentication code to authorize
  WaitCode(AuthorizationStateWaitCode),
  /// TDLib needs an encryption key to decrypt the local database
  WaitEncryptionKey(AuthorizationStateWaitEncryptionKey),
  /// The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link
  WaitOtherDeviceConfirmation(AuthorizationStateWaitOtherDeviceConfirmation),
  /// The user has been authorized, but needs to enter a password to start using the application
  WaitPassword(AuthorizationStateWaitPassword),
  /// TDLib needs the user's phone number to authorize. Call `setAuthenticationPhoneNumber` to provide the phone number, or use `requestQrCodeAuthentication`, or `checkAuthenticationBotToken` for other authentication options
  WaitPhoneNumber(AuthorizationStateWaitPhoneNumber),
  /// The user is unregistered and need to accept terms of service and enter their first name and last name to finish registration
  WaitRegistration(AuthorizationStateWaitRegistration),
  /// TDLib needs TdlibParameters for initialization
  WaitTdlibParameters(AuthorizationStateWaitTdlibParameters),
  /// Returns the current authorization state; this is an offline request. For informational purposes only. Use updateAuthorizationState instead to maintain the current authorization state
  GetAuthorizationState(GetAuthorizationState),

}

impl Default for AuthorizationState {
  fn default() -> Self { AuthorizationState::_Default(()) }
}

impl<'de> Deserialize<'de> for AuthorizationState {
  fn deserialize<D>(deserializer: D) -> Result<AuthorizationState, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      AuthorizationState,
      (authorizationStateClosed, Closed);
      (authorizationStateClosing, Closing);
      (authorizationStateLoggingOut, LoggingOut);
      (authorizationStateReady, Ready);
      (authorizationStateWaitCode, WaitCode);
      (authorizationStateWaitEncryptionKey, WaitEncryptionKey);
      (authorizationStateWaitOtherDeviceConfirmation, WaitOtherDeviceConfirmation);
      (authorizationStateWaitPassword, WaitPassword);
      (authorizationStateWaitPhoneNumber, WaitPhoneNumber);
      (authorizationStateWaitRegistration, WaitRegistration);
      (authorizationStateWaitTdlibParameters, WaitTdlibParameters);
      (getAuthorizationState, GetAuthorizationState);

    )(deserializer)
  }
}

impl RObject for AuthorizationState {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      AuthorizationState::Closed(t) => t.td_name(),
      AuthorizationState::Closing(t) => t.td_name(),
      AuthorizationState::LoggingOut(t) => t.td_name(),
      AuthorizationState::Ready(t) => t.td_name(),
      AuthorizationState::WaitCode(t) => t.td_name(),
      AuthorizationState::WaitEncryptionKey(t) => t.td_name(),
      AuthorizationState::WaitOtherDeviceConfirmation(t) => t.td_name(),
      AuthorizationState::WaitPassword(t) => t.td_name(),
      AuthorizationState::WaitPhoneNumber(t) => t.td_name(),
      AuthorizationState::WaitRegistration(t) => t.td_name(),
      AuthorizationState::WaitTdlibParameters(t) => t.td_name(),
      AuthorizationState::GetAuthorizationState(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
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
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl AuthorizationState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let AuthorizationState::_Default(_) = self { true } else { false } }
}

impl AsRef<AuthorizationState> for AuthorizationState {
  fn as_ref(&self) -> &AuthorizationState { self }
}







/// TDLib client is in its final state. All databases are closed and all resources are released. No other updates will be received after this. All queries will be responded to with error code 500. To continue working, one should create a new instance of the TDLib client
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateClosed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for AuthorizationStateClosed {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateClosed" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateClosed {}



impl AuthorizationStateClosed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateClosedBuilder {
    let mut inner = AuthorizationStateClosed::default();
    inner.td_name = "authorizationStateClosed".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateClosedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateClosedBuilder {
  inner: AuthorizationStateClosed
}

impl RTDAuthorizationStateClosedBuilder {
  pub fn build(&self) -> AuthorizationStateClosed { self.inner.clone() }

}

impl AsRef<AuthorizationStateClosed> for AuthorizationStateClosed {
  fn as_ref(&self) -> &AuthorizationStateClosed { self }
}

impl AsRef<AuthorizationStateClosed> for RTDAuthorizationStateClosedBuilder {
  fn as_ref(&self) -> &AuthorizationStateClosed { &self.inner }
}







/// TDLib is closing, all subsequent queries will be answered with the error 500. Note that closing TDLib can take a while. All resources will be freed only after authorizationStateClosed has been received
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateClosing {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for AuthorizationStateClosing {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateClosing" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateClosing {}



impl AuthorizationStateClosing {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateClosingBuilder {
    let mut inner = AuthorizationStateClosing::default();
    inner.td_name = "authorizationStateClosing".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateClosingBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateClosingBuilder {
  inner: AuthorizationStateClosing
}

impl RTDAuthorizationStateClosingBuilder {
  pub fn build(&self) -> AuthorizationStateClosing { self.inner.clone() }

}

impl AsRef<AuthorizationStateClosing> for AuthorizationStateClosing {
  fn as_ref(&self) -> &AuthorizationStateClosing { self }
}

impl AsRef<AuthorizationStateClosing> for RTDAuthorizationStateClosingBuilder {
  fn as_ref(&self) -> &AuthorizationStateClosing { &self.inner }
}







/// The user is currently logging out
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateLoggingOut {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for AuthorizationStateLoggingOut {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateLoggingOut" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateLoggingOut {}



impl AuthorizationStateLoggingOut {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateLoggingOutBuilder {
    let mut inner = AuthorizationStateLoggingOut::default();
    inner.td_name = "authorizationStateLoggingOut".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateLoggingOutBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateLoggingOutBuilder {
  inner: AuthorizationStateLoggingOut
}

impl RTDAuthorizationStateLoggingOutBuilder {
  pub fn build(&self) -> AuthorizationStateLoggingOut { self.inner.clone() }

}

impl AsRef<AuthorizationStateLoggingOut> for AuthorizationStateLoggingOut {
  fn as_ref(&self) -> &AuthorizationStateLoggingOut { self }
}

impl AsRef<AuthorizationStateLoggingOut> for RTDAuthorizationStateLoggingOutBuilder {
  fn as_ref(&self) -> &AuthorizationStateLoggingOut { &self.inner }
}







/// The user has been successfully authorized. TDLib is now ready to answer queries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateReady {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for AuthorizationStateReady {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateReady" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateReady {}



impl AuthorizationStateReady {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateReadyBuilder {
    let mut inner = AuthorizationStateReady::default();
    inner.td_name = "authorizationStateReady".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateReadyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateReadyBuilder {
  inner: AuthorizationStateReady
}

impl RTDAuthorizationStateReadyBuilder {
  pub fn build(&self) -> AuthorizationStateReady { self.inner.clone() }

}

impl AsRef<AuthorizationStateReady> for AuthorizationStateReady {
  fn as_ref(&self) -> &AuthorizationStateReady { self }
}

impl AsRef<AuthorizationStateReady> for RTDAuthorizationStateReadyBuilder {
  fn as_ref(&self) -> &AuthorizationStateReady { &self.inner }
}







/// TDLib needs the user's authentication code to authorize
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about the authorization code that was sent
  code_info: AuthenticationCodeInfo,
  
}

impl RObject for AuthorizationStateWaitCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateWaitCode {}



impl AuthorizationStateWaitCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateWaitCodeBuilder {
    let mut inner = AuthorizationStateWaitCode::default();
    inner.td_name = "authorizationStateWaitCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateWaitCodeBuilder { inner }
  }

  pub fn code_info(&self) -> &AuthenticationCodeInfo { &self.code_info }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitCodeBuilder {
  inner: AuthorizationStateWaitCode
}

impl RTDAuthorizationStateWaitCodeBuilder {
  pub fn build(&self) -> AuthorizationStateWaitCode { self.inner.clone() }

   
  pub fn code_info<T: AsRef<AuthenticationCodeInfo>>(&mut self, code_info: T) -> &mut Self {
    self.inner.code_info = code_info.as_ref().clone();
    self
  }

}

impl AsRef<AuthorizationStateWaitCode> for AuthorizationStateWaitCode {
  fn as_ref(&self) -> &AuthorizationStateWaitCode { self }
}

impl AsRef<AuthorizationStateWaitCode> for RTDAuthorizationStateWaitCodeBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitCode { &self.inner }
}







/// TDLib needs an encryption key to decrypt the local database
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitEncryptionKey {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the database is currently encrypted
  is_encrypted: bool,
  
}

impl RObject for AuthorizationStateWaitEncryptionKey {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitEncryptionKey" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateWaitEncryptionKey {}



impl AuthorizationStateWaitEncryptionKey {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateWaitEncryptionKeyBuilder {
    let mut inner = AuthorizationStateWaitEncryptionKey::default();
    inner.td_name = "authorizationStateWaitEncryptionKey".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateWaitEncryptionKeyBuilder { inner }
  }

  pub fn is_encrypted(&self) -> bool { self.is_encrypted }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitEncryptionKeyBuilder {
  inner: AuthorizationStateWaitEncryptionKey
}

impl RTDAuthorizationStateWaitEncryptionKeyBuilder {
  pub fn build(&self) -> AuthorizationStateWaitEncryptionKey { self.inner.clone() }

   
  pub fn is_encrypted(&mut self, is_encrypted: bool) -> &mut Self {
    self.inner.is_encrypted = is_encrypted;
    self
  }

}

impl AsRef<AuthorizationStateWaitEncryptionKey> for AuthorizationStateWaitEncryptionKey {
  fn as_ref(&self) -> &AuthorizationStateWaitEncryptionKey { self }
}

impl AsRef<AuthorizationStateWaitEncryptionKey> for RTDAuthorizationStateWaitEncryptionKeyBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitEncryptionKey { &self.inner }
}







/// The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitOtherDeviceConfirmation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A tg:// URL for the QR code. The link will be updated frequently
  link: String,
  
}

impl RObject for AuthorizationStateWaitOtherDeviceConfirmation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitOtherDeviceConfirmation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateWaitOtherDeviceConfirmation {}



impl AuthorizationStateWaitOtherDeviceConfirmation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder {
    let mut inner = AuthorizationStateWaitOtherDeviceConfirmation::default();
    inner.td_name = "authorizationStateWaitOtherDeviceConfirmation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder { inner }
  }

  pub fn link(&self) -> &String { &self.link }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder {
  inner: AuthorizationStateWaitOtherDeviceConfirmation
}

impl RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder {
  pub fn build(&self) -> AuthorizationStateWaitOtherDeviceConfirmation { self.inner.clone() }

   
  pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
    self.inner.link = link.as_ref().to_string();
    self
  }

}

impl AsRef<AuthorizationStateWaitOtherDeviceConfirmation> for AuthorizationStateWaitOtherDeviceConfirmation {
  fn as_ref(&self) -> &AuthorizationStateWaitOtherDeviceConfirmation { self }
}

impl AsRef<AuthorizationStateWaitOtherDeviceConfirmation> for RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitOtherDeviceConfirmation { &self.inner }
}







/// The user has been authorized, but needs to enter a password to start using the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Hint for the password; may be empty
  password_hint: String,
  /// True, if a recovery email address has been set up
  has_recovery_email_address: bool,
  /// Pattern of the email address to which the recovery email was sent; empty until a recovery email has been sent
  recovery_email_address_pattern: String,
  
}

impl RObject for AuthorizationStateWaitPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitPassword" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateWaitPassword {}



impl AuthorizationStateWaitPassword {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateWaitPasswordBuilder {
    let mut inner = AuthorizationStateWaitPassword::default();
    inner.td_name = "authorizationStateWaitPassword".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateWaitPasswordBuilder { inner }
  }

  pub fn password_hint(&self) -> &String { &self.password_hint }

  pub fn has_recovery_email_address(&self) -> bool { self.has_recovery_email_address }

  pub fn recovery_email_address_pattern(&self) -> &String { &self.recovery_email_address_pattern }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitPasswordBuilder {
  inner: AuthorizationStateWaitPassword
}

impl RTDAuthorizationStateWaitPasswordBuilder {
  pub fn build(&self) -> AuthorizationStateWaitPassword { self.inner.clone() }

   
  pub fn password_hint<T: AsRef<str>>(&mut self, password_hint: T) -> &mut Self {
    self.inner.password_hint = password_hint.as_ref().to_string();
    self
  }

   
  pub fn has_recovery_email_address(&mut self, has_recovery_email_address: bool) -> &mut Self {
    self.inner.has_recovery_email_address = has_recovery_email_address;
    self
  }

   
  pub fn recovery_email_address_pattern<T: AsRef<str>>(&mut self, recovery_email_address_pattern: T) -> &mut Self {
    self.inner.recovery_email_address_pattern = recovery_email_address_pattern.as_ref().to_string();
    self
  }

}

impl AsRef<AuthorizationStateWaitPassword> for AuthorizationStateWaitPassword {
  fn as_ref(&self) -> &AuthorizationStateWaitPassword { self }
}

impl AsRef<AuthorizationStateWaitPassword> for RTDAuthorizationStateWaitPasswordBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitPassword { &self.inner }
}







/// TDLib needs the user's phone number to authorize. Call `setAuthenticationPhoneNumber` to provide the phone number, or use `requestQrCodeAuthentication`, or `checkAuthenticationBotToken` for other authentication options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for AuthorizationStateWaitPhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitPhoneNumber" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateWaitPhoneNumber {}



impl AuthorizationStateWaitPhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateWaitPhoneNumberBuilder {
    let mut inner = AuthorizationStateWaitPhoneNumber::default();
    inner.td_name = "authorizationStateWaitPhoneNumber".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateWaitPhoneNumberBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitPhoneNumberBuilder {
  inner: AuthorizationStateWaitPhoneNumber
}

impl RTDAuthorizationStateWaitPhoneNumberBuilder {
  pub fn build(&self) -> AuthorizationStateWaitPhoneNumber { self.inner.clone() }

}

impl AsRef<AuthorizationStateWaitPhoneNumber> for AuthorizationStateWaitPhoneNumber {
  fn as_ref(&self) -> &AuthorizationStateWaitPhoneNumber { self }
}

impl AsRef<AuthorizationStateWaitPhoneNumber> for RTDAuthorizationStateWaitPhoneNumberBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitPhoneNumber { &self.inner }
}







/// The user is unregistered and need to accept terms of service and enter their first name and last name to finish registration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitRegistration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Telegram terms of service
  terms_of_service: TermsOfService,
  
}

impl RObject for AuthorizationStateWaitRegistration {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitRegistration" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateWaitRegistration {}



impl AuthorizationStateWaitRegistration {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateWaitRegistrationBuilder {
    let mut inner = AuthorizationStateWaitRegistration::default();
    inner.td_name = "authorizationStateWaitRegistration".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateWaitRegistrationBuilder { inner }
  }

  pub fn terms_of_service(&self) -> &TermsOfService { &self.terms_of_service }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitRegistrationBuilder {
  inner: AuthorizationStateWaitRegistration
}

impl RTDAuthorizationStateWaitRegistrationBuilder {
  pub fn build(&self) -> AuthorizationStateWaitRegistration { self.inner.clone() }

   
  pub fn terms_of_service<T: AsRef<TermsOfService>>(&mut self, terms_of_service: T) -> &mut Self {
    self.inner.terms_of_service = terms_of_service.as_ref().clone();
    self
  }

}

impl AsRef<AuthorizationStateWaitRegistration> for AuthorizationStateWaitRegistration {
  fn as_ref(&self) -> &AuthorizationStateWaitRegistration { self }
}

impl AsRef<AuthorizationStateWaitRegistration> for RTDAuthorizationStateWaitRegistrationBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitRegistration { &self.inner }
}







/// TDLib needs TdlibParameters for initialization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitTdlibParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for AuthorizationStateWaitTdlibParameters {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitTdlibParameters" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateWaitTdlibParameters {}



impl AuthorizationStateWaitTdlibParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateWaitTdlibParametersBuilder {
    let mut inner = AuthorizationStateWaitTdlibParameters::default();
    inner.td_name = "authorizationStateWaitTdlibParameters".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateWaitTdlibParametersBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitTdlibParametersBuilder {
  inner: AuthorizationStateWaitTdlibParameters
}

impl RTDAuthorizationStateWaitTdlibParametersBuilder {
  pub fn build(&self) -> AuthorizationStateWaitTdlibParameters { self.inner.clone() }

}

impl AsRef<AuthorizationStateWaitTdlibParameters> for AuthorizationStateWaitTdlibParameters {
  fn as_ref(&self) -> &AuthorizationStateWaitTdlibParameters { self }
}

impl AsRef<AuthorizationStateWaitTdlibParameters> for RTDAuthorizationStateWaitTdlibParametersBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitTdlibParameters { &self.inner }
}



