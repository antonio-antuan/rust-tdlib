use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes an additional payment option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentOption {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Title for the payment option

    #[serde(default)]
    title: String,
    /// Payment form URL to be opened in a web view

    #[serde(default)]
    url: String,
}

impl RObject for PaymentOption {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PaymentOption {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PaymentOptionBuilder {
        let mut inner = PaymentOption::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PaymentOptionBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct PaymentOptionBuilder {
    inner: PaymentOption,
}

#[deprecated]
pub type RTDPaymentOptionBuilder = PaymentOptionBuilder;

impl PaymentOptionBuilder {
    pub fn build(&self) -> PaymentOption {
        self.inner.clone()
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<PaymentOption> for PaymentOption {
    fn as_ref(&self) -> &PaymentOption {
        self
    }
}

impl AsRef<PaymentOption> for PaymentOptionBuilder {
    fn as_ref(&self) -> &PaymentOption {
        &self.inner
    }
}
