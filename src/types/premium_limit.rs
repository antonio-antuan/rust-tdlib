use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a limit, increased for Premium users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimit {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The type of the limit

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "PremiumLimitType::_is_default")]
    type_: PremiumLimitType,
    /// Default value of the limit
    default_value: i32,
    /// Value of the limit for Premium users
    premium_value: i32,
}

impl RObject for PremiumLimit {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PremiumLimit {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumLimitBuilder {
        let mut inner = PremiumLimit::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumLimitBuilder { inner }
    }

    pub fn type_(&self) -> &PremiumLimitType {
        &self.type_
    }

    pub fn default_value(&self) -> i32 {
        self.default_value
    }

    pub fn premium_value(&self) -> i32 {
        self.premium_value
    }
}

#[doc(hidden)]
pub struct RTDPremiumLimitBuilder {
    inner: PremiumLimit,
}

impl RTDPremiumLimitBuilder {
    pub fn build(&self) -> PremiumLimit {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<PremiumLimitType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn default_value(&mut self, default_value: i32) -> &mut Self {
        self.inner.default_value = default_value;
        self
    }

    pub fn premium_value(&mut self, premium_value: i32) -> &mut Self {
        self.inner.premium_value = premium_value;
        self
    }
}

impl AsRef<PremiumLimit> for PremiumLimit {
    fn as_ref(&self) -> &PremiumLimit {
        self
    }
}

impl AsRef<PremiumLimit> for RTDPremiumLimitBuilder {
    fn as_ref(&self) -> &PremiumLimit {
        &self.inner
    }
}
