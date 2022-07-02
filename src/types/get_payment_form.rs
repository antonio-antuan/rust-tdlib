use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an invoice payment form. This method must be called when the user presses inlineKeyboardButtonBuy
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPaymentForm {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier of the Invoice message

    #[serde(default)]
    chat_id: i64,
    /// Message identifier

    #[serde(default)]
    message_id: i64,
    /// Preferred payment form theme; pass null to use the default theme
    theme: PaymentFormTheme,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPaymentForm {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPaymentForm {}

impl GetPaymentForm {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetPaymentFormBuilder {
        let mut inner = GetPaymentForm::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPaymentForm".to_string();

        GetPaymentFormBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn theme(&self) -> &PaymentFormTheme {
        &self.theme
    }
}

#[doc(hidden)]
pub struct GetPaymentFormBuilder {
    inner: GetPaymentForm,
}

#[deprecated]
pub type RTDGetPaymentFormBuilder = GetPaymentFormBuilder;

impl GetPaymentFormBuilder {
    pub fn build(&self) -> GetPaymentForm {
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

    pub fn theme<T: AsRef<PaymentFormTheme>>(&mut self, theme: T) -> &mut Self {
        self.inner.theme = theme.as_ref().clone();
        self
    }
}

impl AsRef<GetPaymentForm> for GetPaymentForm {
    fn as_ref(&self) -> &GetPaymentForm {
        self
    }
}

impl AsRef<GetPaymentForm> for GetPaymentFormBuilder {
    fn as_ref(&self) -> &GetPaymentForm {
        &self.inner
    }
}
