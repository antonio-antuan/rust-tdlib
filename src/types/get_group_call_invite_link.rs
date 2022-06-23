use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns invite link to a video chat in a public chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetGroupCallInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier
    group_call_id: i32,
    /// Pass true if the invite link needs to contain an invite hash, passing which to joinGroupCall would allow the invited user to unmute themselves. Requires groupCall.can_be_managed group call flag
    can_self_unmute: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetGroupCallInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetGroupCallInviteLink {}

impl GetGroupCallInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetGroupCallInviteLinkBuilder {
        let mut inner = GetGroupCallInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getGroupCallInviteLink".to_string();

        RTDGetGroupCallInviteLinkBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn can_self_unmute(&self) -> bool {
        self.can_self_unmute
    }
}

#[doc(hidden)]
pub struct RTDGetGroupCallInviteLinkBuilder {
    inner: GetGroupCallInviteLink,
}

impl RTDGetGroupCallInviteLinkBuilder {
    pub fn build(&self) -> GetGroupCallInviteLink {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn can_self_unmute(&mut self, can_self_unmute: bool) -> &mut Self {
        self.inner.can_self_unmute = can_self_unmute;
        self
    }
}

impl AsRef<GetGroupCallInviteLink> for GetGroupCallInviteLink {
    fn as_ref(&self) -> &GetGroupCallInviteLink {
        self
    }
}

impl AsRef<GetGroupCallInviteLink> for RTDGetGroupCallInviteLinkBuilder {
    fn as_ref(&self) -> &GetGroupCallInviteLink {
        &self.inner
    }
}
