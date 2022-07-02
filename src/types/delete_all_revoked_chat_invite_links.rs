use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Deletes all revoked chat invite links created by a given chat administrator. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteAllRevokedChatInviteLinks {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// User identifier of a chat administrator, which links will be deleted. Must be an identifier of the current user for non-owner

    #[serde(default)]
    creator_user_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteAllRevokedChatInviteLinks {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteAllRevokedChatInviteLinks {}

impl DeleteAllRevokedChatInviteLinks {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeleteAllRevokedChatInviteLinksBuilder {
        let mut inner = DeleteAllRevokedChatInviteLinks::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteAllRevokedChatInviteLinks".to_string();

        DeleteAllRevokedChatInviteLinksBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn creator_user_id(&self) -> i64 {
        self.creator_user_id
    }
}

#[doc(hidden)]
pub struct DeleteAllRevokedChatInviteLinksBuilder {
    inner: DeleteAllRevokedChatInviteLinks,
}

#[deprecated]
pub type RTDDeleteAllRevokedChatInviteLinksBuilder = DeleteAllRevokedChatInviteLinksBuilder;

impl DeleteAllRevokedChatInviteLinksBuilder {
    pub fn build(&self) -> DeleteAllRevokedChatInviteLinks {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn creator_user_id(&mut self, creator_user_id: i64) -> &mut Self {
        self.inner.creator_user_id = creator_user_id;
        self
    }
}

impl AsRef<DeleteAllRevokedChatInviteLinks> for DeleteAllRevokedChatInviteLinks {
    fn as_ref(&self) -> &DeleteAllRevokedChatInviteLinks {
        self
    }
}

impl AsRef<DeleteAllRevokedChatInviteLinks> for DeleteAllRevokedChatInviteLinksBuilder {
    fn as_ref(&self) -> &DeleteAllRevokedChatInviteLinks {
        &self.inner
    }
}
