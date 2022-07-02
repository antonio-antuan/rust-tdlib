use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a user or a chat as a member of another chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMember {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat member. Currently, other chats can be only Left or Banned. Only supergroups and channels can have other chats as Left or Banned members and these chats must be supergroups or channels

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    member_id: MessageSender,
    /// Identifier of a user that invited/promoted/banned this member in the chat; 0 if unknown

    #[serde(default)]
    inviter_user_id: i64,
    /// Point in time (Unix timestamp) when the user joined the chat

    #[serde(default)]
    joined_chat_date: i32,
    /// Status of the member in the chat

    #[serde(skip_serializing_if = "ChatMemberStatus::_is_default")]
    status: ChatMemberStatus,
}

impl RObject for ChatMember {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatMember {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatMemberBuilder {
        let mut inner = ChatMember::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatMemberBuilder { inner }
    }

    pub fn member_id(&self) -> &MessageSender {
        &self.member_id
    }

    pub fn inviter_user_id(&self) -> i64 {
        self.inviter_user_id
    }

    pub fn joined_chat_date(&self) -> i32 {
        self.joined_chat_date
    }

    pub fn status(&self) -> &ChatMemberStatus {
        &self.status
    }
}

#[doc(hidden)]
pub struct ChatMemberBuilder {
    inner: ChatMember,
}

#[deprecated]
pub type RTDChatMemberBuilder = ChatMemberBuilder;

impl ChatMemberBuilder {
    pub fn build(&self) -> ChatMember {
        self.inner.clone()
    }

    pub fn member_id<T: AsRef<MessageSender>>(&mut self, member_id: T) -> &mut Self {
        self.inner.member_id = member_id.as_ref().clone();
        self
    }

    pub fn inviter_user_id(&mut self, inviter_user_id: i64) -> &mut Self {
        self.inner.inviter_user_id = inviter_user_id;
        self
    }

    pub fn joined_chat_date(&mut self, joined_chat_date: i32) -> &mut Self {
        self.inner.joined_chat_date = joined_chat_date;
        self
    }

    pub fn status<T: AsRef<ChatMemberStatus>>(&mut self, status: T) -> &mut Self {
        self.inner.status = status.as_ref().clone();
        self
    }
}

impl AsRef<ChatMember> for ChatMember {
    fn as_ref(&self) -> &ChatMember {
        self
    }
}

impl AsRef<ChatMember> for ChatMemberBuilder {
    fn as_ref(&self) -> &ChatMember {
        &self.inner
    }
}
