use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about a payment provider
pub trait TDPaymentProvider: Debug + RObject {}

/// Contains information about a payment provider
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentProvider {
    #[doc(hidden)]
    _Default,
    /// Some other payment provider, for which a web payment form must be shown
    #[serde(rename(deserialize = "paymentProviderOther"))]
    Other(PaymentProviderOther),
    /// Smart Glocal payment provider
    #[serde(rename(deserialize = "paymentProviderSmartGlocal"))]
    SmartGlocal(PaymentProviderSmartGlocal),
    /// Stripe payment provider
    #[serde(rename(deserialize = "paymentProviderStripe"))]
    Stripe(PaymentProviderStripe),
}

impl Default for PaymentProvider {
    fn default() -> Self {
        PaymentProvider::_Default
    }
}

impl RObject for PaymentProvider {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PaymentProvider::Other(t) => t.extra(),
            PaymentProvider::SmartGlocal(t) => t.extra(),
            PaymentProvider::Stripe(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PaymentProvider::Other(t) => t.client_id(),
            PaymentProvider::SmartGlocal(t) => t.client_id(),
            PaymentProvider::Stripe(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PaymentProvider {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PaymentProvider::_Default)
    }
}

impl AsRef<PaymentProvider> for PaymentProvider {
    fn as_ref(&self) -> &PaymentProvider {
        self
    }
}

/// Some other payment provider, for which a web payment form must be shown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentProviderOther {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Payment form URL

    #[serde(default)]
    url: String,
}

impl RObject for PaymentProviderOther {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPaymentProvider for PaymentProviderOther {}

impl PaymentProviderOther {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPaymentProviderOtherBuilder {
        let mut inner = PaymentProviderOther::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPaymentProviderOtherBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct RTDPaymentProviderOtherBuilder {
    inner: PaymentProviderOther,
}

impl RTDPaymentProviderOtherBuilder {
    pub fn build(&self) -> PaymentProviderOther {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<PaymentProviderOther> for PaymentProviderOther {
    fn as_ref(&self) -> &PaymentProviderOther {
        self
    }
}

impl AsRef<PaymentProviderOther> for RTDPaymentProviderOtherBuilder {
    fn as_ref(&self) -> &PaymentProviderOther {
        &self.inner
    }
}

/// Smart Glocal payment provider
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentProviderSmartGlocal {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Public payment token

    #[serde(default)]
    public_token: String,
}

impl RObject for PaymentProviderSmartGlocal {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPaymentProvider for PaymentProviderSmartGlocal {}

impl PaymentProviderSmartGlocal {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPaymentProviderSmartGlocalBuilder {
        let mut inner = PaymentProviderSmartGlocal::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPaymentProviderSmartGlocalBuilder { inner }
    }

    pub fn public_token(&self) -> &String {
        &self.public_token
    }
}

#[doc(hidden)]
pub struct RTDPaymentProviderSmartGlocalBuilder {
    inner: PaymentProviderSmartGlocal,
}

impl RTDPaymentProviderSmartGlocalBuilder {
    pub fn build(&self) -> PaymentProviderSmartGlocal {
        self.inner.clone()
    }

    pub fn public_token<T: AsRef<str>>(&mut self, public_token: T) -> &mut Self {
        self.inner.public_token = public_token.as_ref().to_string();
        self
    }
}

impl AsRef<PaymentProviderSmartGlocal> for PaymentProviderSmartGlocal {
    fn as_ref(&self) -> &PaymentProviderSmartGlocal {
        self
    }
}

impl AsRef<PaymentProviderSmartGlocal> for RTDPaymentProviderSmartGlocalBuilder {
    fn as_ref(&self) -> &PaymentProviderSmartGlocal {
        &self.inner
    }
}

/// Stripe payment provider
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentProviderStripe {
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

impl RObject for PaymentProviderStripe {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPaymentProvider for PaymentProviderStripe {}

impl PaymentProviderStripe {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPaymentProviderStripeBuilder {
        let mut inner = PaymentProviderStripe::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPaymentProviderStripeBuilder { inner }
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
pub struct RTDPaymentProviderStripeBuilder {
    inner: PaymentProviderStripe,
}

impl RTDPaymentProviderStripeBuilder {
    pub fn build(&self) -> PaymentProviderStripe {
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

impl AsRef<PaymentProviderStripe> for PaymentProviderStripe {
    fn as_ref(&self) -> &PaymentProviderStripe {
        self
    }
}

impl AsRef<PaymentProviderStripe> for RTDPaymentProviderStripeBuilder {
    fn as_ref(&self) -> &PaymentProviderStripe {
        &self.inner
    }
}
