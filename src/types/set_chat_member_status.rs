use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the status of a chat member, needs appropriate privileges. This function is currently not suitable for adding new members to the chat and transferring chat ownership; instead, use addChatMember or transferChatOwnership. The chat member status will not be changed until it has been synchronized with the server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatMemberStatus {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// User identifier
    user_id: i32,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetChatMemberStatusBuilder {
        let mut inner = SetChatMemberStatus::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatMemberStatus".to_string();

        RTDSetChatMemberStatusBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn status(&self) -> &ChatMemberStatus {
        &self.status
    }
}

#[doc(hidden)]
pub struct RTDSetChatMemberStatusBuilder {
    inner: SetChatMemberStatus,
}

impl RTDSetChatMemberStatusBuilder {
    pub fn build(&self) -> SetChatMemberStatus {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
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

impl AsRef<SetChatMemberStatus> for RTDSetChatMemberStatusBuilder {
    fn as_ref(&self) -> &SetChatMemberStatus {
        &self.inner
    }
}
