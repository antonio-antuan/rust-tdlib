use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks phone number confirmation code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckPhoneNumberConfirmationCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Confirmation code to check

    #[serde(default)]
    code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckPhoneNumberConfirmationCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckPhoneNumberConfirmationCode {}

impl CheckPhoneNumberConfirmationCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckPhoneNumberConfirmationCodeBuilder {
        let mut inner = CheckPhoneNumberConfirmationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkPhoneNumberConfirmationCode".to_string();

        CheckPhoneNumberConfirmationCodeBuilder { inner }
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}

#[doc(hidden)]
pub struct CheckPhoneNumberConfirmationCodeBuilder {
    inner: CheckPhoneNumberConfirmationCode,
}

#[deprecated]
pub type RTDCheckPhoneNumberConfirmationCodeBuilder = CheckPhoneNumberConfirmationCodeBuilder;

impl CheckPhoneNumberConfirmationCodeBuilder {
    pub fn build(&self) -> CheckPhoneNumberConfirmationCode {
        self.inner.clone()
    }

    pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
        self.inner.code = code.as_ref().to_string();
        self
    }
}

impl AsRef<CheckPhoneNumberConfirmationCode> for CheckPhoneNumberConfirmationCode {
    fn as_ref(&self) -> &CheckPhoneNumberConfirmationCode {
        self
    }
}

impl AsRef<CheckPhoneNumberConfirmationCode> for CheckPhoneNumberConfirmationCodeBuilder {
    fn as_ref(&self) -> &CheckPhoneNumberConfirmationCode {
        &self.inner
    }
}
