use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Deletes saved credentials for all payment provider bots
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteSavedCredentials {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteSavedCredentials {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteSavedCredentials {}

impl DeleteSavedCredentials {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeleteSavedCredentialsBuilder {
        let mut inner = DeleteSavedCredentials::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteSavedCredentials".to_string();

        RTDDeleteSavedCredentialsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDDeleteSavedCredentialsBuilder {
    inner: DeleteSavedCredentials,
}

impl RTDDeleteSavedCredentialsBuilder {
    pub fn build(&self) -> DeleteSavedCredentials {
        self.inner.clone()
    }
}

impl AsRef<DeleteSavedCredentials> for DeleteSavedCredentials {
    fn as_ref(&self) -> &DeleteSavedCredentials {
        self
    }
}

impl AsRef<DeleteSavedCredentials> for RTDDeleteSavedCredentialsBuilder {
    fn as_ref(&self) -> &DeleteSavedCredentials {
        &self.inner
    }
}
