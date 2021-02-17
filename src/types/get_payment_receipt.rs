use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a successful payment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPaymentReceipt {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier of the PaymentSuccessful message
    chat_id: i64,
    /// Message identifier
    message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPaymentReceipt {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPaymentReceipt {}

impl GetPaymentReceipt {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetPaymentReceiptBuilder {
        let mut inner = GetPaymentReceipt::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPaymentReceipt".to_string();

        RTDGetPaymentReceiptBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct RTDGetPaymentReceiptBuilder {
    inner: GetPaymentReceipt,
}

impl RTDGetPaymentReceiptBuilder {
    pub fn build(&self) -> GetPaymentReceipt {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }
}

impl AsRef<GetPaymentReceipt> for GetPaymentReceipt {
    fn as_ref(&self) -> &GetPaymentReceipt {
        self
    }
}

impl AsRef<GetPaymentReceipt> for RTDGetPaymentReceiptBuilder {
    fn as_ref(&self) -> &GetPaymentReceipt {
        &self.inner
    }
}
