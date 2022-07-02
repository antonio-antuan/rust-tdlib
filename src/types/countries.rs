use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about countries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Countries {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The list of countries

    #[serde(default)]
    countries: Vec<CountryInfo>,
}

impl RObject for Countries {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Countries {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CountriesBuilder {
        let mut inner = Countries::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CountriesBuilder { inner }
    }

    pub fn countries(&self) -> &Vec<CountryInfo> {
        &self.countries
    }
}

#[doc(hidden)]
pub struct CountriesBuilder {
    inner: Countries,
}

#[deprecated]
pub type RTDCountriesBuilder = CountriesBuilder;

impl CountriesBuilder {
    pub fn build(&self) -> Countries {
        self.inner.clone()
    }

    pub fn countries(&mut self, countries: Vec<CountryInfo>) -> &mut Self {
        self.inner.countries = countries;
        self
    }
}

impl AsRef<Countries> for Countries {
    fn as_ref(&self) -> &Countries {
        self
    }
}

impl AsRef<Countries> for CountriesBuilder {
    fn as_ref(&self) -> &Countries {
        &self.inner
    }
}
