use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Creates a link for the given invoice; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInvoiceLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Information about the invoice of the type inputMessageInvoice

    #[serde(skip_serializing_if = "InputMessageContent::_is_default")]
    invoice: InputMessageContent,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateInvoiceLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateInvoiceLink {}

impl CreateInvoiceLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCreateInvoiceLinkBuilder {
        let mut inner = CreateInvoiceLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createInvoiceLink".to_string();

        RTDCreateInvoiceLinkBuilder { inner }
    }

    pub fn invoice(&self) -> &InputMessageContent {
        &self.invoice
    }
}

#[doc(hidden)]
pub struct RTDCreateInvoiceLinkBuilder {
    inner: CreateInvoiceLink,
}

impl RTDCreateInvoiceLinkBuilder {
    pub fn build(&self) -> CreateInvoiceLink {
        self.inner.clone()
    }

    pub fn invoice<T: AsRef<InputMessageContent>>(&mut self, invoice: T) -> &mut Self {
        self.inner.invoice = invoice.as_ref().clone();
        self
    }
}

impl AsRef<CreateInvoiceLink> for CreateInvoiceLink {
    fn as_ref(&self) -> &CreateInvoiceLink {
        self
    }
}

impl AsRef<CreateInvoiceLink> for RTDCreateInvoiceLinkBuilder {
    fn as_ref(&self) -> &CreateInvoiceLink {
        &self.inner
    }
}
