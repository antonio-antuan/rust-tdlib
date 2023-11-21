use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains state of Telegram Premium subscription and promotion videos for Premium features
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumState {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text description of the state of the current Premium subscription; may be empty if the current user has no Telegram Premium subscription
    state: FormattedText,
    /// The list of available options for buying Telegram Premium

    #[serde(default)]
    payment_options: Vec<PremiumStatePaymentOption>,
    /// The list of available promotion animations for Premium features

    #[serde(default)]
    animations: Vec<PremiumFeaturePromotionAnimation>,
}

impl RObject for PremiumState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PremiumState {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumStateBuilder {
        let mut inner = PremiumState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumStateBuilder { inner }
    }

    pub fn state(&self) -> &FormattedText {
        &self.state
    }

    pub fn payment_options(&self) -> &Vec<PremiumStatePaymentOption> {
        &self.payment_options
    }

    pub fn animations(&self) -> &Vec<PremiumFeaturePromotionAnimation> {
        &self.animations
    }
}

#[doc(hidden)]
pub struct PremiumStateBuilder {
    inner: PremiumState,
}

#[deprecated]
pub type RTDPremiumStateBuilder = PremiumStateBuilder;

impl PremiumStateBuilder {
    pub fn build(&self) -> PremiumState {
        self.inner.clone()
    }

    pub fn state<T: AsRef<FormattedText>>(&mut self, state: T) -> &mut Self {
        self.inner.state = state.as_ref().clone();
        self
    }

    pub fn payment_options(
        &mut self,
        payment_options: Vec<PremiumStatePaymentOption>,
    ) -> &mut Self {
        self.inner.payment_options = payment_options;
        self
    }

    pub fn animations(&mut self, animations: Vec<PremiumFeaturePromotionAnimation>) -> &mut Self {
        self.inner.animations = animations;
        self
    }
}

impl AsRef<PremiumState> for PremiumState {
    fn as_ref(&self) -> &PremiumState {
        self
    }
}

impl AsRef<PremiumState> for PremiumStateBuilder {
    fn as_ref(&self) -> &PremiumState {
        &self.inner
    }
}
