use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains settings for the authentication of the user's phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhoneNumberAuthenticationSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pass true if the authentication code may be sent via flash call to the specified phone number
    allow_flash_call: bool,
    /// Pass true if the authenticated phone number is used on the current device
    is_current_phone_number: bool,
    /// For official applications only. True, if the application can use Android SMS Retriever API (requires Google Play Services >= 10.2) to automatically receive the authentication code from the SMS. See https://developers.google.com/identity/sms-retriever/ for more details
    allow_sms_retriever_api: bool,
}

impl RObject for PhoneNumberAuthenticationSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PhoneNumberAuthenticationSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPhoneNumberAuthenticationSettingsBuilder {
        let mut inner = PhoneNumberAuthenticationSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPhoneNumberAuthenticationSettingsBuilder { inner }
    }

    pub fn allow_flash_call(&self) -> bool {
        self.allow_flash_call
    }

    pub fn is_current_phone_number(&self) -> bool {
        self.is_current_phone_number
    }

    pub fn allow_sms_retriever_api(&self) -> bool {
        self.allow_sms_retriever_api
    }
}

#[doc(hidden)]
pub struct RTDPhoneNumberAuthenticationSettingsBuilder {
    inner: PhoneNumberAuthenticationSettings,
}

impl RTDPhoneNumberAuthenticationSettingsBuilder {
    pub fn build(&self) -> PhoneNumberAuthenticationSettings {
        self.inner.clone()
    }

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
    fn as_ref(&self) -> &PhoneNumberAuthenticationSettings {
        self
    }
}

impl AsRef<PhoneNumberAuthenticationSettings> for RTDPhoneNumberAuthenticationSettingsBuilder {
    fn as_ref(&self) -> &PhoneNumberAuthenticationSettings {
        &self.inner
    }
}
