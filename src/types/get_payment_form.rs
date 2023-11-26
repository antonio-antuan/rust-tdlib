use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an invoice payment form. This method must be called when the user presses inline button of the type inlineKeyboardButtonTypeBuy
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPaymentForm {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The invoice

    #[serde(skip_serializing_if = "InputInvoice::_is_default")]
    input_invoice: InputInvoice,
    /// Preferred payment form theme; pass null to use the default theme
    theme: ThemeParameters,

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

    pub fn input_invoice(&self) -> &InputInvoice {
        &self.input_invoice
    }

    pub fn theme(&self) -> &ThemeParameters {
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

    pub fn input_invoice<T: AsRef<InputInvoice>>(&mut self, input_invoice: T) -> &mut Self {
        self.inner.input_invoice = input_invoice.as_ref().clone();
        self
    }

    pub fn theme<T: AsRef<ThemeParameters>>(&mut self, theme: T) -> &mut Self {
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
