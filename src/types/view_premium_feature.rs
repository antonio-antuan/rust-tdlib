use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that the user viewed detailed information about a Premium feature on the Premium features screen
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ViewPremiumFeature {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The viewed premium feature

    #[serde(skip_serializing_if = "PremiumFeature::_is_default")]
    feature: PremiumFeature,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ViewPremiumFeature {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ViewPremiumFeature {}

impl ViewPremiumFeature {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDViewPremiumFeatureBuilder {
        let mut inner = ViewPremiumFeature::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "viewPremiumFeature".to_string();

        RTDViewPremiumFeatureBuilder { inner }
    }

    pub fn feature(&self) -> &PremiumFeature {
        &self.feature
    }
}

#[doc(hidden)]
pub struct RTDViewPremiumFeatureBuilder {
    inner: ViewPremiumFeature,
}

impl RTDViewPremiumFeatureBuilder {
    pub fn build(&self) -> ViewPremiumFeature {
        self.inner.clone()
    }

    pub fn feature<T: AsRef<PremiumFeature>>(&mut self, feature: T) -> &mut Self {
        self.inner.feature = feature.as_ref().clone();
        self
    }
}

impl AsRef<ViewPremiumFeature> for ViewPremiumFeature {
    fn as_ref(&self) -> &ViewPremiumFeature {
        self
    }
}

impl AsRef<ViewPremiumFeature> for RTDViewPremiumFeatureBuilder {
    fn as_ref(&self) -> &ViewPremiumFeature {
        &self.inner
    }
}
