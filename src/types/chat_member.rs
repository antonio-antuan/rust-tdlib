use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A user with information about joining/leaving a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMember {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier of the chat member
    user_id: i32,
    /// Identifier of a user that invited/promoted/banned this member in the chat; 0 if unknown
    inviter_user_id: i32,
    /// Point in time (Unix timestamp) when the user joined the chat
    joined_chat_date: i32,
    /// Status of the member in the chat

    #[serde(skip_serializing_if = "ChatMemberStatus::_is_default")]
    status: ChatMemberStatus,
    /// If the user is a bot, information about the bot; may be null. Can be null even for a bot if the bot is not the chat member
    bot_info: Option<BotInfo>,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatMemberBuilder {
        let mut inner = ChatMember::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatMemberBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn inviter_user_id(&self) -> i32 {
        self.inviter_user_id
    }

    pub fn joined_chat_date(&self) -> i32 {
        self.joined_chat_date
    }

    pub fn status(&self) -> &ChatMemberStatus {
        &self.status
    }

    pub fn bot_info(&self) -> &Option<BotInfo> {
        &self.bot_info
    }
}

#[doc(hidden)]
pub struct RTDChatMemberBuilder {
    inner: ChatMember,
}

impl RTDChatMemberBuilder {
    pub fn build(&self) -> ChatMember {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn inviter_user_id(&mut self, inviter_user_id: i32) -> &mut Self {
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

    pub fn bot_info<T: AsRef<BotInfo>>(&mut self, bot_info: T) -> &mut Self {
        self.inner.bot_info = Some(bot_info.as_ref().clone());
        self
    }
}

impl AsRef<ChatMember> for ChatMember {
    fn as_ref(&self) -> &ChatMember {
        self
    }
}

impl AsRef<ChatMember> for RTDChatMemberBuilder {
    fn as_ref(&self) -> &ChatMember {
        &self.inner
    }
}
