use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Call {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Call identifier, not persistent
    id: i32,
    /// Peer user identifier
    user_id: i32,
    /// True, if the call is outgoing
    is_outgoing: bool,
    /// True, if the call is a video call
    is_video: bool,
    /// Call state

    #[serde(skip_serializing_if = "CallState::_is_default")]
    state: CallState,
}

impl RObject for Call {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Call {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallBuilder {
        let mut inner = Call::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn is_outgoing(&self) -> bool {
        self.is_outgoing
    }

    pub fn is_video(&self) -> bool {
        self.is_video
    }

    pub fn state(&self) -> &CallState {
        &self.state
    }
}

#[doc(hidden)]
pub struct RTDCallBuilder {
    inner: Call,
}

impl RTDCallBuilder {
    pub fn build(&self) -> Call {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn is_outgoing(&mut self, is_outgoing: bool) -> &mut Self {
        self.inner.is_outgoing = is_outgoing;
        self
    }

    pub fn is_video(&mut self, is_video: bool) -> &mut Self {
        self.inner.is_video = is_video;
        self
    }

    pub fn state<T: AsRef<CallState>>(&mut self, state: T) -> &mut Self {
        self.inner.state = state.as_ref().clone();
        self
    }
}

impl AsRef<Call> for Call {
    fn as_ref(&self) -> &Call {
        self
    }
}

impl AsRef<Call> for RTDCallBuilder {
    fn as_ref(&self) -> &Call {
        &self.inner
    }
}
