use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks the phone number verification code for Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckPhoneNumberVerificationCode {
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

impl RObject for CheckPhoneNumberVerificationCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckPhoneNumberVerificationCode {}

impl CheckPhoneNumberVerificationCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckPhoneNumberVerificationCodeBuilder {
        let mut inner = CheckPhoneNumberVerificationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkPhoneNumberVerificationCode".to_string();

        CheckPhoneNumberVerificationCodeBuilder { inner }
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}

#[doc(hidden)]
pub struct CheckPhoneNumberVerificationCodeBuilder {
    inner: CheckPhoneNumberVerificationCode,
}

#[deprecated]
pub type RTDCheckPhoneNumberVerificationCodeBuilder = CheckPhoneNumberVerificationCodeBuilder;

impl CheckPhoneNumberVerificationCodeBuilder {
    pub fn build(&self) -> CheckPhoneNumberVerificationCode {
        self.inner.clone()
    }

    pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
        self.inner.code = code.as_ref().to_string();
        self
    }
}

impl AsRef<CheckPhoneNumberVerificationCode> for CheckPhoneNumberVerificationCode {
    fn as_ref(&self) -> &CheckPhoneNumberVerificationCode {
        self
    }
}

impl AsRef<CheckPhoneNumberVerificationCode> for CheckPhoneNumberVerificationCodeBuilder {
    fn as_ref(&self) -> &CheckPhoneNumberVerificationCode {
        &self.inner
    }
}
