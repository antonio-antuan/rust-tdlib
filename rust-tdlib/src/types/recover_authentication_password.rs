use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Recovers the password with a password recovery code sent to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecoverAuthenticationPassword {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Recovery code to check
    recovery_code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RecoverAuthenticationPassword {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RecoverAuthenticationPassword {}

impl RecoverAuthenticationPassword {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRecoverAuthenticationPasswordBuilder {
        let mut inner = RecoverAuthenticationPassword::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "recoverAuthenticationPassword".to_string();

        RTDRecoverAuthenticationPasswordBuilder { inner }
    }

    pub fn recovery_code(&self) -> &String {
        &self.recovery_code
    }
}

#[doc(hidden)]
pub struct RTDRecoverAuthenticationPasswordBuilder {
    inner: RecoverAuthenticationPassword,
}

impl RTDRecoverAuthenticationPasswordBuilder {
    pub fn build(&self) -> RecoverAuthenticationPassword {
        self.inner.clone()
    }

    pub fn recovery_code<T: AsRef<str>>(&mut self, recovery_code: T) -> &mut Self {
        self.inner.recovery_code = recovery_code.as_ref().to_string();
        self
    }
}

impl AsRef<RecoverAuthenticationPassword> for RecoverAuthenticationPassword {
    fn as_ref(&self) -> &RecoverAuthenticationPassword {
        self
    }
}

impl AsRef<RecoverAuthenticationPassword> for RTDRecoverAuthenticationPasswordBuilder {
    fn as_ref(&self) -> &RecoverAuthenticationPassword {
        &self.inner
    }
}
