use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks whether a 2-step verification password recovery code sent to an email address is valid
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckPasswordRecoveryCode {
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

impl RObject for CheckPasswordRecoveryCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckPasswordRecoveryCode {}

impl CheckPasswordRecoveryCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckPasswordRecoveryCodeBuilder {
        let mut inner = CheckPasswordRecoveryCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkPasswordRecoveryCode".to_string();

        CheckPasswordRecoveryCodeBuilder { inner }
    }

    pub fn recovery_code(&self) -> &String {
        &self.recovery_code
    }
}

#[doc(hidden)]
pub struct CheckPasswordRecoveryCodeBuilder {
    inner: CheckPasswordRecoveryCode,
}

#[deprecated]
pub type RTDCheckPasswordRecoveryCodeBuilder = CheckPasswordRecoveryCodeBuilder;

impl CheckPasswordRecoveryCodeBuilder {
    pub fn build(&self) -> CheckPasswordRecoveryCode {
        self.inner.clone()
    }

    pub fn recovery_code<T: AsRef<str>>(&mut self, recovery_code: T) -> &mut Self {
        self.inner.recovery_code = recovery_code.as_ref().to_string();
        self
    }
}

impl AsRef<CheckPasswordRecoveryCode> for CheckPasswordRecoveryCode {
    fn as_ref(&self) -> &CheckPasswordRecoveryCode {
        self
    }
}

impl AsRef<CheckPasswordRecoveryCode> for CheckPasswordRecoveryCodeBuilder {
    fn as_ref(&self) -> &CheckPasswordRecoveryCode {
        &self.inner
    }
}
