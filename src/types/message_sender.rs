use crate::errors::Result;
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
    #[serde(rename = "messageSenderChat")]
    Chat(MessageSenderChat),
    /// The message was sent by a known user
    #[serde(rename = "messageSenderUser")]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
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

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSenderChatBuilder {
        let mut inner = MessageSenderChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSenderChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct MessageSenderChatBuilder {
    inner: MessageSenderChat,
}

#[deprecated]
pub type RTDMessageSenderChatBuilder = MessageSenderChatBuilder;

impl MessageSenderChatBuilder {
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

impl AsRef<MessageSenderChat> for MessageSenderChatBuilder {
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

    #[serde(default)]
    user_id: i64,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSenderUserBuilder {
        let mut inner = MessageSenderUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSenderUserBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct MessageSenderUserBuilder {
    inner: MessageSenderUser,
}

#[deprecated]
pub type RTDMessageSenderUserBuilder = MessageSenderUserBuilder;

impl MessageSenderUserBuilder {
    pub fn build(&self) -> MessageSenderUser {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<MessageSenderUser> for MessageSenderUser {
    fn as_ref(&self) -> &MessageSenderUser {
        self
    }
}

impl AsRef<MessageSenderUser> for MessageSenderUserBuilder {
    fn as_ref(&self) -> &MessageSenderUser {
        &self.inner
    }
}
