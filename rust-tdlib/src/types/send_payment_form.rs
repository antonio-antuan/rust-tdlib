use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends a filled-out payment form to the bot for final verification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendPaymentForm {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier of the Invoice message
    chat_id: i64,
    /// Message identifier
    message_id: i64,
    /// Identifier returned by ValidateOrderInfo, or an empty string
    order_info_id: String,
    /// Identifier of a chosen shipping option, if applicable
    shipping_option_id: String,
    /// The credentials chosen by user for payment

    #[serde(skip_serializing_if = "InputCredentials::_is_default")]
    credentials: InputCredentials,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendPaymentForm {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendPaymentForm {}

impl SendPaymentForm {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendPaymentFormBuilder {
        let mut inner = SendPaymentForm::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendPaymentForm".to_string();

        RTDSendPaymentFormBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn order_info_id(&self) -> &String {
        &self.order_info_id
    }

    pub fn shipping_option_id(&self) -> &String {
        &self.shipping_option_id
    }

    pub fn credentials(&self) -> &InputCredentials {
        &self.credentials
    }
}

#[doc(hidden)]
pub struct RTDSendPaymentFormBuilder {
    inner: SendPaymentForm,
}

impl RTDSendPaymentFormBuilder {
    pub fn build(&self) -> SendPaymentForm {
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

    pub fn order_info_id<T: AsRef<str>>(&mut self, order_info_id: T) -> &mut Self {
        self.inner.order_info_id = order_info_id.as_ref().to_string();
        self
    }

    pub fn shipping_option_id<T: AsRef<str>>(&mut self, shipping_option_id: T) -> &mut Self {
        self.inner.shipping_option_id = shipping_option_id.as_ref().to_string();
        self
    }

    pub fn credentials<T: AsRef<InputCredentials>>(&mut self, credentials: T) -> &mut Self {
        self.inner.credentials = credentials.as_ref().clone();
        self
    }
}

impl AsRef<SendPaymentForm> for SendPaymentForm {
    fn as_ref(&self) -> &SendPaymentForm {
        self
    }
}

impl AsRef<SendPaymentForm> for RTDSendPaymentFormBuilder {
    fn as_ref(&self) -> &SendPaymentForm {
        &self.inner
    }
}
