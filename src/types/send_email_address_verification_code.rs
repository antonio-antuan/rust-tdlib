use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sends a code to verify an email address to be added to a user's Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendEmailAddressVerificationCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Email address

    #[serde(default)]
    email_address: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendEmailAddressVerificationCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendEmailAddressVerificationCode {}

impl SendEmailAddressVerificationCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SendEmailAddressVerificationCodeBuilder {
        let mut inner = SendEmailAddressVerificationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendEmailAddressVerificationCode".to_string();

        SendEmailAddressVerificationCodeBuilder { inner }
    }

    pub fn email_address(&self) -> &String {
        &self.email_address
    }
}

#[doc(hidden)]
pub struct SendEmailAddressVerificationCodeBuilder {
    inner: SendEmailAddressVerificationCode,
}

#[deprecated]
pub type RTDSendEmailAddressVerificationCodeBuilder = SendEmailAddressVerificationCodeBuilder;

impl SendEmailAddressVerificationCodeBuilder {
    pub fn build(&self) -> SendEmailAddressVerificationCode {
        self.inner.clone()
    }

    pub fn email_address<T: AsRef<str>>(&mut self, email_address: T) -> &mut Self {
        self.inner.email_address = email_address.as_ref().to_string();
        self
    }
}

impl AsRef<SendEmailAddressVerificationCode> for SendEmailAddressVerificationCode {
    fn as_ref(&self) -> &SendEmailAddressVerificationCode {
        self
    }
}

impl AsRef<SendEmailAddressVerificationCode> for SendEmailAddressVerificationCodeBuilder {
    fn as_ref(&self) -> &SendEmailAddressVerificationCode {
        &self.inner
    }
}
