use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sends a code to verify a phone number to be added to a user's Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendPhoneNumberVerificationCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The phone number of the user, in international format

    #[serde(default)]
    phone_number: String,
    /// Settings for the authentication of the user's phone number; pass null to use default settings
    settings: PhoneNumberAuthenticationSettings,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendPhoneNumberVerificationCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendPhoneNumberVerificationCode {}

impl SendPhoneNumberVerificationCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SendPhoneNumberVerificationCodeBuilder {
        let mut inner = SendPhoneNumberVerificationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendPhoneNumberVerificationCode".to_string();

        SendPhoneNumberVerificationCodeBuilder { inner }
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }

    pub fn settings(&self) -> &PhoneNumberAuthenticationSettings {
        &self.settings
    }
}

#[doc(hidden)]
pub struct SendPhoneNumberVerificationCodeBuilder {
    inner: SendPhoneNumberVerificationCode,
}

#[deprecated]
pub type RTDSendPhoneNumberVerificationCodeBuilder = SendPhoneNumberVerificationCodeBuilder;

impl SendPhoneNumberVerificationCodeBuilder {
    pub fn build(&self) -> SendPhoneNumberVerificationCode {
        self.inner.clone()
    }

    pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
        self.inner.phone_number = phone_number.as_ref().to_string();
        self
    }

    pub fn settings<T: AsRef<PhoneNumberAuthenticationSettings>>(
        &mut self,
        settings: T,
    ) -> &mut Self {
        self.inner.settings = settings.as_ref().clone();
        self
    }
}

impl AsRef<SendPhoneNumberVerificationCode> for SendPhoneNumberVerificationCode {
    fn as_ref(&self) -> &SendPhoneNumberVerificationCode {
        self
    }
}

impl AsRef<SendPhoneNumberVerificationCode> for SendPhoneNumberVerificationCodeBuilder {
    fn as_ref(&self) -> &SendPhoneNumberVerificationCode {
        &self.inner
    }
}
