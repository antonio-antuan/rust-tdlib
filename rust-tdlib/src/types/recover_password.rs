use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Recovers the password using a recovery code sent to an email address that was previously set up
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecoverPassword {
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

impl RObject for RecoverPassword {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RecoverPassword {}

impl RecoverPassword {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRecoverPasswordBuilder {
        let mut inner = RecoverPassword::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "recoverPassword".to_string();

        RTDRecoverPasswordBuilder { inner }
    }

    pub fn recovery_code(&self) -> &String {
        &self.recovery_code
    }
}

#[doc(hidden)]
pub struct RTDRecoverPasswordBuilder {
    inner: RecoverPassword,
}

impl RTDRecoverPasswordBuilder {
    pub fn build(&self) -> RecoverPassword {
        self.inner.clone()
    }

    pub fn recovery_code<T: AsRef<str>>(&mut self, recovery_code: T) -> &mut Self {
        self.inner.recovery_code = recovery_code.as_ref().to_string();
        self
    }
}

impl AsRef<RecoverPassword> for RecoverPassword {
    fn as_ref(&self) -> &RecoverPassword {
        self
    }
}

impl AsRef<RecoverPassword> for RTDRecoverPasswordBuilder {
    fn as_ref(&self) -> &RecoverPassword {
        &self.inner
    }
}
