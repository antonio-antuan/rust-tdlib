use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a chat member joined a chat via an invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatInviteLinkMember {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier

    #[serde(default)]
    user_id: i64,
    /// Point in time (Unix timestamp) when the user joined the chat

    #[serde(default)]
    joined_chat_date: i32,
    /// User identifier of the chat administrator, approved user join request

    #[serde(default)]
    approver_user_id: i64,
}

impl RObject for ChatInviteLinkMember {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatInviteLinkMember {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatInviteLinkMemberBuilder {
        let mut inner = ChatInviteLinkMember::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatInviteLinkMemberBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn joined_chat_date(&self) -> i32 {
        self.joined_chat_date
    }

    pub fn approver_user_id(&self) -> i64 {
        self.approver_user_id
    }
}

#[doc(hidden)]
pub struct ChatInviteLinkMemberBuilder {
    inner: ChatInviteLinkMember,
}

#[deprecated]
pub type RTDChatInviteLinkMemberBuilder = ChatInviteLinkMemberBuilder;

impl ChatInviteLinkMemberBuilder {
    pub fn build(&self) -> ChatInviteLinkMember {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn joined_chat_date(&mut self, joined_chat_date: i32) -> &mut Self {
        self.inner.joined_chat_date = joined_chat_date;
        self
    }

    pub fn approver_user_id(&mut self, approver_user_id: i64) -> &mut Self {
        self.inner.approver_user_id = approver_user_id;
        self
    }
}

impl AsRef<ChatInviteLinkMember> for ChatInviteLinkMember {
    fn as_ref(&self) -> &ChatInviteLinkMember {
        self
    }
}

impl AsRef<ChatInviteLinkMember> for ChatInviteLinkMemberBuilder {
    fn as_ref(&self) -> &ChatInviteLinkMember {
        &self.inner
    }
}
