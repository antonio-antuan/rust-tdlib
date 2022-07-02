use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a recently speaking participant in a group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallRecentSpeaker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call participant identifier

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    participant_id: MessageSender,
    /// True, is the user has spoken recently

    #[serde(default)]
    is_speaking: bool,
}

impl RObject for GroupCallRecentSpeaker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl GroupCallRecentSpeaker {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GroupCallRecentSpeakerBuilder {
        let mut inner = GroupCallRecentSpeaker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        GroupCallRecentSpeakerBuilder { inner }
    }

    pub fn participant_id(&self) -> &MessageSender {
        &self.participant_id
    }

    pub fn is_speaking(&self) -> bool {
        self.is_speaking
    }
}

#[doc(hidden)]
pub struct GroupCallRecentSpeakerBuilder {
    inner: GroupCallRecentSpeaker,
}

#[deprecated]
pub type RTDGroupCallRecentSpeakerBuilder = GroupCallRecentSpeakerBuilder;

impl GroupCallRecentSpeakerBuilder {
    pub fn build(&self) -> GroupCallRecentSpeaker {
        self.inner.clone()
    }

    pub fn participant_id<T: AsRef<MessageSender>>(&mut self, participant_id: T) -> &mut Self {
        self.inner.participant_id = participant_id.as_ref().clone();
        self
    }

    pub fn is_speaking(&mut self, is_speaking: bool) -> &mut Self {
        self.inner.is_speaking = is_speaking;
        self
    }
}

impl AsRef<GroupCallRecentSpeaker> for GroupCallRecentSpeaker {
    fn as_ref(&self) -> &GroupCallRecentSpeaker {
        self
    }
}

impl AsRef<GroupCallRecentSpeaker> for GroupCallRecentSpeakerBuilder {
    fn as_ref(&self) -> &GroupCallRecentSpeaker {
        &self.inner
    }
}
