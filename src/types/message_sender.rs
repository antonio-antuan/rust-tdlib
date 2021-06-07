use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about the sender of a message
pub trait TDMessageSender: Debug + RObject {}

/// Contains information about the sender of a message
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSender {
    #[doc(hidden)]
    _Default,
    /// The message was sent on behalf of a chat
    #[serde(rename(serialize = "messageSenderChat", deserialize = "messageSenderChat"))]
    Chat(MessageSenderChat),
    /// The message was sent by a known user
    #[serde(rename(serialize = "messageSenderUser", deserialize = "messageSenderUser"))]
    User(MessageSenderUser),
}

impl Default for MessageSender {
    fn default() -> Self {
        MessageSender::_Default
    }
}

impl RObject for MessageSender {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            MessageSender::Chat(t) => t.extra(),
            MessageSender::User(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            MessageSender::Chat(t) => t.client_id(),
            MessageSender::User(t) => t.client_id(),

            _ => None,
        }
    }
}

impl MessageSender {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, MessageSender::_Default)
    }
}

impl AsRef<MessageSender> for MessageSender {
    fn as_ref(&self) -> &MessageSender {
        self
    }
}

/// The message was sent on behalf of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSenderChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat that sent the message
    chat_id: i64,
}

impl RObject for MessageSenderChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSender for MessageSenderChat {}

impl MessageSenderChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageSenderChatBuilder {
        let mut inner = MessageSenderChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageSenderChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDMessageSenderChatBuilder {
    inner: MessageSenderChat,
}

impl RTDMessageSenderChatBuilder {
    pub fn build(&self) -> MessageSenderChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<MessageSenderChat> for MessageSenderChat {
    fn as_ref(&self) -> &MessageSenderChat {
        self
    }
}

impl AsRef<MessageSenderChat> for RTDMessageSenderChatBuilder {
    fn as_ref(&self) -> &MessageSenderChat {
        &self.inner
    }
}

/// The message was sent by a known user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSenderUser {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the user that sent the message
    user_id: i32,
}

impl RObject for MessageSenderUser {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSender for MessageSenderUser {}

impl MessageSenderUser {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageSenderUserBuilder {
        let mut inner = MessageSenderUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageSenderUserBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct RTDMessageSenderUserBuilder {
    inner: MessageSenderUser,
}

impl RTDMessageSenderUserBuilder {
    pub fn build(&self) -> MessageSenderUser {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<MessageSenderUser> for MessageSenderUser {
    fn as_ref(&self) -> &MessageSenderUser {
        self
    }
}

impl AsRef<MessageSenderUser> for RTDMessageSenderUserBuilder {
    fn as_ref(&self) -> &MessageSenderUser {
        &self.inner
    }
}
