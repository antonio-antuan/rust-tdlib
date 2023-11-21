use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes an option for creating Telegram Premium gift codes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiftCodePaymentOption {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// ISO 4217 currency code for Telegram Premium gift code payment

    #[serde(default)]
    currency: String,
    /// The amount to pay, in the smallest units of the currency

    #[serde(default)]
    amount: i64,
    /// Number of users which will be able to activate the gift codes

    #[serde(default)]
    user_count: i32,
    /// Number of month the Telegram Premium subscription will be active

    #[serde(default)]
    month_count: i32,
    /// Identifier of the store product associated with the option; may be empty if none

    #[serde(default)]
    store_product_id: String,
    /// Number of times the store product must be paid

    #[serde(default)]
    store_product_quantity: i32,
}

impl RObject for PremiumGiftCodePaymentOption {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PremiumGiftCodePaymentOption {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumGiftCodePaymentOptionBuilder {
        let mut inner = PremiumGiftCodePaymentOption::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumGiftCodePaymentOptionBuilder { inner }
    }

    pub fn currency(&self) -> &String {
        &self.currency
    }

    pub fn amount(&self) -> i64 {
        self.amount
    }

    pub fn user_count(&self) -> i32 {
        self.user_count
    }

    pub fn month_count(&self) -> i32 {
        self.month_count
    }

    pub fn store_product_id(&self) -> &String {
        &self.store_product_id
    }

    pub fn store_product_quantity(&self) -> i32 {
        self.store_product_quantity
    }
}

#[doc(hidden)]
pub struct PremiumGiftCodePaymentOptionBuilder {
    inner: PremiumGiftCodePaymentOption,
}

#[deprecated]
pub type RTDPremiumGiftCodePaymentOptionBuilder = PremiumGiftCodePaymentOptionBuilder;

impl PremiumGiftCodePaymentOptionBuilder {
    pub fn build(&self) -> PremiumGiftCodePaymentOption {
        self.inner.clone()
    }

    pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
        self.inner.currency = currency.as_ref().to_string();
        self
    }

    pub fn amount(&mut self, amount: i64) -> &mut Self {
        self.inner.amount = amount;
        self
    }

    pub fn user_count(&mut self, user_count: i32) -> &mut Self {
        self.inner.user_count = user_count;
        self
    }

    pub fn month_count(&mut self, month_count: i32) -> &mut Self {
        self.inner.month_count = month_count;
        self
    }

    pub fn store_product_id<T: AsRef<str>>(&mut self, store_product_id: T) -> &mut Self {
        self.inner.store_product_id = store_product_id.as_ref().to_string();
        self
    }

    pub fn store_product_quantity(&mut self, store_product_quantity: i32) -> &mut Self {
        self.inner.store_product_quantity = store_product_quantity;
        self
    }
}

impl AsRef<PremiumGiftCodePaymentOption> for PremiumGiftCodePaymentOption {
    fn as_ref(&self) -> &PremiumGiftCodePaymentOption {
        self
    }
}

impl AsRef<PremiumGiftCodePaymentOption> for PremiumGiftCodePaymentOptionBuilder {
    fn as_ref(&self) -> &PremiumGiftCodePaymentOption {
        &self.inner
    }
}
