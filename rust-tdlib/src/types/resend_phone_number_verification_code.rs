use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Re-sends the code to verify a phone number to be added to a user's Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendPhoneNumberVerificationCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ResendPhoneNumberVerificationCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ResendPhoneNumberVerificationCode {}

impl ResendPhoneNumberVerificationCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDResendPhoneNumberVerificationCodeBuilder {
        let mut inner = ResendPhoneNumberVerificationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resendPhoneNumberVerificationCode".to_string();

        RTDResendPhoneNumberVerificationCodeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDResendPhoneNumberVerificationCodeBuilder {
    inner: ResendPhoneNumberVerificationCode,
}

impl RTDResendPhoneNumberVerificationCodeBuilder {
    pub fn build(&self) -> ResendPhoneNumberVerificationCode {
        self.inner.clone()
    }
}

impl AsRef<ResendPhoneNumberVerificationCode> for ResendPhoneNumberVerificationCode {
    fn as_ref(&self) -> &ResendPhoneNumberVerificationCode {
        self
    }
}

impl AsRef<ResendPhoneNumberVerificationCode> for RTDResendPhoneNumberVerificationCodeBuilder {
    fn as_ref(&self) -> &ResendPhoneNumberVerificationCode {
        &self.inner
    }
}
