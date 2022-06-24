use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes volume level of a participant of an active group call. If the current user can manage the group call, then the participant's volume level will be changed for all users with the default volume level
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetGroupCallParticipantVolumeLevel {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier
    group_call_id: i32,
    /// Participant identifier

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    participant_id: MessageSender,
    /// New participant's volume level; 1-20000 in hundreds of percents
    volume_level: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetGroupCallParticipantVolumeLevel {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetGroupCallParticipantVolumeLevel {}

impl SetGroupCallParticipantVolumeLevel {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetGroupCallParticipantVolumeLevelBuilder {
        let mut inner = SetGroupCallParticipantVolumeLevel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setGroupCallParticipantVolumeLevel".to_string();

        RTDSetGroupCallParticipantVolumeLevelBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn participant_id(&self) -> &MessageSender {
        &self.participant_id
    }

    pub fn volume_level(&self) -> i32 {
        self.volume_level
    }
}

#[doc(hidden)]
pub struct RTDSetGroupCallParticipantVolumeLevelBuilder {
    inner: SetGroupCallParticipantVolumeLevel,
}

impl RTDSetGroupCallParticipantVolumeLevelBuilder {
    pub fn build(&self) -> SetGroupCallParticipantVolumeLevel {
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

    pub fn volume_level(&mut self, volume_level: i32) -> &mut Self {
        self.inner.volume_level = volume_level;
        self
    }
}

impl AsRef<SetGroupCallParticipantVolumeLevel> for SetGroupCallParticipantVolumeLevel {
    fn as_ref(&self) -> &SetGroupCallParticipantVolumeLevel {
        self
    }
}

impl AsRef<SetGroupCallParticipantVolumeLevel> for RTDSetGroupCallParticipantVolumeLevelBuilder {
    fn as_ref(&self) -> &SetGroupCallParticipantVolumeLevel {
        &self.inner
    }
}