use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an IETF language tag of the language preferred in the country, which must be used to fill native fields in Telegram Passport personal details. Returns a 404 error if unknown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPreferredCountryLanguage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A two-letter ISO 3166-1 alpha-2 country code

    #[serde(default)]
    country_code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPreferredCountryLanguage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPreferredCountryLanguage {}

impl GetPreferredCountryLanguage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetPreferredCountryLanguageBuilder {
        let mut inner = GetPreferredCountryLanguage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPreferredCountryLanguage".to_string();

        GetPreferredCountryLanguageBuilder { inner }
    }

    pub fn country_code(&self) -> &String {
        &self.country_code
    }
}

#[doc(hidden)]
pub struct GetPreferredCountryLanguageBuilder {
    inner: GetPreferredCountryLanguage,
}

#[deprecated]
pub type RTDGetPreferredCountryLanguageBuilder = GetPreferredCountryLanguageBuilder;

impl GetPreferredCountryLanguageBuilder {
    pub fn build(&self) -> GetPreferredCountryLanguage {
        self.inner.clone()
    }

    pub fn country_code<T: AsRef<str>>(&mut self, country_code: T) -> &mut Self {
        self.inner.country_code = country_code.as_ref().to_string();
        self
    }
}

impl AsRef<GetPreferredCountryLanguage> for GetPreferredCountryLanguage {
    fn as_ref(&self) -> &GetPreferredCountryLanguage {
        self
    }
}

impl AsRef<GetPreferredCountryLanguage> for GetPreferredCountryLanguageBuilder {
    fn as_ref(&self) -> &GetPreferredCountryLanguage {
        &self.inner
    }
}
