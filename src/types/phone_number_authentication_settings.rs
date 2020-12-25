
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains settings for the authentication of the user's phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhoneNumberAuthenticationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Pass true if the authentication code may be sent via flash call to the specified phone number
  allow_flash_call: bool,
  /// Pass true if the authenticated phone number is used on the current device
  is_current_phone_number: bool,
  /// For official applications only. True, if the app can use Android SMS Retriever API (requires Google Play Services >= 10.2) to automatically receive the authentication code from the SMS. See https://developers.google.com/identity/sms-retriever/ for more details
  allow_sms_retriever_api: bool,
  
}

impl RObject for PhoneNumberAuthenticationSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "phoneNumberAuthenticationSettings" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PhoneNumberAuthenticationSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPhoneNumberAuthenticationSettingsBuilder {
    let mut inner = PhoneNumberAuthenticationSettings::default();
    inner.td_name = "phoneNumberAuthenticationSettings".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPhoneNumberAuthenticationSettingsBuilder { inner }
  }

  pub fn allow_flash_call(&self) -> bool { self.allow_flash_call }

  pub fn is_current_phone_number(&self) -> bool { self.is_current_phone_number }

  pub fn allow_sms_retriever_api(&self) -> bool { self.allow_sms_retriever_api }

}

#[doc(hidden)]
pub struct RTDPhoneNumberAuthenticationSettingsBuilder {
  inner: PhoneNumberAuthenticationSettings
}

impl RTDPhoneNumberAuthenticationSettingsBuilder {
  pub fn build(&self) -> PhoneNumberAuthenticationSettings { self.inner.clone() }

   
  pub fn allow_flash_call(&mut self, allow_flash_call: bool) -> &mut Self {
    self.inner.allow_flash_call = allow_flash_call;
    self
  }

   
  pub fn is_current_phone_number(&mut self, is_current_phone_number: bool) -> &mut Self {
    self.inner.is_current_phone_number = is_current_phone_number;
    self
  }

   
  pub fn allow_sms_retriever_api(&mut self, allow_sms_retriever_api: bool) -> &mut Self {
    self.inner.allow_sms_retriever_api = allow_sms_retriever_api;
    self
  }

}

impl AsRef<PhoneNumberAuthenticationSettings> for PhoneNumberAuthenticationSettings {
  fn as_ref(&self) -> &PhoneNumberAuthenticationSettings { self }
}

impl AsRef<PhoneNumberAuthenticationSettings> for RTDPhoneNumberAuthenticationSettingsBuilder {
  fn as_ref(&self) -> &PhoneNumberAuthenticationSettings { &self.inner }
}



