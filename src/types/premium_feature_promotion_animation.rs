use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a promotion animation for a Premium feature
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeaturePromotionAnimation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Premium feature

    #[serde(skip_serializing_if = "PremiumFeature::_is_default")]
    feature: PremiumFeature,
    /// Promotion animation for the feature
    animation: Animation,
}

impl RObject for PremiumFeaturePromotionAnimation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PremiumFeaturePromotionAnimation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeaturePromotionAnimationBuilder {
        let mut inner = PremiumFeaturePromotionAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeaturePromotionAnimationBuilder { inner }
    }

    pub fn feature(&self) -> &PremiumFeature {
        &self.feature
    }

    pub fn animation(&self) -> &Animation {
        &self.animation
    }
}

#[doc(hidden)]
pub struct PremiumFeaturePromotionAnimationBuilder {
    inner: PremiumFeaturePromotionAnimation,
}

#[deprecated]
pub type RTDPremiumFeaturePromotionAnimationBuilder = PremiumFeaturePromotionAnimationBuilder;

impl PremiumFeaturePromotionAnimationBuilder {
    pub fn build(&self) -> PremiumFeaturePromotionAnimation {
        self.inner.clone()
    }

    pub fn feature<T: AsRef<PremiumFeature>>(&mut self, feature: T) -> &mut Self {
        self.inner.feature = feature.as_ref().clone();
        self
    }

    pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
        self.inner.animation = animation.as_ref().clone();
        self
    }
}

impl AsRef<PremiumFeaturePromotionAnimation> for PremiumFeaturePromotionAnimation {
    fn as_ref(&self) -> &PremiumFeaturePromotionAnimation {
        self
    }
}

impl AsRef<PremiumFeaturePromotionAnimation> for PremiumFeaturePromotionAnimationBuilder {
    fn as_ref(&self) -> &PremiumFeaturePromotionAnimation {
        &self.inner
    }
}
