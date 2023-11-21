use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a limit, increased for Premium users. Returns a 404 error if the limit is unknown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPremiumLimit {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the limit

    #[serde(skip_serializing_if = "PremiumLimitType::_is_default")]
    limit_type: PremiumLimitType,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPremiumLimit {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPremiumLimit {}

impl GetPremiumLimit {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetPremiumLimitBuilder {
        let mut inner = GetPremiumLimit::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPremiumLimit".to_string();

        GetPremiumLimitBuilder { inner }
    }

    pub fn limit_type(&self) -> &PremiumLimitType {
        &self.limit_type
    }
}

#[doc(hidden)]
pub struct GetPremiumLimitBuilder {
    inner: GetPremiumLimit,
}

#[deprecated]
pub type RTDGetPremiumLimitBuilder = GetPremiumLimitBuilder;

impl GetPremiumLimitBuilder {
    pub fn build(&self) -> GetPremiumLimit {
        self.inner.clone()
    }

    pub fn limit_type<T: AsRef<PremiumLimitType>>(&mut self, limit_type: T) -> &mut Self {
        self.inner.limit_type = limit_type.as_ref().clone();
        self
    }
}

impl AsRef<GetPremiumLimit> for GetPremiumLimit {
    fn as_ref(&self) -> &GetPremiumLimit {
        self
    }
}

impl AsRef<GetPremiumLimit> for GetPremiumLimitBuilder {
    fn as_ref(&self) -> &GetPremiumLimit {
        &self.inner
    }
}
