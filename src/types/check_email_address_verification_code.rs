use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks the email address verification code for Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckEmailAddressVerificationCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Verification code to check

    #[serde(default)]
    code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckEmailAddressVerificationCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckEmailAddressVerificationCode {}

impl CheckEmailAddressVerificationCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckEmailAddressVerificationCodeBuilder {
        let mut inner = CheckEmailAddressVerificationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkEmailAddressVerificationCode".to_string();

        CheckEmailAddressVerificationCodeBuilder { inner }
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}

#[doc(hidden)]
pub struct CheckEmailAddressVerificationCodeBuilder {
    inner: CheckEmailAddressVerificationCode,
}

#[deprecated]
pub type RTDCheckEmailAddressVerificationCodeBuilder = CheckEmailAddressVerificationCodeBuilder;

impl CheckEmailAddressVerificationCodeBuilder {
    pub fn build(&self) -> CheckEmailAddressVerificationCode {
        self.inner.clone()
    }

    pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
        self.inner.code = code.as_ref().to_string();
        self
    }
}

impl AsRef<CheckEmailAddressVerificationCode> for CheckEmailAddressVerificationCode {
    fn as_ref(&self) -> &CheckEmailAddressVerificationCode {
        self
    }
}

impl AsRef<CheckEmailAddressVerificationCode> for CheckEmailAddressVerificationCodeBuilder {
    fn as_ref(&self) -> &CheckEmailAddressVerificationCode {
        &self.inner
    }
}
