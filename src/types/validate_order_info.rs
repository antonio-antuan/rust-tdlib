use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Validates the order information provided by a user and returns the available shipping options for a flexible invoice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidateOrderInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The invoice

    #[serde(skip_serializing_if = "InputInvoice::_is_default")]
    input_invoice: InputInvoice,
    /// The order information, provided by the user; pass null if empty
    order_info: OrderInfo,
    /// Pass true to save the order information

    #[serde(default)]
    allow_save: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ValidateOrderInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ValidateOrderInfo {}

impl ValidateOrderInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ValidateOrderInfoBuilder {
        let mut inner = ValidateOrderInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "validateOrderInfo".to_string();

        ValidateOrderInfoBuilder { inner }
    }

    pub fn input_invoice(&self) -> &InputInvoice {
        &self.input_invoice
    }

    pub fn order_info(&self) -> &OrderInfo {
        &self.order_info
    }

    pub fn allow_save(&self) -> bool {
        self.allow_save
    }
}

#[doc(hidden)]
pub struct ValidateOrderInfoBuilder {
    inner: ValidateOrderInfo,
}

#[deprecated]
pub type RTDValidateOrderInfoBuilder = ValidateOrderInfoBuilder;

impl ValidateOrderInfoBuilder {
    pub fn build(&self) -> ValidateOrderInfo {
        self.inner.clone()
    }

    pub fn input_invoice<T: AsRef<InputInvoice>>(&mut self, input_invoice: T) -> &mut Self {
        self.inner.input_invoice = input_invoice.as_ref().clone();
        self
    }

    pub fn order_info<T: AsRef<OrderInfo>>(&mut self, order_info: T) -> &mut Self {
        self.inner.order_info = order_info.as_ref().clone();
        self
    }

    pub fn allow_save(&mut self, allow_save: bool) -> &mut Self {
        self.inner.allow_save = allow_save;
        self
    }
}

impl AsRef<ValidateOrderInfo> for ValidateOrderInfo {
    fn as_ref(&self) -> &ValidateOrderInfo {
        self
    }
}

impl AsRef<ValidateOrderInfo> for ValidateOrderInfoBuilder {
    fn as_ref(&self) -> &ValidateOrderInfo {
        &self.inner
    }
}
