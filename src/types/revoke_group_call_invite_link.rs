use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Revokes invite link for a group call. Requires groupCall.can_be_managed group call flag
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RevokeGroupCallInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RevokeGroupCallInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RevokeGroupCallInviteLink {}

impl RevokeGroupCallInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RevokeGroupCallInviteLinkBuilder {
        let mut inner = RevokeGroupCallInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "revokeGroupCallInviteLink".to_string();

        RevokeGroupCallInviteLinkBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }
}

#[doc(hidden)]
pub struct RevokeGroupCallInviteLinkBuilder {
    inner: RevokeGroupCallInviteLink,
}

#[deprecated]
pub type RTDRevokeGroupCallInviteLinkBuilder = RevokeGroupCallInviteLinkBuilder;

impl RevokeGroupCallInviteLinkBuilder {
    pub fn build(&self) -> RevokeGroupCallInviteLink {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }
}

impl AsRef<RevokeGroupCallInviteLink> for RevokeGroupCallInviteLink {
    fn as_ref(&self) -> &RevokeGroupCallInviteLink {
        self
    }
}

impl AsRef<RevokeGroupCallInviteLink> for RevokeGroupCallInviteLinkBuilder {
    fn as_ref(&self) -> &RevokeGroupCallInviteLink {
        &self.inner
    }
}
