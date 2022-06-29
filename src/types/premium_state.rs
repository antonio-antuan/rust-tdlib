use crate::errors::*;
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
    /// ISO 4217 currency code for Telegram Premium subscription payment

    #[serde(default)]
    currency: String,
    /// Monthly subscription payment for Telegram Premium subscription, in the smallest units of the currency

    #[serde(default)]
    monthly_amount: i64,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumStateBuilder {
        let mut inner = PremiumState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumStateBuilder { inner }
    }

    pub fn state(&self) -> &FormattedText {
        &self.state
    }

    pub fn currency(&self) -> &String {
        &self.currency
    }

    pub fn monthly_amount(&self) -> i64 {
        self.monthly_amount
    }

    pub fn animations(&self) -> &Vec<PremiumFeaturePromotionAnimation> {
        &self.animations
    }
}

#[doc(hidden)]
pub struct RTDPremiumStateBuilder {
    inner: PremiumState,
}

impl RTDPremiumStateBuilder {
    pub fn build(&self) -> PremiumState {
        self.inner.clone()
    }

    pub fn state<T: AsRef<FormattedText>>(&mut self, state: T) -> &mut Self {
        self.inner.state = state.as_ref().clone();
        self
    }

    pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
        self.inner.currency = currency.as_ref().to_string();
        self
    }

    pub fn monthly_amount(&mut self, monthly_amount: i64) -> &mut Self {
        self.inner.monthly_amount = monthly_amount;
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

impl AsRef<PremiumState> for RTDPremiumStateBuilder {
    fn as_ref(&self) -> &PremiumState {
        &self.inner
    }
}
