use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Recovers the 2-step verification password using a recovery code sent to an email address that was previously set up
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecoverPassword {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Recovery code to check

    #[serde(default)]
    recovery_code: String,
    /// New password of the user; may be empty to remove the password

    #[serde(default)]
    new_password: String,
    /// New password hint; may be empty

    #[serde(default)]
    new_hint: String,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RecoverPasswordBuilder {
        let mut inner = RecoverPassword::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "recoverPassword".to_string();

        RecoverPasswordBuilder { inner }
    }

    pub fn recovery_code(&self) -> &String {
        &self.recovery_code
    }

    pub fn new_password(&self) -> &String {
        &self.new_password
    }

    pub fn new_hint(&self) -> &String {
        &self.new_hint
    }
}

#[doc(hidden)]
pub struct RecoverPasswordBuilder {
    inner: RecoverPassword,
}

#[deprecated]
pub type RTDRecoverPasswordBuilder = RecoverPasswordBuilder;

impl RecoverPasswordBuilder {
    pub fn build(&self) -> RecoverPassword {
        self.inner.clone()
    }

    pub fn recovery_code<T: AsRef<str>>(&mut self, recovery_code: T) -> &mut Self {
        self.inner.recovery_code = recovery_code.as_ref().to_string();
        self
    }

    pub fn new_password<T: AsRef<str>>(&mut self, new_password: T) -> &mut Self {
        self.inner.new_password = new_password.as_ref().to_string();
        self
    }

    pub fn new_hint<T: AsRef<str>>(&mut self, new_hint: T) -> &mut Self {
        self.inner.new_hint = new_hint.as_ref().to_string();
        self
    }
}

impl AsRef<RecoverPassword> for RecoverPassword {
    fn as_ref(&self) -> &RecoverPassword {
        self
    }
}

impl AsRef<RecoverPassword> for RecoverPasswordBuilder {
    fn as_ref(&self) -> &RecoverPassword {
        &self.inner
    }
}
