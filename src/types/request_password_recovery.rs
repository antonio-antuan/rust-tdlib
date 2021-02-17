use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Requests to send a password recovery code to an email address that was previously set up
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestPasswordRecovery {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RequestPasswordRecovery {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RequestPasswordRecovery {}

impl RequestPasswordRecovery {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRequestPasswordRecoveryBuilder {
        let mut inner = RequestPasswordRecovery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "requestPasswordRecovery".to_string();

        RTDRequestPasswordRecoveryBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDRequestPasswordRecoveryBuilder {
    inner: RequestPasswordRecovery,
}

impl RTDRequestPasswordRecoveryBuilder {
    pub fn build(&self) -> RequestPasswordRecovery {
        self.inner.clone()
    }
}

impl AsRef<RequestPasswordRecovery> for RequestPasswordRecovery {
    fn as_ref(&self) -> &RequestPasswordRecovery {
        self
    }
}

impl AsRef<RequestPasswordRecovery> for RTDRequestPasswordRecoveryBuilder {
    fn as_ref(&self) -> &RequestPasswordRecovery {
        &self.inner
    }
}
