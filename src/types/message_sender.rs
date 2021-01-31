use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains information about the sender of a message
pub trait TDMessageSender: Debug + RObject {}

/// Contains information about the sender of a message
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageSender {
    #[doc(hidden)]
    _Default(()),
    /// The message was sent on behalf of a chat
    Chat(MessageSenderChat),
    /// The message was sent by a known user
    User(MessageSenderUser),
}

impl Default for MessageSender {
    fn default() -> Self {
        MessageSender::_Default(())
    }
}

impl<'de> Deserialize<'de> for MessageSender {
    fn deserialize<D>(deserializer: D) -> Result<MessageSender, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          MessageSender,
          (messageSenderChat, Chat);
          (messageSenderUser, User);

        )(deserializer)
    }
}

impl RObject for MessageSender {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            MessageSender::Chat(t) => t.td_name(),
            MessageSender::User(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            MessageSender::Chat(t) => t.extra(),
            MessageSender::User(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl MessageSender {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, MessageSender::_Default(_))
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Identifier of the chat that sent the message
    chat_id: i64,
}

impl RObject for MessageSenderChat {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "messageSenderChat"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDMessageSender for MessageSenderChat {}

impl MessageSenderChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageSenderChatBuilder {
        let mut inner = MessageSenderChat::default();
        inner.td_name = "messageSenderChat".to_string();
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Identifier of the user that sent the message
    user_id: i32,
}

impl RObject for MessageSenderUser {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "messageSenderUser"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDMessageSender for MessageSenderUser {}

impl MessageSenderUser {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageSenderUserBuilder {
        let mut inner = MessageSenderUser::default();
        inner.td_name = "messageSenderUser".to_string();
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
