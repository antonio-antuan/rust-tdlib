use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks the authentication password for correctness. Works only when the current authorization state is authorizationStateWaitPassword
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckAuthenticationPassword {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The password to check

    #[serde(default)]
    password: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckAuthenticationPassword {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckAuthenticationPassword {}

impl CheckAuthenticationPassword {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckAuthenticationPasswordBuilder {
        let mut inner = CheckAuthenticationPassword::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkAuthenticationPassword".to_string();

        CheckAuthenticationPasswordBuilder { inner }
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

#[doc(hidden)]
pub struct CheckAuthenticationPasswordBuilder {
    inner: CheckAuthenticationPassword,
}

#[deprecated]
pub type RTDCheckAuthenticationPasswordBuilder = CheckAuthenticationPasswordBuilder;

impl CheckAuthenticationPasswordBuilder {
    pub fn build(&self) -> CheckAuthenticationPassword {
        self.inner.clone()
    }

    pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.inner.password = password.as_ref().to_string();
        self
    }
}

impl AsRef<CheckAuthenticationPassword> for CheckAuthenticationPassword {
    fn as_ref(&self) -> &CheckAuthenticationPassword {
        self
    }
}

impl AsRef<CheckAuthenticationPassword> for CheckAuthenticationPasswordBuilder {
    fn as_ref(&self) -> &CheckAuthenticationPassword {
        &self.inner
    }
}
