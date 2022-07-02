use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether a participant of an active group call is muted, unmuted, or allowed to unmute themselves
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleGroupCallParticipantIsMuted {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,
    /// Participant identifier

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    participant_id: MessageSender,
    /// Pass true if the user must be muted and false otherwise

    #[serde(default)]
    is_muted: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleGroupCallParticipantIsMuted {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleGroupCallParticipantIsMuted {}

impl ToggleGroupCallParticipantIsMuted {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleGroupCallParticipantIsMutedBuilder {
        let mut inner = ToggleGroupCallParticipantIsMuted::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleGroupCallParticipantIsMuted".to_string();

        ToggleGroupCallParticipantIsMutedBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn participant_id(&self) -> &MessageSender {
        &self.participant_id
    }

    pub fn is_muted(&self) -> bool {
        self.is_muted
    }
}

#[doc(hidden)]
pub struct ToggleGroupCallParticipantIsMutedBuilder {
    inner: ToggleGroupCallParticipantIsMuted,
}

#[deprecated]
pub type RTDToggleGroupCallParticipantIsMutedBuilder = ToggleGroupCallParticipantIsMutedBuilder;

impl ToggleGroupCallParticipantIsMutedBuilder {
    pub fn build(&self) -> ToggleGroupCallParticipantIsMuted {
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

    pub fn is_muted(&mut self, is_muted: bool) -> &mut Self {
        self.inner.is_muted = is_muted;
        self
    }
}

impl AsRef<ToggleGroupCallParticipantIsMuted> for ToggleGroupCallParticipantIsMuted {
    fn as_ref(&self) -> &ToggleGroupCallParticipantIsMuted {
        self
    }
}

impl AsRef<ToggleGroupCallParticipantIsMuted> for ToggleGroupCallParticipantIsMutedBuilder {
    fn as_ref(&self) -> &ToggleGroupCallParticipantIsMuted {
        &self.inner
    }
}
