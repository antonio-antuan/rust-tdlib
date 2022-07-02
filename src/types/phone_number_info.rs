use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhoneNumberInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Information about the country to which the phone number belongs; may be null
    country: Option<CountryInfo>,
    /// The part of the phone number denoting country calling code or its part

    #[serde(default)]
    country_calling_code: String,
    /// The phone number without country calling code formatted accordingly to local rules. Expected digits are returned as '-', but even more digits might be entered by the user

    #[serde(default)]
    formatted_phone_number: String,
}

impl RObject for PhoneNumberInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PhoneNumberInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PhoneNumberInfoBuilder {
        let mut inner = PhoneNumberInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PhoneNumberInfoBuilder { inner }
    }

    pub fn country(&self) -> &Option<CountryInfo> {
        &self.country
    }

    pub fn country_calling_code(&self) -> &String {
        &self.country_calling_code
    }

    pub fn formatted_phone_number(&self) -> &String {
        &self.formatted_phone_number
    }
}

#[doc(hidden)]
pub struct PhoneNumberInfoBuilder {
    inner: PhoneNumberInfo,
}

#[deprecated]
pub type RTDPhoneNumberInfoBuilder = PhoneNumberInfoBuilder;

impl PhoneNumberInfoBuilder {
    pub fn build(&self) -> PhoneNumberInfo {
        self.inner.clone()
    }

    pub fn country<T: AsRef<CountryInfo>>(&mut self, country: T) -> &mut Self {
        self.inner.country = Some(country.as_ref().clone());
        self
    }

    pub fn country_calling_code<T: AsRef<str>>(&mut self, country_calling_code: T) -> &mut Self {
        self.inner.country_calling_code = country_calling_code.as_ref().to_string();
        self
    }

    pub fn formatted_phone_number<T: AsRef<str>>(
        &mut self,
        formatted_phone_number: T,
    ) -> &mut Self {
        self.inner.formatted_phone_number = formatted_phone_number.as_ref().to_string();
        self
    }
}

impl AsRef<PhoneNumberInfo> for PhoneNumberInfo {
    fn as_ref(&self) -> &PhoneNumberInfo {
        self
    }
}

impl AsRef<PhoneNumberInfo> for PhoneNumberInfoBuilder {
    fn as_ref(&self) -> &PhoneNumberInfo {
        &self.inner
    }
}
