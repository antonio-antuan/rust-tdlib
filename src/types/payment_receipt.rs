
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about a successful payment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentReceipt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the payment was made
  date: i64,
  /// User identifier of the payment provider bot
  payments_provider_user_id: i64,
  /// Contains information about the invoice
  invoice: Invoice,
  /// Contains order information; may be null
  order_info: Option<OrderInfo>,
  /// Chosen shipping option; may be null
  shipping_option: Option<ShippingOption>,
  /// Title of the saved credentials
  credentials_title: String,
  
}

impl RObject for PaymentReceipt {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "paymentReceipt" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PaymentReceipt {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPaymentReceiptBuilder {
    let mut inner = PaymentReceipt::default();
    inner.td_name = "paymentReceipt".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPaymentReceiptBuilder { inner }
  }

  pub fn date(&self) -> i64 { self.date }

  pub fn payments_provider_user_id(&self) -> i64 { self.payments_provider_user_id }

  pub fn invoice(&self) -> &Invoice { &self.invoice }

  pub fn order_info(&self) -> &Option<OrderInfo> { &self.order_info }

  pub fn shipping_option(&self) -> &Option<ShippingOption> { &self.shipping_option }

  pub fn credentials_title(&self) -> &String { &self.credentials_title }

}

#[doc(hidden)]
pub struct RTDPaymentReceiptBuilder {
  inner: PaymentReceipt
}

impl RTDPaymentReceiptBuilder {
  pub fn build(&self) -> PaymentReceipt { self.inner.clone() }

   
  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
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

}

impl AsRef<PaymentReceipt> for PaymentReceipt {
  fn as_ref(&self) -> &PaymentReceipt { self }
}

impl AsRef<PaymentReceipt> for RTDPaymentReceiptBuilder {
  fn as_ref(&self) -> &PaymentReceipt { &self.inner }
}



