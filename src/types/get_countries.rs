use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about existing countries. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCountries {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetCountries {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetCountries {}

impl GetCountries {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetCountriesBuilder {
        let mut inner = GetCountries::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getCountries".to_string();

        GetCountriesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetCountriesBuilder {
    inner: GetCountries,
}

#[deprecated]
pub type RTDGetCountriesBuilder = GetCountriesBuilder;

impl GetCountriesBuilder {
    pub fn build(&self) -> GetCountries {
        self.inner.clone()
    }
}

impl AsRef<GetCountries> for GetCountries {
    fn as_ref(&self) -> &GetCountries {
        self
    }
}

impl AsRef<GetCountries> for GetCountriesBuilder {
    fn as_ref(&self) -> &GetCountries {
        &self.inner
    }
}
