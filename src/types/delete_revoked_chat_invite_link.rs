use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Deletes revoked chat invite links. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteRevokedChatInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// Invite link to revoke
    invite_link: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteRevokedChatInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteRevokedChatInviteLink {}

impl DeleteRevokedChatInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeleteRevokedChatInviteLinkBuilder {
        let mut inner = DeleteRevokedChatInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteRevokedChatInviteLink".to_string();

        RTDDeleteRevokedChatInviteLinkBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }
}

#[doc(hidden)]
pub struct RTDDeleteRevokedChatInviteLinkBuilder {
    inner: DeleteRevokedChatInviteLink,
}

impl RTDDeleteRevokedChatInviteLinkBuilder {
    pub fn build(&self) -> DeleteRevokedChatInviteLink {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = invite_link.as_ref().to_string();
        self
    }
}

impl AsRef<DeleteRevokedChatInviteLink> for DeleteRevokedChatInviteLink {
    fn as_ref(&self) -> &DeleteRevokedChatInviteLink {
        self
    }
}

impl AsRef<DeleteRevokedChatInviteLink> for RTDDeleteRevokedChatInviteLinkBuilder {
    fn as_ref(&self) -> &DeleteRevokedChatInviteLink {
        &self.inner
    }
}
