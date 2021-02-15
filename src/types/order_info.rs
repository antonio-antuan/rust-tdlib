use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Order information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the user
    name: String,
    /// Phone number of the user
    phone_number: String,
    /// Email address of the user
    email_address: String,
    /// Shipping address for this order; may be null
    shipping_address: Option<Address>,
}

impl RObject for OrderInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl OrderInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDOrderInfoBuilder {
        let mut inner = OrderInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDOrderInfoBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }

    pub fn email_address(&self) -> &String {
        &self.email_address
    }

    pub fn shipping_address(&self) -> &Option<Address> {
        &self.shipping_address
    }
}

#[doc(hidden)]
pub struct RTDOrderInfoBuilder {
    inner: OrderInfo,
}

impl RTDOrderInfoBuilder {
    pub fn build(&self) -> OrderInfo {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
        self.inner.phone_number = phone_number.as_ref().to_string();
        self
    }

    pub fn email_address<T: AsRef<str>>(&mut self, email_address: T) -> &mut Self {
        self.inner.email_address = email_address.as_ref().to_string();
        self
    }

    pub fn shipping_address<T: AsRef<Address>>(&mut self, shipping_address: T) -> &mut Self {
        self.inner.shipping_address = Some(shipping_address.as_ref().clone());
        self
    }
}

impl AsRef<OrderInfo> for OrderInfo {
    fn as_ref(&self) -> &OrderInfo {
        self
    }
}

impl AsRef<OrderInfo> for RTDOrderInfoBuilder {
    fn as_ref(&self) -> &OrderInfo {
        &self.inner
    }
}
