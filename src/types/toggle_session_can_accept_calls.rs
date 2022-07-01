use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether a session can accept incoming calls
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleSessionCanAcceptCalls {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Session identifier

    #[serde(deserialize_with = "super::_common::number_from_string")]
    #[serde(default)]
    session_id: i64,
    /// True, if incoming calls can be accepted by the session

    #[serde(default)]
    can_accept_calls: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleSessionCanAcceptCalls {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleSessionCanAcceptCalls {}

impl ToggleSessionCanAcceptCalls {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDToggleSessionCanAcceptCallsBuilder {
        let mut inner = ToggleSessionCanAcceptCalls::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleSessionCanAcceptCalls".to_string();

        RTDToggleSessionCanAcceptCallsBuilder { inner }
    }

    pub fn session_id(&self) -> i64 {
        self.session_id
    }

    pub fn can_accept_calls(&self) -> bool {
        self.can_accept_calls
    }
}

#[doc(hidden)]
pub struct RTDToggleSessionCanAcceptCallsBuilder {
    inner: ToggleSessionCanAcceptCalls,
}

impl RTDToggleSessionCanAcceptCallsBuilder {
    pub fn build(&self) -> ToggleSessionCanAcceptCalls {
        self.inner.clone()
    }

    pub fn session_id(&mut self, session_id: i64) -> &mut Self {
        self.inner.session_id = session_id;
        self
    }

    pub fn can_accept_calls(&mut self, can_accept_calls: bool) -> &mut Self {
        self.inner.can_accept_calls = can_accept_calls;
        self
    }
}

impl AsRef<ToggleSessionCanAcceptCalls> for ToggleSessionCanAcceptCalls {
    fn as_ref(&self) -> &ToggleSessionCanAcceptCalls {
        self
    }
}

impl AsRef<ToggleSessionCanAcceptCalls> for RTDToggleSessionCanAcceptCallsBuilder {
    fn as_ref(&self) -> &ToggleSessionCanAcceptCalls {
        &self.inner
    }
}
