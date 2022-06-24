use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that speaking state of a participant of an active group has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetGroupCallParticipantIsSpeaking {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier
    group_call_id: i32,
    /// Group call participant's synchronization audio source identifier, or 0 for the current user
    audio_source: i32,
    /// Pass true if the user is speaking
    is_speaking: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetGroupCallParticipantIsSpeaking {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetGroupCallParticipantIsSpeaking {}

impl SetGroupCallParticipantIsSpeaking {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetGroupCallParticipantIsSpeakingBuilder {
        let mut inner = SetGroupCallParticipantIsSpeaking::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setGroupCallParticipantIsSpeaking".to_string();

        RTDSetGroupCallParticipantIsSpeakingBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn audio_source(&self) -> i32 {
        self.audio_source
    }

    pub fn is_speaking(&self) -> bool {
        self.is_speaking
    }
}

#[doc(hidden)]
pub struct RTDSetGroupCallParticipantIsSpeakingBuilder {
    inner: SetGroupCallParticipantIsSpeaking,
}

impl RTDSetGroupCallParticipantIsSpeakingBuilder {
    pub fn build(&self) -> SetGroupCallParticipantIsSpeaking {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn audio_source(&mut self, audio_source: i32) -> &mut Self {
        self.inner.audio_source = audio_source;
        self
    }

    pub fn is_speaking(&mut self, is_speaking: bool) -> &mut Self {
        self.inner.is_speaking = is_speaking;
        self
    }
}

impl AsRef<SetGroupCallParticipantIsSpeaking> for SetGroupCallParticipantIsSpeaking {
    fn as_ref(&self) -> &SetGroupCallParticipantIsSpeaking {
        self
    }
}

impl AsRef<SetGroupCallParticipantIsSpeaking> for RTDSetGroupCallParticipantIsSpeakingBuilder {
    fn as_ref(&self) -> &SetGroupCallParticipantIsSpeaking {
        &self.inner
    }
}