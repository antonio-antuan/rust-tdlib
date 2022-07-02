use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks whether a password recovery code sent to an email address is valid. Works only when the current authorization state is authorizationStateWaitPassword
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckAuthenticationPasswordRecoveryCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Recovery code to check

    #[serde(default)]
    recovery_code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckAuthenticationPasswordRecoveryCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckAuthenticationPasswordRecoveryCode {}

impl CheckAuthenticationPasswordRecoveryCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckAuthenticationPasswordRecoveryCodeBuilder {
        let mut inner = CheckAuthenticationPasswordRecoveryCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkAuthenticationPasswordRecoveryCode".to_string();

        CheckAuthenticationPasswordRecoveryCodeBuilder { inner }
    }

    pub fn recovery_code(&self) -> &String {
        &self.recovery_code
    }
}

#[doc(hidden)]
pub struct CheckAuthenticationPasswordRecoveryCodeBuilder {
    inner: CheckAuthenticationPasswordRecoveryCode,
}

#[deprecated]
pub type RTDCheckAuthenticationPasswordRecoveryCodeBuilder =
    CheckAuthenticationPasswordRecoveryCodeBuilder;

impl CheckAuthenticationPasswordRecoveryCodeBuilder {
    pub fn build(&self) -> CheckAuthenticationPasswordRecoveryCode {
        self.inner.clone()
    }

    pub fn recovery_code<T: AsRef<str>>(&mut self, recovery_code: T) -> &mut Self {
        self.inner.recovery_code = recovery_code.as_ref().to_string();
        self
    }
}

impl AsRef<CheckAuthenticationPasswordRecoveryCode> for CheckAuthenticationPasswordRecoveryCode {
    fn as_ref(&self) -> &CheckAuthenticationPasswordRecoveryCode {
        self
    }
}

impl AsRef<CheckAuthenticationPasswordRecoveryCode>
    for CheckAuthenticationPasswordRecoveryCodeBuilder
{
    fn as_ref(&self) -> &CheckAuthenticationPasswordRecoveryCode {
        &self.inner
    }
}
