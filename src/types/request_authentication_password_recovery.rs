use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Requests to send a password recovery code to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestAuthenticationPasswordRecovery {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RequestAuthenticationPasswordRecovery {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RequestAuthenticationPasswordRecovery {}

impl RequestAuthenticationPasswordRecovery {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RequestAuthenticationPasswordRecoveryBuilder {
        let mut inner = RequestAuthenticationPasswordRecovery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "requestAuthenticationPasswordRecovery".to_string();

        RequestAuthenticationPasswordRecoveryBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RequestAuthenticationPasswordRecoveryBuilder {
    inner: RequestAuthenticationPasswordRecovery,
}

#[deprecated]
pub type RTDRequestAuthenticationPasswordRecoveryBuilder =
    RequestAuthenticationPasswordRecoveryBuilder;

impl RequestAuthenticationPasswordRecoveryBuilder {
    pub fn build(&self) -> RequestAuthenticationPasswordRecovery {
        self.inner.clone()
    }
}

impl AsRef<RequestAuthenticationPasswordRecovery> for RequestAuthenticationPasswordRecovery {
    fn as_ref(&self) -> &RequestAuthenticationPasswordRecovery {
        self
    }
}

impl AsRef<RequestAuthenticationPasswordRecovery> for RequestAuthenticationPasswordRecoveryBuilder {
    fn as_ref(&self) -> &RequestAuthenticationPasswordRecovery {
        &self.inner
    }
}
