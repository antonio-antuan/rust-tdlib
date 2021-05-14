use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains the result of a payment request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentResult {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the payment request was successful; otherwise the verification_url will be not empty
    success: bool,
    /// URL for additional payment credentials verification
    verification_url: String,
}

impl RObject for PaymentResult {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PaymentResult {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPaymentResultBuilder {
        let mut inner = PaymentResult::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPaymentResultBuilder { inner }
    }

    pub fn success(&self) -> bool {
        self.success
    }

    pub fn verification_url(&self) -> &String {
        &self.verification_url
    }
}

#[doc(hidden)]
pub struct RTDPaymentResultBuilder {
    inner: PaymentResult,
}

impl RTDPaymentResultBuilder {
    pub fn build(&self) -> PaymentResult {
        self.inner.clone()
    }

    pub fn success(&mut self, success: bool) -> &mut Self {
        self.inner.success = success;
        self
    }

    pub fn verification_url<T: AsRef<str>>(&mut self, verification_url: T) -> &mut Self {
        self.inner.verification_url = verification_url.as_ref().to_string();
        self
    }
}

impl AsRef<PaymentResult> for PaymentResult {
    fn as_ref(&self) -> &PaymentResult {
        self
    }
}

impl AsRef<PaymentResult> for RTDPaymentResultBuilder {
    fn as_ref(&self) -> &PaymentResult {
        &self.inner
    }
}
