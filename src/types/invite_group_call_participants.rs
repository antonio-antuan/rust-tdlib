use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Invites users to an active group call. Sends a service message of type messageInviteToGroupCall for video chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteGroupCallParticipants {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,
    /// User identifiers. At most 10 users can be invited simultaneously

    #[serde(default)]
    user_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for InviteGroupCallParticipants {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for InviteGroupCallParticipants {}

impl InviteGroupCallParticipants {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InviteGroupCallParticipantsBuilder {
        let mut inner = InviteGroupCallParticipants::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "inviteGroupCallParticipants".to_string();

        InviteGroupCallParticipantsBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn user_ids(&self) -> &Vec<i64> {
        &self.user_ids
    }
}

#[doc(hidden)]
pub struct InviteGroupCallParticipantsBuilder {
    inner: InviteGroupCallParticipants,
}

#[deprecated]
pub type RTDInviteGroupCallParticipantsBuilder = InviteGroupCallParticipantsBuilder;

impl InviteGroupCallParticipantsBuilder {
    pub fn build(&self) -> InviteGroupCallParticipants {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
        self.inner.user_ids = user_ids;
        self
    }
}

impl AsRef<InviteGroupCallParticipants> for InviteGroupCallParticipants {
    fn as_ref(&self) -> &InviteGroupCallParticipants {
        self
    }
}

impl AsRef<InviteGroupCallParticipants> for InviteGroupCallParticipantsBuilder {
    fn as_ref(&self) -> &InviteGroupCallParticipants {
        &self.inner
    }
}
