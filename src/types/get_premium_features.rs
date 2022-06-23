use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about features, available to Premium users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPremiumFeatures {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Source of the request; pass null if the method is called from some non-standard source

    #[serde(skip_serializing_if = "PremiumSource::_is_default")]
    source: PremiumSource,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPremiumFeatures {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPremiumFeatures {}

impl GetPremiumFeatures {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetPremiumFeaturesBuilder {
        let mut inner = GetPremiumFeatures::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPremiumFeatures".to_string();

        RTDGetPremiumFeaturesBuilder { inner }
    }

    pub fn source(&self) -> &PremiumSource {
        &self.source
    }
}

#[doc(hidden)]
pub struct RTDGetPremiumFeaturesBuilder {
    inner: GetPremiumFeatures,
}

impl RTDGetPremiumFeaturesBuilder {
    pub fn build(&self) -> GetPremiumFeatures {
        self.inner.clone()
    }

    pub fn source<T: AsRef<PremiumSource>>(&mut self, source: T) -> &mut Self {
        self.inner.source = source.as_ref().clone();
        self
    }
}

impl AsRef<GetPremiumFeatures> for GetPremiumFeatures {
    fn as_ref(&self) -> &GetPremiumFeatures {
        self
    }
}

impl AsRef<GetPremiumFeatures> for RTDGetPremiumFeaturesBuilder {
    fn as_ref(&self) -> &GetPremiumFeatures {
        &self.inner
    }
}
