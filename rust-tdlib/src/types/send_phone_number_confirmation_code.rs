use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends phone number confirmation code. Should be called when user presses "https://t.me/confirmphone?phone=*******&hash=**********" or "tg://confirmphone?phone=*******&hash=**********" link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendPhoneNumberConfirmationCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Value of the "hash" parameter from the link
    hash: String,
    /// Value of the "phone" parameter from the link
    phone_number: String,
    /// Settings for the authentication of the user's phone number
    settings: PhoneNumberAuthenticationSettings,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendPhoneNumberConfirmationCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendPhoneNumberConfirmationCode {}

impl SendPhoneNumberConfirmationCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendPhoneNumberConfirmationCodeBuilder {
        let mut inner = SendPhoneNumberConfirmationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendPhoneNumberConfirmationCode".to_string();

        RTDSendPhoneNumberConfirmationCodeBuilder { inner }
    }

    pub fn hash(&self) -> &String {
        &self.hash
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }

    pub fn settings(&self) -> &PhoneNumberAuthenticationSettings {
        &self.settings
    }
}

#[doc(hidden)]
pub struct RTDSendPhoneNumberConfirmationCodeBuilder {
    inner: SendPhoneNumberConfirmationCode,
}

impl RTDSendPhoneNumberConfirmationCodeBuilder {
    pub fn build(&self) -> SendPhoneNumberConfirmationCode {
        self.inner.clone()
    }

    pub fn hash<T: AsRef<str>>(&mut self, hash: T) -> &mut Self {
        self.inner.hash = hash.as_ref().to_string();
        self
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

impl AsRef<SendPhoneNumberConfirmationCode> for SendPhoneNumberConfirmationCode {
    fn as_ref(&self) -> &SendPhoneNumberConfirmationCode {
        self
    }
}

impl AsRef<SendPhoneNumberConfirmationCode> for RTDSendPhoneNumberConfirmationCodeBuilder {
    fn as_ref(&self) -> &SendPhoneNumberConfirmationCode {
        &self.inner
    }
}
