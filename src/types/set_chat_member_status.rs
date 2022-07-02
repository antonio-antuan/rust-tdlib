use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the status of a chat member, needs appropriate privileges. This function is currently not suitable for transferring chat ownership; use transferChatOwnership instead. Use addChatMember or banChatMember if some additional parameters needs to be passed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatMemberStatus {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Member identifier. Chats can be only banned and unbanned in supergroups and channels

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    member_id: MessageSender,
    /// The new status of the member in the chat

    #[serde(skip_serializing_if = "ChatMemberStatus::_is_default")]
    status: ChatMemberStatus,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatMemberStatus {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatMemberStatus {}

impl SetChatMemberStatus {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetChatMemberStatusBuilder {
        let mut inner = SetChatMemberStatus::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatMemberStatus".to_string();

        SetChatMemberStatusBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn member_id(&self) -> &MessageSender {
        &self.member_id
    }

    pub fn status(&self) -> &ChatMemberStatus {
        &self.status
    }
}

#[doc(hidden)]
pub struct SetChatMemberStatusBuilder {
    inner: SetChatMemberStatus,
}

#[deprecated]
pub type RTDSetChatMemberStatusBuilder = SetChatMemberStatusBuilder;

impl SetChatMemberStatusBuilder {
    pub fn build(&self) -> SetChatMemberStatus {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn member_id<T: AsRef<MessageSender>>(&mut self, member_id: T) -> &mut Self {
        self.inner.member_id = member_id.as_ref().clone();
        self
    }

    pub fn status<T: AsRef<ChatMemberStatus>>(&mut self, status: T) -> &mut Self {
        self.inner.status = status.as_ref().clone();
        self
    }
}

impl AsRef<SetChatMemberStatus> for SetChatMemberStatus {
    fn as_ref(&self) -> &SetChatMemberStatus {
        self
    }
}

impl AsRef<SetChatMemberStatus> for SetChatMemberStatusBuilder {
    fn as_ref(&self) -> &SetChatMemberStatus {
        &self.inner
    }
}
