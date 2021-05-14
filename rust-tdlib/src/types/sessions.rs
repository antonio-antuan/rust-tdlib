use crate::errors::*;
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
    sessions: Vec<Session>,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSessionsBuilder {
        let mut inner = Sessions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSessionsBuilder { inner }
    }

    pub fn sessions(&self) -> &Vec<Session> {
        &self.sessions
    }
}

#[doc(hidden)]
pub struct RTDSessionsBuilder {
    inner: Sessions,
}

impl RTDSessionsBuilder {
    pub fn build(&self) -> Sessions {
        self.inner.clone()
    }

    pub fn sessions(&mut self, sessions: Vec<Session>) -> &mut Self {
        self.inner.sessions = sessions;
        self
    }
}

impl AsRef<Sessions> for Sessions {
    fn as_ref(&self) -> &Sessions {
        self
    }
}

impl AsRef<Sessions> for RTDSessionsBuilder {
    fn as_ref(&self) -> &Sessions {
        &self.inner
    }
}
