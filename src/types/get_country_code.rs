use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Uses the current IP address to find the current country. Returns two-letter ISO 3166-1 alpha-2 country code. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCountryCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetCountryCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetCountryCode {}

impl GetCountryCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetCountryCodeBuilder {
        let mut inner = GetCountryCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getCountryCode".to_string();

        GetCountryCodeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetCountryCodeBuilder {
    inner: GetCountryCode,
}

#[deprecated]
pub type RTDGetCountryCodeBuilder = GetCountryCodeBuilder;

impl GetCountryCodeBuilder {
    pub fn build(&self) -> GetCountryCode {
        self.inner.clone()
    }
}

impl AsRef<GetCountryCode> for GetCountryCode {
    fn as_ref(&self) -> &GetCountryCode {
        self
    }
}

impl AsRef<GetCountryCode> for GetCountryCodeBuilder {
    fn as_ref(&self) -> &GetCountryCode {
        &self.inner
    }
}
