use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Terminates all other sessions of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TerminateAllOtherSessions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TerminateAllOtherSessions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TerminateAllOtherSessions {}

impl TerminateAllOtherSessions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TerminateAllOtherSessionsBuilder {
        let mut inner = TerminateAllOtherSessions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "terminateAllOtherSessions".to_string();

        TerminateAllOtherSessionsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TerminateAllOtherSessionsBuilder {
    inner: TerminateAllOtherSessions,
}

#[deprecated]
pub type RTDTerminateAllOtherSessionsBuilder = TerminateAllOtherSessionsBuilder;

impl TerminateAllOtherSessionsBuilder {
    pub fn build(&self) -> TerminateAllOtherSessions {
        self.inner.clone()
    }
}

impl AsRef<TerminateAllOtherSessions> for TerminateAllOtherSessions {
    fn as_ref(&self) -> &TerminateAllOtherSessions {
        self
    }
}

impl AsRef<TerminateAllOtherSessions> for TerminateAllOtherSessionsBuilder {
    fn as_ref(&self) -> &TerminateAllOtherSessions {
        &self.inner
    }
}
