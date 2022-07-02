use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a phone number by its prefix synchronously. getCountries must be called at least once after changing localization to the specified language if properly localized country information is expected. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPhoneNumberInfoSync {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A two-letter ISO 639-1 country code for country information localization

    #[serde(default)]
    language_code: String,
    /// The phone number prefix

    #[serde(default)]
    phone_number_prefix: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPhoneNumberInfoSync {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPhoneNumberInfoSync {}

impl GetPhoneNumberInfoSync {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetPhoneNumberInfoSyncBuilder {
        let mut inner = GetPhoneNumberInfoSync::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPhoneNumberInfoSync".to_string();

        GetPhoneNumberInfoSyncBuilder { inner }
    }

    pub fn language_code(&self) -> &String {
        &self.language_code
    }

    pub fn phone_number_prefix(&self) -> &String {
        &self.phone_number_prefix
    }
}

#[doc(hidden)]
pub struct GetPhoneNumberInfoSyncBuilder {
    inner: GetPhoneNumberInfoSync,
}

#[deprecated]
pub type RTDGetPhoneNumberInfoSyncBuilder = GetPhoneNumberInfoSyncBuilder;

impl GetPhoneNumberInfoSyncBuilder {
    pub fn build(&self) -> GetPhoneNumberInfoSync {
        self.inner.clone()
    }

    pub fn language_code<T: AsRef<str>>(&mut self, language_code: T) -> &mut Self {
        self.inner.language_code = language_code.as_ref().to_string();
        self
    }

    pub fn phone_number_prefix<T: AsRef<str>>(&mut self, phone_number_prefix: T) -> &mut Self {
        self.inner.phone_number_prefix = phone_number_prefix.as_ref().to_string();
        self
    }
}

impl AsRef<GetPhoneNumberInfoSync> for GetPhoneNumberInfoSync {
    fn as_ref(&self) -> &GetPhoneNumberInfoSync {
        self
    }
}

impl AsRef<GetPhoneNumberInfoSync> for GetPhoneNumberInfoSyncBuilder {
    fn as_ref(&self) -> &GetPhoneNumberInfoSync {
        &self.inner
    }
}
