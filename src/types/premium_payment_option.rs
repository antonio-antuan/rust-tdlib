use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes an option for buying Telegram Premium to a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumPaymentOption {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// ISO 4217 currency code for Telegram Premium subscription payment

    #[serde(default)]
    currency: String,
    /// The amount to pay, in the smallest units of the currency

    #[serde(default)]
    amount: i64,
    /// The discount associated with this option, as a percentage

    #[serde(default)]
    discount_percentage: i32,
    /// Number of month the Telegram Premium subscription will be active

    #[serde(default)]
    month_count: i32,
    /// Identifier of the store product associated with the option

    #[serde(default)]
    store_product_id: String,
    /// An internal link to be opened for buying Telegram Premium to the user if store payment isn't possible; may be null if direct payment isn't available
    payment_link: Option<InternalLinkType>,
}

impl RObject for PremiumPaymentOption {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PremiumPaymentOption {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumPaymentOptionBuilder {
        let mut inner = PremiumPaymentOption::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumPaymentOptionBuilder { inner }
    }

    pub fn currency(&self) -> &String {
        &self.currency
    }

    pub fn amount(&self) -> i64 {
        self.amount
    }

    pub fn discount_percentage(&self) -> i32 {
        self.discount_percentage
    }

    pub fn month_count(&self) -> i32 {
        self.month_count
    }

    pub fn store_product_id(&self) -> &String {
        &self.store_product_id
    }

    pub fn payment_link(&self) -> &Option<InternalLinkType> {
        &self.payment_link
    }
}

#[doc(hidden)]
pub struct PremiumPaymentOptionBuilder {
    inner: PremiumPaymentOption,
}

#[deprecated]
pub type RTDPremiumPaymentOptionBuilder = PremiumPaymentOptionBuilder;

impl PremiumPaymentOptionBuilder {
    pub fn build(&self) -> PremiumPaymentOption {
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

    pub fn discount_percentage(&mut self, discount_percentage: i32) -> &mut Self {
        self.inner.discount_percentage = discount_percentage;
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

    pub fn payment_link<T: AsRef<InternalLinkType>>(&mut self, payment_link: T) -> &mut Self {
        self.inner.payment_link = Some(payment_link.as_ref().clone());
        self
    }
}

impl AsRef<PremiumPaymentOption> for PremiumPaymentOption {
    fn as_ref(&self) -> &PremiumPaymentOption {
        self
    }
}

impl AsRef<PremiumPaymentOption> for PremiumPaymentOptionBuilder {
    fn as_ref(&self) -> &PremiumPaymentOption {
        &self.inner
    }
}
