use crate::errors::*;
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
    /// The phone number confirmation code
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCheckPhoneNumberConfirmationCodeBuilder {
        let mut inner = CheckPhoneNumberConfirmationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkPhoneNumberConfirmationCode".to_string();

        RTDCheckPhoneNumberConfirmationCodeBuilder { inner }
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}

#[doc(hidden)]
pub struct RTDCheckPhoneNumberConfirmationCodeBuilder {
    inner: CheckPhoneNumberConfirmationCode,
}

impl RTDCheckPhoneNumberConfirmationCodeBuilder {
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

impl AsRef<CheckPhoneNumberConfirmationCode> for RTDCheckPhoneNumberConfirmationCodeBuilder {
    fn as_ref(&self) -> &CheckPhoneNumberConfirmationCode {
        &self.inner
    }
}
