use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about features, available to Premium users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatures {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The list of available features
    features: Vec<PremiumFeature>,
    /// The list of limits, increased for Premium users
    limits: Vec<PremiumLimit>,
    /// An internal link to be opened to pay for Telegram Premium if store payment isn't possible; may be null if direct payment isn't available. If the link has type internalLinkTypeBotStart, then sendBotStartMessage must be called automatically
    payment_link: Option<InternalLinkType>,
}

impl RObject for PremiumFeatures {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PremiumFeatures {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumFeaturesBuilder {
        let mut inner = PremiumFeatures::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumFeaturesBuilder { inner }
    }

    pub fn features(&self) -> &Vec<PremiumFeature> {
        &self.features
    }

    pub fn limits(&self) -> &Vec<PremiumLimit> {
        &self.limits
    }

    pub fn payment_link(&self) -> &Option<InternalLinkType> {
        &self.payment_link
    }
}

#[doc(hidden)]
pub struct RTDPremiumFeaturesBuilder {
    inner: PremiumFeatures,
}

impl RTDPremiumFeaturesBuilder {
    pub fn build(&self) -> PremiumFeatures {
        self.inner.clone()
    }

    pub fn features(&mut self, features: Vec<PremiumFeature>) -> &mut Self {
        self.inner.features = features;
        self
    }

    pub fn limits(&mut self, limits: Vec<PremiumLimit>) -> &mut Self {
        self.inner.limits = limits;
        self
    }

    pub fn payment_link<T: AsRef<InternalLinkType>>(&mut self, payment_link: T) -> &mut Self {
        self.inner.payment_link = Some(payment_link.as_ref().clone());
        self
    }
}

impl AsRef<PremiumFeatures> for PremiumFeatures {
    fn as_ref(&self) -> &PremiumFeatures {
        self
    }
}

impl AsRef<PremiumFeatures> for RTDPremiumFeaturesBuilder {
    fn as_ref(&self) -> &PremiumFeatures {
        &self.inner
    }
}
