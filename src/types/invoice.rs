use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Product invoice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Invoice {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// ISO 4217 currency code

    #[serde(default)]
    currency: String,
    /// A list of objects used to calculate the total price of the product

    #[serde(default)]
    price_parts: Vec<LabeledPricePart>,
    /// The maximum allowed amount of tip in the smallest units of the currency

    #[serde(default)]
    max_tip_amount: i64,
    /// Suggested amounts of tip in the smallest units of the currency

    #[serde(default)]
    suggested_tip_amounts: Vec<i64>,
    /// An HTTP URL with terms of service for recurring payments. If non-empty, the invoice payment will result in recurring payments and the user must accept the terms of service before allowed to pay

    #[serde(default)]
    recurring_payment_terms_of_service_url: String,
    /// True, if the payment is a test payment

    #[serde(default)]
    is_test: bool,
    /// True, if the user's name is needed for payment

    #[serde(default)]
    need_name: bool,
    /// True, if the user's phone number is needed for payment

    #[serde(default)]
    need_phone_number: bool,
    /// True, if the user's email address is needed for payment

    #[serde(default)]
    need_email_address: bool,
    /// True, if the user's shipping address is needed for payment

    #[serde(default)]
    need_shipping_address: bool,
    /// True, if the user's phone number will be sent to the provider

    #[serde(default)]
    send_phone_number_to_provider: bool,
    /// True, if the user's email address will be sent to the provider

    #[serde(default)]
    send_email_address_to_provider: bool,
    /// True, if the total price depends on the shipping method

    #[serde(default)]
    is_flexible: bool,
}

impl RObject for Invoice {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Invoice {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInvoiceBuilder {
        let mut inner = Invoice::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInvoiceBuilder { inner }
    }

    pub fn currency(&self) -> &String {
        &self.currency
    }

    pub fn price_parts(&self) -> &Vec<LabeledPricePart> {
        &self.price_parts
    }

    pub fn max_tip_amount(&self) -> i64 {
        self.max_tip_amount
    }

    pub fn suggested_tip_amounts(&self) -> &Vec<i64> {
        &self.suggested_tip_amounts
    }

    pub fn recurring_payment_terms_of_service_url(&self) -> &String {
        &self.recurring_payment_terms_of_service_url
    }

    pub fn is_test(&self) -> bool {
        self.is_test
    }

    pub fn need_name(&self) -> bool {
        self.need_name
    }

    pub fn need_phone_number(&self) -> bool {
        self.need_phone_number
    }

    pub fn need_email_address(&self) -> bool {
        self.need_email_address
    }

    pub fn need_shipping_address(&self) -> bool {
        self.need_shipping_address
    }

    pub fn send_phone_number_to_provider(&self) -> bool {
        self.send_phone_number_to_provider
    }

    pub fn send_email_address_to_provider(&self) -> bool {
        self.send_email_address_to_provider
    }

    pub fn is_flexible(&self) -> bool {
        self.is_flexible
    }
}

#[doc(hidden)]
pub struct RTDInvoiceBuilder {
    inner: Invoice,
}

impl RTDInvoiceBuilder {
    pub fn build(&self) -> Invoice {
        self.inner.clone()
    }

    pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
        self.inner.currency = currency.as_ref().to_string();
        self
    }

    pub fn price_parts(&mut self, price_parts: Vec<LabeledPricePart>) -> &mut Self {
        self.inner.price_parts = price_parts;
        self
    }

    pub fn max_tip_amount(&mut self, max_tip_amount: i64) -> &mut Self {
        self.inner.max_tip_amount = max_tip_amount;
        self
    }

    pub fn suggested_tip_amounts(&mut self, suggested_tip_amounts: Vec<i64>) -> &mut Self {
        self.inner.suggested_tip_amounts = suggested_tip_amounts;
        self
    }

    pub fn recurring_payment_terms_of_service_url<T: AsRef<str>>(
        &mut self,
        recurring_payment_terms_of_service_url: T,
    ) -> &mut Self {
        self.inner.recurring_payment_terms_of_service_url =
            recurring_payment_terms_of_service_url.as_ref().to_string();
        self
    }

    pub fn is_test(&mut self, is_test: bool) -> &mut Self {
        self.inner.is_test = is_test;
        self
    }

    pub fn need_name(&mut self, need_name: bool) -> &mut Self {
        self.inner.need_name = need_name;
        self
    }

    pub fn need_phone_number(&mut self, need_phone_number: bool) -> &mut Self {
        self.inner.need_phone_number = need_phone_number;
        self
    }

    pub fn need_email_address(&mut self, need_email_address: bool) -> &mut Self {
        self.inner.need_email_address = need_email_address;
        self
    }

    pub fn need_shipping_address(&mut self, need_shipping_address: bool) -> &mut Self {
        self.inner.need_shipping_address = need_shipping_address;
        self
    }

    pub fn send_phone_number_to_provider(
        &mut self,
        send_phone_number_to_provider: bool,
    ) -> &mut Self {
        self.inner.send_phone_number_to_provider = send_phone_number_to_provider;
        self
    }

    pub fn send_email_address_to_provider(
        &mut self,
        send_email_address_to_provider: bool,
    ) -> &mut Self {
        self.inner.send_email_address_to_provider = send_email_address_to_provider;
        self
    }

    pub fn is_flexible(&mut self, is_flexible: bool) -> &mut Self {
        self.inner.is_flexible = is_flexible;
        self
    }
}

impl AsRef<Invoice> for Invoice {
    fn as_ref(&self) -> &Invoice {
        self
    }
}

impl AsRef<Invoice> for RTDInvoiceBuilder {
    fn as_ref(&self) -> &Invoice {
        &self.inner
    }
}
