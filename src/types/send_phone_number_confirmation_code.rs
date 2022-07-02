use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sends phone number confirmation code to handle links of the type internalLinkTypePhoneNumberConfirmation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendPhoneNumberConfirmationCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Hash value from the link

    #[serde(default)]
    hash: String,
    /// Phone number value from the link

    #[serde(default)]
    phone_number: String,
    /// Settings for the authentication of the user's phone number; pass null to use default settings
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SendPhoneNumberConfirmationCodeBuilder {
        let mut inner = SendPhoneNumberConfirmationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendPhoneNumberConfirmationCode".to_string();

        SendPhoneNumberConfirmationCodeBuilder { inner }
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
pub struct SendPhoneNumberConfirmationCodeBuilder {
    inner: SendPhoneNumberConfirmationCode,
}

#[deprecated]
pub type RTDSendPhoneNumberConfirmationCodeBuilder = SendPhoneNumberConfirmationCodeBuilder;

impl SendPhoneNumberConfirmationCodeBuilder {
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

impl AsRef<SendPhoneNumberConfirmationCode> for SendPhoneNumberConfirmationCodeBuilder {
    fn as_ref(&self) -> &SendPhoneNumberConfirmationCode {
        &self.inner
    }
}
