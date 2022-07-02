use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes default participant identifier, on whose behalf a video chat in the chat will be joined
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetVideoChatDefaultParticipant {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Default group call participant identifier to join the video chats

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    default_participant_id: MessageSender,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetVideoChatDefaultParticipant {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetVideoChatDefaultParticipant {}

impl SetVideoChatDefaultParticipant {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetVideoChatDefaultParticipantBuilder {
        let mut inner = SetVideoChatDefaultParticipant::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setVideoChatDefaultParticipant".to_string();

        SetVideoChatDefaultParticipantBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn default_participant_id(&self) -> &MessageSender {
        &self.default_participant_id
    }
}

#[doc(hidden)]
pub struct SetVideoChatDefaultParticipantBuilder {
    inner: SetVideoChatDefaultParticipant,
}

#[deprecated]
pub type RTDSetVideoChatDefaultParticipantBuilder = SetVideoChatDefaultParticipantBuilder;

impl SetVideoChatDefaultParticipantBuilder {
    pub fn build(&self) -> SetVideoChatDefaultParticipant {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn default_participant_id<T: AsRef<MessageSender>>(
        &mut self,
        default_participant_id: T,
    ) -> &mut Self {
        self.inner.default_participant_id = default_participant_id.as_ref().clone();
        self
    }
}

impl AsRef<SetVideoChatDefaultParticipant> for SetVideoChatDefaultParticipant {
    fn as_ref(&self) -> &SetVideoChatDefaultParticipant {
        self
    }
}

impl AsRef<SetVideoChatDefaultParticipant> for SetVideoChatDefaultParticipantBuilder {
    fn as_ref(&self) -> &SetVideoChatDefaultParticipant {
        &self.inner
    }
}
