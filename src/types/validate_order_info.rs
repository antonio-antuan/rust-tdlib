use crate::errors::*;
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
    /// Chat identifier of the Invoice message
    chat_id: i64,
    /// Message identifier
    message_id: i64,
    /// The order information, provided by the user
    order_info: OrderInfo,
    /// True, if the order information can be saved
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDValidateOrderInfoBuilder {
        let mut inner = ValidateOrderInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "validateOrderInfo".to_string();

        RTDValidateOrderInfoBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn order_info(&self) -> &OrderInfo {
        &self.order_info
    }

    pub fn allow_save(&self) -> bool {
        self.allow_save
    }
}

#[doc(hidden)]
pub struct RTDValidateOrderInfoBuilder {
    inner: ValidateOrderInfo,
}

impl RTDValidateOrderInfoBuilder {
    pub fn build(&self) -> ValidateOrderInfo {
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

impl AsRef<ValidateOrderInfo> for RTDValidateOrderInfoBuilder {
    fn as_ref(&self) -> &ValidateOrderInfo {
        &self.inner
    }
}
