use crate::errors::*;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetCountriesBuilder {
        let mut inner = GetCountries::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getCountries".to_string();

        RTDGetCountriesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetCountriesBuilder {
    inner: GetCountries,
}

impl RTDGetCountriesBuilder {
    pub fn build(&self) -> GetCountries {
        self.inner.clone()
    }
}

impl AsRef<GetCountries> for GetCountries {
    fn as_ref(&self) -> &GetCountries {
        self
    }
}

impl AsRef<GetCountries> for RTDGetCountriesBuilder {
    fn as_ref(&self) -> &GetCountries {
        &self.inner
    }
}
