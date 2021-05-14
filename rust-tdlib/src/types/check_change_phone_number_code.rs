use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Checks the authentication code sent to confirm a new phone number of the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChangePhoneNumberCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Verification code received by SMS, phone call or flash call
    code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckChangePhoneNumberCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckChangePhoneNumberCode {}

impl CheckChangePhoneNumberCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCheckChangePhoneNumberCodeBuilder {
        let mut inner = CheckChangePhoneNumberCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkChangePhoneNumberCode".to_string();

        RTDCheckChangePhoneNumberCodeBuilder { inner }
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}

#[doc(hidden)]
pub struct RTDCheckChangePhoneNumberCodeBuilder {
    inner: CheckChangePhoneNumberCode,
}

impl RTDCheckChangePhoneNumberCodeBuilder {
    pub fn build(&self) -> CheckChangePhoneNumberCode {
        self.inner.clone()
    }

    pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
        self.inner.code = code.as_ref().to_string();
        self
    }
}

impl AsRef<CheckChangePhoneNumberCode> for CheckChangePhoneNumberCode {
    fn as_ref(&self) -> &CheckChangePhoneNumberCode {
        self
    }
}

impl AsRef<CheckChangePhoneNumberCode> for RTDCheckChangePhoneNumberCodeBuilder {
    fn as_ref(&self) -> &CheckChangePhoneNumberCode {
        &self.inner
    }
}
