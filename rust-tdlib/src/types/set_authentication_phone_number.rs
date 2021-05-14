use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sets the phone number of the user and sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitPhoneNumber, or if there is no pending authentication query and the current authorization state is authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAuthenticationPhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The phone number of the user, in international format
    phone_number: String,
    /// Settings for the authentication of the user's phone number
    settings: PhoneNumberAuthenticationSettings,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetAuthenticationPhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetAuthenticationPhoneNumber {}

impl SetAuthenticationPhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetAuthenticationPhoneNumberBuilder {
        let mut inner = SetAuthenticationPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setAuthenticationPhoneNumber".to_string();

        RTDSetAuthenticationPhoneNumberBuilder { inner }
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }

    pub fn settings(&self) -> &PhoneNumberAuthenticationSettings {
        &self.settings
    }
}

#[doc(hidden)]
pub struct RTDSetAuthenticationPhoneNumberBuilder {
    inner: SetAuthenticationPhoneNumber,
}

impl RTDSetAuthenticationPhoneNumberBuilder {
    pub fn build(&self) -> SetAuthenticationPhoneNumber {
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

impl AsRef<SetAuthenticationPhoneNumber> for SetAuthenticationPhoneNumber {
    fn as_ref(&self) -> &SetAuthenticationPhoneNumber {
        self
    }
}

impl AsRef<SetAuthenticationPhoneNumber> for RTDSetAuthenticationPhoneNumberBuilder {
    fn as_ref(&self) -> &SetAuthenticationPhoneNumber {
        &self.inner
    }
}
