use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether a group call participant hand is rased
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleGroupCallParticipantIsHandRaised {
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
    /// Pass true if the user's hand needs to be raised. Only self hand can be raised. Requires groupCall.can_be_managed group call flag to lower other's hand

    #[serde(default)]
    is_hand_raised: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleGroupCallParticipantIsHandRaised {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleGroupCallParticipantIsHandRaised {}

impl ToggleGroupCallParticipantIsHandRaised {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleGroupCallParticipantIsHandRaisedBuilder {
        let mut inner = ToggleGroupCallParticipantIsHandRaised::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleGroupCallParticipantIsHandRaised".to_string();

        ToggleGroupCallParticipantIsHandRaisedBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn participant_id(&self) -> &MessageSender {
        &self.participant_id
    }

    pub fn is_hand_raised(&self) -> bool {
        self.is_hand_raised
    }
}

#[doc(hidden)]
pub struct ToggleGroupCallParticipantIsHandRaisedBuilder {
    inner: ToggleGroupCallParticipantIsHandRaised,
}

#[deprecated]
pub type RTDToggleGroupCallParticipantIsHandRaisedBuilder =
    ToggleGroupCallParticipantIsHandRaisedBuilder;

impl ToggleGroupCallParticipantIsHandRaisedBuilder {
    pub fn build(&self) -> ToggleGroupCallParticipantIsHandRaised {
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

    pub fn is_hand_raised(&mut self, is_hand_raised: bool) -> &mut Self {
        self.inner.is_hand_raised = is_hand_raised;
        self
    }
}

impl AsRef<ToggleGroupCallParticipantIsHandRaised> for ToggleGroupCallParticipantIsHandRaised {
    fn as_ref(&self) -> &ToggleGroupCallParticipantIsHandRaised {
        self
    }
}

impl AsRef<ToggleGroupCallParticipantIsHandRaised>
    for ToggleGroupCallParticipantIsHandRaisedBuilder
{
    fn as_ref(&self) -> &ToggleGroupCallParticipantIsHandRaised {
        &self.inner
    }
}
