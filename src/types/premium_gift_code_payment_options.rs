use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of options for creating Telegram Premium gift codes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiftCodePaymentOptions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The list of options

    #[serde(default)]
    options: Vec<PremiumGiftCodePaymentOption>,
}

impl RObject for PremiumGiftCodePaymentOptions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PremiumGiftCodePaymentOptions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumGiftCodePaymentOptionsBuilder {
        let mut inner = PremiumGiftCodePaymentOptions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumGiftCodePaymentOptionsBuilder { inner }
    }

    pub fn options(&self) -> &Vec<PremiumGiftCodePaymentOption> {
        &self.options
    }
}

#[doc(hidden)]
pub struct PremiumGiftCodePaymentOptionsBuilder {
    inner: PremiumGiftCodePaymentOptions,
}

#[deprecated]
pub type RTDPremiumGiftCodePaymentOptionsBuilder = PremiumGiftCodePaymentOptionsBuilder;

impl PremiumGiftCodePaymentOptionsBuilder {
    pub fn build(&self) -> PremiumGiftCodePaymentOptions {
        self.inner.clone()
    }

    pub fn options(&mut self, options: Vec<PremiumGiftCodePaymentOption>) -> &mut Self {
        self.inner.options = options;
        self
    }
}

impl AsRef<PremiumGiftCodePaymentOptions> for PremiumGiftCodePaymentOptions {
    fn as_ref(&self) -> &PremiumGiftCodePaymentOptions {
        self
    }
}

impl AsRef<PremiumGiftCodePaymentOptions> for PremiumGiftCodePaymentOptionsBuilder {
    fn as_ref(&self) -> &PremiumGiftCodePaymentOptions {
        &self.inner
    }
}
