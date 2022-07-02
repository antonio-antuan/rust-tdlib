use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether new participants of a group call can be unmuted only by administrators of the group call. Requires groupCall.can_toggle_mute_new_participants group call flag
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleGroupCallMuteNewParticipants {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,
    /// New value of the mute_new_participants setting

    #[serde(default)]
    mute_new_participants: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleGroupCallMuteNewParticipants {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleGroupCallMuteNewParticipants {}

impl ToggleGroupCallMuteNewParticipants {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleGroupCallMuteNewParticipantsBuilder {
        let mut inner = ToggleGroupCallMuteNewParticipants::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleGroupCallMuteNewParticipants".to_string();

        ToggleGroupCallMuteNewParticipantsBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn mute_new_participants(&self) -> bool {
        self.mute_new_participants
    }
}

#[doc(hidden)]
pub struct ToggleGroupCallMuteNewParticipantsBuilder {
    inner: ToggleGroupCallMuteNewParticipants,
}

#[deprecated]
pub type RTDToggleGroupCallMuteNewParticipantsBuilder = ToggleGroupCallMuteNewParticipantsBuilder;

impl ToggleGroupCallMuteNewParticipantsBuilder {
    pub fn build(&self) -> ToggleGroupCallMuteNewParticipants {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn mute_new_participants(&mut self, mute_new_participants: bool) -> &mut Self {
        self.inner.mute_new_participants = mute_new_participants;
        self
    }
}

impl AsRef<ToggleGroupCallMuteNewParticipants> for ToggleGroupCallMuteNewParticipants {
    fn as_ref(&self) -> &ToggleGroupCallMuteNewParticipants {
        self
    }
}

impl AsRef<ToggleGroupCallMuteNewParticipants> for ToggleGroupCallMuteNewParticipantsBuilder {
    fn as_ref(&self) -> &ToggleGroupCallMuteNewParticipants {
        &self.inner
    }
}
