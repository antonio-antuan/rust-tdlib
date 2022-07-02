use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Removes 2-step verification password without previous password and access to recovery email address. The password can't be reset immediately and the request needs to be repeated after the specified time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetPassword {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ResetPassword {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDResetPasswordResult for ResetPassword {}

impl RFunction for ResetPassword {}

impl ResetPassword {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ResetPasswordBuilder {
        let mut inner = ResetPassword::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resetPassword".to_string();

        ResetPasswordBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ResetPasswordBuilder {
    inner: ResetPassword,
}

#[deprecated]
pub type RTDResetPasswordBuilder = ResetPasswordBuilder;

impl ResetPasswordBuilder {
    pub fn build(&self) -> ResetPassword {
        self.inner.clone()
    }
}

impl AsRef<ResetPassword> for ResetPassword {
    fn as_ref(&self) -> &ResetPassword {
        self
    }
}

impl AsRef<ResetPassword> for ResetPasswordBuilder {
    fn as_ref(&self) -> &ResetPassword {
        &self.inner
    }
}
