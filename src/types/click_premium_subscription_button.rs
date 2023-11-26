use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that the user clicked Premium subscription button on the Premium features screen
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClickPremiumSubscriptionButton {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ClickPremiumSubscriptionButton {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ClickPremiumSubscriptionButton {}

impl ClickPremiumSubscriptionButton {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ClickPremiumSubscriptionButtonBuilder {
        let mut inner = ClickPremiumSubscriptionButton::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "clickPremiumSubscriptionButton".to_string();

        ClickPremiumSubscriptionButtonBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ClickPremiumSubscriptionButtonBuilder {
    inner: ClickPremiumSubscriptionButton,
}

#[deprecated]
pub type RTDClickPremiumSubscriptionButtonBuilder = ClickPremiumSubscriptionButtonBuilder;

impl ClickPremiumSubscriptionButtonBuilder {
    pub fn build(&self) -> ClickPremiumSubscriptionButton {
        self.inner.clone()
    }
}

impl AsRef<ClickPremiumSubscriptionButton> for ClickPremiumSubscriptionButton {
    fn as_ref(&self) -> &ClickPremiumSubscriptionButton {
        self
    }
}

impl AsRef<ClickPremiumSubscriptionButton> for ClickPremiumSubscriptionButtonBuilder {
    fn as_ref(&self) -> &ClickPremiumSubscriptionButton {
        &self.inner
    }
}
