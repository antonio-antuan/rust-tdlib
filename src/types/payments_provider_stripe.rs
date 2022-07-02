use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Stripe payment provider
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentsProviderStripe {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Stripe API publishable key

    #[serde(default)]
    publishable_key: String,
    /// True, if the user country must be provided

    #[serde(default)]
    need_country: bool,
    /// True, if the user ZIP/postal code must be provided

    #[serde(default)]
    need_postal_code: bool,
    /// True, if the cardholder name must be provided

    #[serde(default)]
    need_cardholder_name: bool,
}

impl RObject for PaymentsProviderStripe {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PaymentsProviderStripe {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PaymentsProviderStripeBuilder {
        let mut inner = PaymentsProviderStripe::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PaymentsProviderStripeBuilder { inner }
    }

    pub fn publishable_key(&self) -> &String {
        &self.publishable_key
    }

    pub fn need_country(&self) -> bool {
        self.need_country
    }

    pub fn need_postal_code(&self) -> bool {
        self.need_postal_code
    }

    pub fn need_cardholder_name(&self) -> bool {
        self.need_cardholder_name
    }
}

#[doc(hidden)]
pub struct PaymentsProviderStripeBuilder {
    inner: PaymentsProviderStripe,
}

#[deprecated]
pub type RTDPaymentsProviderStripeBuilder = PaymentsProviderStripeBuilder;

impl PaymentsProviderStripeBuilder {
    pub fn build(&self) -> PaymentsProviderStripe {
        self.inner.clone()
    }

    pub fn publishable_key<T: AsRef<str>>(&mut self, publishable_key: T) -> &mut Self {
        self.inner.publishable_key = publishable_key.as_ref().to_string();
        self
    }

    pub fn need_country(&mut self, need_country: bool) -> &mut Self {
        self.inner.need_country = need_country;
        self
    }

    pub fn need_postal_code(&mut self, need_postal_code: bool) -> &mut Self {
        self.inner.need_postal_code = need_postal_code;
        self
    }

    pub fn need_cardholder_name(&mut self, need_cardholder_name: bool) -> &mut Self {
        self.inner.need_cardholder_name = need_cardholder_name;
        self
    }
}

impl AsRef<PaymentsProviderStripe> for PaymentsProviderStripe {
    fn as_ref(&self) -> &PaymentsProviderStripe {
        self
    }
}

impl AsRef<PaymentsProviderStripe> for PaymentsProviderStripeBuilder {
    fn as_ref(&self) -> &PaymentsProviderStripe {
        &self.inner
    }
}
