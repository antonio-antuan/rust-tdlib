use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about countries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Countries {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// The list of countries
    countries: Vec<CountryInfo>,
}

impl RObject for Countries {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "countries"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl Countries {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCountriesBuilder {
        let mut inner = Countries::default();
        inner.td_name = "countries".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCountriesBuilder { inner }
    }

    pub fn countries(&self) -> &Vec<CountryInfo> {
        &self.countries
    }
}

#[doc(hidden)]
pub struct RTDCountriesBuilder {
    inner: Countries,
}

impl RTDCountriesBuilder {
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

impl AsRef<Countries> for RTDCountriesBuilder {
    fn as_ref(&self) -> &Countries {
        &self.inner
    }
}
