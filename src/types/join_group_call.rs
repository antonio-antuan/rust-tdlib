use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Joins an active group call. Returns join response payload for tgcalls
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JoinGroupCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,
    /// Identifier of a group call participant, which will be used to join the call; pass null to join as self; video chats only

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    participant_id: MessageSender,
    /// Caller audio channel synchronization source identifier; received from tgcalls

    #[serde(default)]
    audio_source_id: i32,
    /// Group call join payload; received from tgcalls

    #[serde(default)]
    payload: String,
    /// True, if the user's microphone is muted

    #[serde(default)]
    is_muted: bool,
    /// True, if the user's video is enabled

    #[serde(default)]
    is_my_video_enabled: bool,
    /// If non-empty, invite hash to be used to join the group call without being muted by administrators

    #[serde(default)]
    invite_hash: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for JoinGroupCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for JoinGroupCall {}

impl JoinGroupCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> JoinGroupCallBuilder {
        let mut inner = JoinGroupCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "joinGroupCall".to_string();

        JoinGroupCallBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn participant_id(&self) -> &MessageSender {
        &self.participant_id
    }

    pub fn audio_source_id(&self) -> i32 {
        self.audio_source_id
    }

    pub fn payload(&self) -> &String {
        &self.payload
    }

    pub fn is_muted(&self) -> bool {
        self.is_muted
    }

    pub fn is_my_video_enabled(&self) -> bool {
        self.is_my_video_enabled
    }

    pub fn invite_hash(&self) -> &String {
        &self.invite_hash
    }
}

#[doc(hidden)]
pub struct JoinGroupCallBuilder {
    inner: JoinGroupCall,
}

#[deprecated]
pub type RTDJoinGroupCallBuilder = JoinGroupCallBuilder;

impl JoinGroupCallBuilder {
    pub fn build(&self) -> JoinGroupCall {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn participant_id<T: AsRef<MessageSender>>(&mut self, participant_id: T) -> &mut Self {
        self.inner.participant_id = participant_id.as_ref().clone();
        self
    }

    pub fn audio_source_id(&mut self, audio_source_id: i32) -> &mut Self {
        self.inner.audio_source_id = audio_source_id;
        self
    }

    pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
        self.inner.payload = payload.as_ref().to_string();
        self
    }

    pub fn is_muted(&mut self, is_muted: bool) -> &mut Self {
        self.inner.is_muted = is_muted;
        self
    }

    pub fn is_my_video_enabled(&mut self, is_my_video_enabled: bool) -> &mut Self {
        self.inner.is_my_video_enabled = is_my_video_enabled;
        self
    }

    pub fn invite_hash<T: AsRef<str>>(&mut self, invite_hash: T) -> &mut Self {
        self.inner.invite_hash = invite_hash.as_ref().to_string();
        self
    }
}

impl AsRef<JoinGroupCall> for JoinGroupCall {
    fn as_ref(&self) -> &JoinGroupCall {
        self
    }
}

impl AsRef<JoinGroupCall> for JoinGroupCallBuilder {
    fn as_ref(&self) -> &JoinGroupCall {
        &self.inner
    }
}
