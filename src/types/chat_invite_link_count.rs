use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a chat administrator with a number of active and revoked chat invite links
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatInviteLinkCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Administrator's user identifier

    #[serde(default)]
    user_id: i64,
    /// Number of active invite links

    #[serde(default)]
    invite_link_count: i32,
    /// Number of revoked invite links

    #[serde(default)]
    revoked_invite_link_count: i32,
}

impl RObject for ChatInviteLinkCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatInviteLinkCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatInviteLinkCountBuilder {
        let mut inner = ChatInviteLinkCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatInviteLinkCountBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn invite_link_count(&self) -> i32 {
        self.invite_link_count
    }

    pub fn revoked_invite_link_count(&self) -> i32 {
        self.revoked_invite_link_count
    }
}

#[doc(hidden)]
pub struct ChatInviteLinkCountBuilder {
    inner: ChatInviteLinkCount,
}

#[deprecated]
pub type RTDChatInviteLinkCountBuilder = ChatInviteLinkCountBuilder;

impl ChatInviteLinkCountBuilder {
    pub fn build(&self) -> ChatInviteLinkCount {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn invite_link_count(&mut self, invite_link_count: i32) -> &mut Self {
        self.inner.invite_link_count = invite_link_count;
        self
    }

    pub fn revoked_invite_link_count(&mut self, revoked_invite_link_count: i32) -> &mut Self {
        self.inner.revoked_invite_link_count = revoked_invite_link_count;
        self
    }
}

impl AsRef<ChatInviteLinkCount> for ChatInviteLinkCount {
    fn as_ref(&self) -> &ChatInviteLinkCount {
        self
    }
}

impl AsRef<ChatInviteLinkCount> for ChatInviteLinkCountBuilder {
    fn as_ref(&self) -> &ChatInviteLinkCount {
        &self.inner
    }
}
