use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of sessions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sessions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of sessions

    #[serde(default)]
    sessions: Vec<Session>,
    /// Number of days of inactivity before sessions will automatically be terminated; 1-366 days

    #[serde(default)]
    inactive_session_ttl_days: i32,
}

impl RObject for Sessions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Sessions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SessionsBuilder {
        let mut inner = Sessions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SessionsBuilder { inner }
    }

    pub fn sessions(&self) -> &Vec<Session> {
        &self.sessions
    }

    pub fn inactive_session_ttl_days(&self) -> i32 {
        self.inactive_session_ttl_days
    }
}

#[doc(hidden)]
pub struct SessionsBuilder {
    inner: Sessions,
}

#[deprecated]
pub type RTDSessionsBuilder = SessionsBuilder;

impl SessionsBuilder {
    pub fn build(&self) -> Sessions {
        self.inner.clone()
    }

    pub fn sessions(&mut self, sessions: Vec<Session>) -> &mut Self {
        self.inner.sessions = sessions;
        self
    }

    pub fn inactive_session_ttl_days(&mut self, inactive_session_ttl_days: i32) -> &mut Self {
        self.inner.inactive_session_ttl_days = inactive_session_ttl_days;
        self
    }
}

impl AsRef<Sessions> for Sessions {
    fn as_ref(&self) -> &Sessions {
        self
    }
}

impl AsRef<Sessions> for SessionsBuilder {
    fn as_ref(&self) -> &Sessions {
        &self.inner
    }
}
