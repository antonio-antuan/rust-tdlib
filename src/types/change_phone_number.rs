use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the phone number of the user and sends an authentication code to the user's new phone number. On success, returns information about the sent code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangePhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new phone number of the user in international format

    #[serde(default)]
    phone_number: String,
    /// Settings for the authentication of the user's phone number; pass null to use default settings
    settings: PhoneNumberAuthenticationSettings,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ChangePhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ChangePhoneNumber {}

impl ChangePhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChangePhoneNumberBuilder {
        let mut inner = ChangePhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "changePhoneNumber".to_string();

        ChangePhoneNumberBuilder { inner }
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }

    pub fn settings(&self) -> &PhoneNumberAuthenticationSettings {
        &self.settings
    }
}

#[doc(hidden)]
pub struct ChangePhoneNumberBuilder {
    inner: ChangePhoneNumber,
}

#[deprecated]
pub type RTDChangePhoneNumberBuilder = ChangePhoneNumberBuilder;

impl ChangePhoneNumberBuilder {
    pub fn build(&self) -> ChangePhoneNumber {
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

impl AsRef<ChangePhoneNumber> for ChangePhoneNumber {
    fn as_ref(&self) -> &ChangePhoneNumber {
        self
    }
}

impl AsRef<ChangePhoneNumber> for ChangePhoneNumberBuilder {
    fn as_ref(&self) -> &ChangePhoneNumber {
        &self.inner
    }
}
