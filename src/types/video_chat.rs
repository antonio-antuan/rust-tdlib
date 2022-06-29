use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a video chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier of an active video chat; 0 if none. Full information about the video chat can be received through the method getGroupCall

    #[serde(default)]
    group_call_id: i32,
    /// True, if the video chat has participants

    #[serde(default)]
    has_participants: bool,
    /// Default group call participant identifier to join the video chat; may be null
    default_participant_id: Option<MessageSender>,
}

impl RObject for VideoChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl VideoChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDVideoChatBuilder {
        let mut inner = VideoChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDVideoChatBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn has_participants(&self) -> bool {
        self.has_participants
    }

    pub fn default_participant_id(&self) -> &Option<MessageSender> {
        &self.default_participant_id
    }
}

#[doc(hidden)]
pub struct RTDVideoChatBuilder {
    inner: VideoChat,
}

impl RTDVideoChatBuilder {
    pub fn build(&self) -> VideoChat {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn has_participants(&mut self, has_participants: bool) -> &mut Self {
        self.inner.has_participants = has_participants;
        self
    }

    pub fn default_participant_id<T: AsRef<MessageSender>>(
        &mut self,
        default_participant_id: T,
    ) -> &mut Self {
        self.inner.default_participant_id = Some(default_participant_id.as_ref().clone());
        self
    }
}

impl AsRef<VideoChat> for VideoChat {
    fn as_ref(&self) -> &VideoChat {
        self
    }
}

impl AsRef<VideoChat> for RTDVideoChatBuilder {
    fn as_ref(&self) -> &VideoChat {
        &self.inner
    }
}
