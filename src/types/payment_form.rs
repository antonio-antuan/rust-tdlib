use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about an invoice payment form
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentForm {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The payment form identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// Full information of the invoice
    invoice: Invoice,
    /// Payment form URL

    #[serde(default)]
    url: String,
    /// User identifier of the seller bot

    #[serde(default)]
    seller_bot_user_id: i64,
    /// User identifier of the payment provider bot

    #[serde(default)]
    payments_provider_user_id: i64,
    /// Information about the payment provider, if available, to support it natively without the need for opening the URL; may be null
    payments_provider: Option<PaymentsProviderStripe>,
    /// Saved server-side order information; may be null
    saved_order_info: Option<OrderInfo>,
    /// Information about saved card credentials; may be null
    saved_credentials: Option<SavedCredentials>,
    /// True, if the user can choose to save credentials

    #[serde(default)]
    can_save_credentials: bool,
    /// True, if the user will be able to save credentials protected by a password they set up

    #[serde(default)]
    need_password: bool,
}

impl RObject for PaymentForm {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PaymentForm {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PaymentFormBuilder {
        let mut inner = PaymentForm::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PaymentFormBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn invoice(&self) -> &Invoice {
        &self.invoice
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn seller_bot_user_id(&self) -> i64 {
        self.seller_bot_user_id
    }

    pub fn payments_provider_user_id(&self) -> i64 {
        self.payments_provider_user_id
    }

    pub fn payments_provider(&self) -> &Option<PaymentsProviderStripe> {
        &self.payments_provider
    }

    pub fn saved_order_info(&self) -> &Option<OrderInfo> {
        &self.saved_order_info
    }

    pub fn saved_credentials(&self) -> &Option<SavedCredentials> {
        &self.saved_credentials
    }

    pub fn can_save_credentials(&self) -> bool {
        self.can_save_credentials
    }

    pub fn need_password(&self) -> bool {
        self.need_password
    }
}

#[doc(hidden)]
pub struct PaymentFormBuilder {
    inner: PaymentForm,
}

#[deprecated]
pub type RTDPaymentFormBuilder = PaymentFormBuilder;

impl PaymentFormBuilder {
    pub fn build(&self) -> PaymentForm {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn invoice<T: AsRef<Invoice>>(&mut self, invoice: T) -> &mut Self {
        self.inner.invoice = invoice.as_ref().clone();
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
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

    pub fn payments_provider<T: AsRef<PaymentsProviderStripe>>(
        &mut self,
        payments_provider: T,
    ) -> &mut Self {
        self.inner.payments_provider = Some(payments_provider.as_ref().clone());
        self
    }

    pub fn saved_order_info<T: AsRef<OrderInfo>>(&mut self, saved_order_info: T) -> &mut Self {
        self.inner.saved_order_info = Some(saved_order_info.as_ref().clone());
        self
    }

    pub fn saved_credentials<T: AsRef<SavedCredentials>>(
        &mut self,
        saved_credentials: T,
    ) -> &mut Self {
        self.inner.saved_credentials = Some(saved_credentials.as_ref().clone());
        self
    }

    pub fn can_save_credentials(&mut self, can_save_credentials: bool) -> &mut Self {
        self.inner.can_save_credentials = can_save_credentials;
        self
    }

    pub fn need_password(&mut self, need_password: bool) -> &mut Self {
        self.inner.need_password = need_password;
        self
    }
}

impl AsRef<PaymentForm> for PaymentForm {
    fn as_ref(&self) -> &PaymentForm {
        self
    }
}

impl AsRef<PaymentForm> for PaymentFormBuilder {
    fn as_ref(&self) -> &PaymentForm {
        &self.inner
    }
}
