use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Terminates a session of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TerminateSession {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Session identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    session_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TerminateSession {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TerminateSession {}

impl TerminateSession {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TerminateSessionBuilder {
        let mut inner = TerminateSession::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "terminateSession".to_string();

        TerminateSessionBuilder { inner }
    }

    pub fn session_id(&self) -> i64 {
        self.session_id
    }
}

#[doc(hidden)]
pub struct TerminateSessionBuilder {
    inner: TerminateSession,
}

#[deprecated]
pub type RTDTerminateSessionBuilder = TerminateSessionBuilder;

impl TerminateSessionBuilder {
    pub fn build(&self) -> TerminateSession {
        self.inner.clone()
    }

    pub fn session_id(&mut self, session_id: i64) -> &mut Self {
        self.inner.session_id = session_id;
        self
    }
}

impl AsRef<TerminateSession> for TerminateSession {
    fn as_ref(&self) -> &TerminateSession {
        self
    }
}

impl AsRef<TerminateSession> for TerminateSessionBuilder {
    fn as_ref(&self) -> &TerminateSession {
        &self.inner
    }
}
