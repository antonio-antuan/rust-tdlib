use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Re-sends the code to verify an email address to be added to a user's Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendEmailAddressVerificationCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ResendEmailAddressVerificationCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ResendEmailAddressVerificationCode {}

impl ResendEmailAddressVerificationCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDResendEmailAddressVerificationCodeBuilder {
        let mut inner = ResendEmailAddressVerificationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resendEmailAddressVerificationCode".to_string();

        RTDResendEmailAddressVerificationCodeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDResendEmailAddressVerificationCodeBuilder {
    inner: ResendEmailAddressVerificationCode,
}

impl RTDResendEmailAddressVerificationCodeBuilder {
    pub fn build(&self) -> ResendEmailAddressVerificationCode {
        self.inner.clone()
    }
}

impl AsRef<ResendEmailAddressVerificationCode> for ResendEmailAddressVerificationCode {
    fn as_ref(&self) -> &ResendEmailAddressVerificationCode {
        self
    }
}

impl AsRef<ResendEmailAddressVerificationCode> for RTDResendEmailAddressVerificationCodeBuilder {
    fn as_ref(&self) -> &ResendEmailAddressVerificationCode {
        &self.inner
    }
}
