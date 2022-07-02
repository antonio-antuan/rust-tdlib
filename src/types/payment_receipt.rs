use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a successful payment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentReceipt {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Product title

    #[serde(default)]
    title: String,
    /// Contains information about a successful payment

    #[serde(default)]
    description: String,
    /// Product photo; may be null
    photo: Option<Photo>,
    /// Point in time (Unix timestamp) when the payment was made

    #[serde(default)]
    date: i32,
    /// User identifier of the seller bot

    #[serde(default)]
    seller_bot_user_id: i64,
    /// User identifier of the payment provider bot

    #[serde(default)]
    payments_provider_user_id: i64,
    /// Information about the invoice
    invoice: Invoice,
    /// Order information; may be null
    order_info: Option<OrderInfo>,
    /// Chosen shipping option; may be null
    shipping_option: Option<ShippingOption>,
    /// Title of the saved credentials chosen by the buyer

    #[serde(default)]
    credentials_title: String,
    /// The amount of tip chosen by the buyer in the smallest units of the currency

    #[serde(default)]
    tip_amount: i64,
}

impl RObject for PaymentReceipt {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PaymentReceipt {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PaymentReceiptBuilder {
        let mut inner = PaymentReceipt::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PaymentReceiptBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn photo(&self) -> &Option<Photo> {
        &self.photo
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn seller_bot_user_id(&self) -> i64 {
        self.seller_bot_user_id
    }

    pub fn payments_provider_user_id(&self) -> i64 {
        self.payments_provider_user_id
    }

    pub fn invoice(&self) -> &Invoice {
        &self.invoice
    }

    pub fn order_info(&self) -> &Option<OrderInfo> {
        &self.order_info
    }

    pub fn shipping_option(&self) -> &Option<ShippingOption> {
        &self.shipping_option
    }

    pub fn credentials_title(&self) -> &String {
        &self.credentials_title
    }

    pub fn tip_amount(&self) -> i64 {
        self.tip_amount
    }
}

#[doc(hidden)]
pub struct PaymentReceiptBuilder {
    inner: PaymentReceipt,
}

#[deprecated]
pub type RTDPaymentReceiptBuilder = PaymentReceiptBuilder;

impl PaymentReceiptBuilder {
    pub fn build(&self) -> PaymentReceipt {
        self.inner.clone()
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }

    pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = Some(photo.as_ref().clone());
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn seller_bot_user_id(&mut self, seller_bot_user_id: i64) -> &mut Self {
        self.inner.seller_bot_user_id = seller_bot_user_id;
        self
    }

    pub fn payments_provider_user_id(&mut self, payments_provider_user_id: i64) -> &mut Self {
        self.inner.payments_provider_user_id = payments_provider_user_id;
        self
    }

    pub fn invoice<T: AsRef<Invoice>>(&mut self, invoice: T) -> &mut Self {
        self.inner.invoice = invoice.as_ref().clone();
        self
    }

    pub fn order_info<T: AsRef<OrderInfo>>(&mut self, order_info: T) -> &mut Self {
        self.inner.order_info = Some(order_info.as_ref().clone());
        self
    }

    pub fn shipping_option<T: AsRef<ShippingOption>>(&mut self, shipping_option: T) -> &mut Self {
        self.inner.shipping_option = Some(shipping_option.as_ref().clone());
        self
    }

    pub fn credentials_title<T: AsRef<str>>(&mut self, credentials_title: T) -> &mut Self {
        self.inner.credentials_title = credentials_title.as_ref().to_string();
        self
    }

    pub fn tip_amount(&mut self, tip_amount: i64) -> &mut Self {
        self.inner.tip_amount = tip_amount;
        self
    }
}

impl AsRef<PaymentReceipt> for PaymentReceipt {
    fn as_ref(&self) -> &PaymentReceipt {
        self
    }
}

impl AsRef<PaymentReceipt> for PaymentReceiptBuilder {
    fn as_ref(&self) -> &PaymentReceipt {
        &self.inner
    }
}
