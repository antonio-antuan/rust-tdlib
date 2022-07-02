use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns all active sessions of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetActiveSessions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetActiveSessions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetActiveSessions {}

impl GetActiveSessions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetActiveSessionsBuilder {
        let mut inner = GetActiveSessions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getActiveSessions".to_string();

        GetActiveSessionsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetActiveSessionsBuilder {
    inner: GetActiveSessions,
}

#[deprecated]
pub type RTDGetActiveSessionsBuilder = GetActiveSessionsBuilder;

impl GetActiveSessionsBuilder {
    pub fn build(&self) -> GetActiveSessions {
        self.inner.clone()
    }
}

impl AsRef<GetActiveSessions> for GetActiveSessions {
    fn as_ref(&self) -> &GetActiveSessions {
        self
    }
}

impl AsRef<GetActiveSessions> for GetActiveSessionsBuilder {
    fn as_ref(&self) -> &GetActiveSessions {
        &self.inner
    }
}
