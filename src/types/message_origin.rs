use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about the origin of a message
pub trait TDMessageOrigin: Debug + RObject {}

/// Contains information about the origin of a message
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum MessageOrigin {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The message was originally a post in a channel
    #[serde(rename = "messageOriginChannel")]
    Channel(MessageOriginChannel),
    /// The message was originally sent on behalf of a chat
    #[serde(rename = "messageOriginChat")]
    Chat(MessageOriginChat),
    /// The message was originally sent by a user, which is hidden by their privacy settings
    #[serde(rename = "messageOriginHiddenUser")]
    HiddenUser(MessageOriginHiddenUser),
    /// The message was originally sent by a known user
    #[serde(rename = "messageOriginUser")]
    User(MessageOriginUser),
}

impl RObject for MessageOrigin {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            MessageOrigin::Channel(t) => t.extra(),
            MessageOrigin::Chat(t) => t.extra(),
            MessageOrigin::HiddenUser(t) => t.extra(),
            MessageOrigin::User(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            MessageOrigin::Channel(t) => t.client_id(),
            MessageOrigin::Chat(t) => t.client_id(),
            MessageOrigin::HiddenUser(t) => t.client_id(),
            MessageOrigin::User(t) => t.client_id(),

            _ => None,
        }
    }
}

impl MessageOrigin {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, MessageOrigin::_Default)
    }
}

impl AsRef<MessageOrigin> for MessageOrigin {
    fn as_ref(&self) -> &MessageOrigin {
        self
    }
}

/// The message was originally a post in a channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageOriginChannel {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the channel chat to which the message was originally sent

    #[serde(default)]
    chat_id: i64,
    /// Message identifier of the original message

    #[serde(default)]
    message_id: i64,
    /// Original post author signature

    #[serde(default)]
    author_signature: String,
}

impl RObject for MessageOriginChannel {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageOrigin for MessageOriginChannel {}

impl MessageOriginChannel {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageOriginChannelBuilder {
        let mut inner = MessageOriginChannel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageOriginChannelBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn author_signature(&self) -> &String {
        &self.author_signature
    }
}

#[doc(hidden)]
pub struct MessageOriginChannelBuilder {
    inner: MessageOriginChannel,
}

#[deprecated]
pub type RTDMessageOriginChannelBuilder = MessageOriginChannelBuilder;

impl MessageOriginChannelBuilder {
    pub fn build(&self) -> MessageOriginChannel {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn author_signature<T: AsRef<str>>(&mut self, author_signature: T) -> &mut Self {
        self.inner.author_signature = author_signature.as_ref().to_string();
        self
    }
}

impl AsRef<MessageOriginChannel> for MessageOriginChannel {
    fn as_ref(&self) -> &MessageOriginChannel {
        self
    }
}

impl AsRef<MessageOriginChannel> for MessageOriginChannelBuilder {
    fn as_ref(&self) -> &MessageOriginChannel {
        &self.inner
    }
}

/// The message was originally sent on behalf of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageOriginChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat that originally sent the message

    #[serde(default)]
    sender_chat_id: i64,
    /// For messages originally sent by an anonymous chat administrator, original message author signature

    #[serde(default)]
    author_signature: String,
}

impl RObject for MessageOriginChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageOrigin for MessageOriginChat {}

impl MessageOriginChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageOriginChatBuilder {
        let mut inner = MessageOriginChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageOriginChatBuilder { inner }
    }

    pub fn sender_chat_id(&self) -> i64 {
        self.sender_chat_id
    }

    pub fn author_signature(&self) -> &String {
        &self.author_signature
    }
}

#[doc(hidden)]
pub struct MessageOriginChatBuilder {
    inner: MessageOriginChat,
}

#[deprecated]
pub type RTDMessageOriginChatBuilder = MessageOriginChatBuilder;

impl MessageOriginChatBuilder {
    pub fn build(&self) -> MessageOriginChat {
        self.inner.clone()
    }

    pub fn sender_chat_id(&mut self, sender_chat_id: i64) -> &mut Self {
        self.inner.sender_chat_id = sender_chat_id;
        self
    }

    pub fn author_signature<T: AsRef<str>>(&mut self, author_signature: T) -> &mut Self {
        self.inner.author_signature = author_signature.as_ref().to_string();
        self
    }
}

impl AsRef<MessageOriginChat> for MessageOriginChat {
    fn as_ref(&self) -> &MessageOriginChat {
        self
    }
}

impl AsRef<MessageOriginChat> for MessageOriginChatBuilder {
    fn as_ref(&self) -> &MessageOriginChat {
        &self.inner
    }
}

/// The message was originally sent by a user, which is hidden by their privacy settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageOriginHiddenUser {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the sender

    #[serde(default)]
    sender_name: String,
}

impl RObject for MessageOriginHiddenUser {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageOrigin for MessageOriginHiddenUser {}

impl MessageOriginHiddenUser {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageOriginHiddenUserBuilder {
        let mut inner = MessageOriginHiddenUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageOriginHiddenUserBuilder { inner }
    }

    pub fn sender_name(&self) -> &String {
        &self.sender_name
    }
}

#[doc(hidden)]
pub struct MessageOriginHiddenUserBuilder {
    inner: MessageOriginHiddenUser,
}

#[deprecated]
pub type RTDMessageOriginHiddenUserBuilder = MessageOriginHiddenUserBuilder;

impl MessageOriginHiddenUserBuilder {
    pub fn build(&self) -> MessageOriginHiddenUser {
        self.inner.clone()
    }

    pub fn sender_name<T: AsRef<str>>(&mut self, sender_name: T) -> &mut Self {
        self.inner.sender_name = sender_name.as_ref().to_string();
        self
    }
}

impl AsRef<MessageOriginHiddenUser> for MessageOriginHiddenUser {
    fn as_ref(&self) -> &MessageOriginHiddenUser {
        self
    }
}

impl AsRef<MessageOriginHiddenUser> for MessageOriginHiddenUserBuilder {
    fn as_ref(&self) -> &MessageOriginHiddenUser {
        &self.inner
    }
}

/// The message was originally sent by a known user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageOriginUser {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the user that originally sent the message

    #[serde(default)]
    sender_user_id: i64,
}

impl RObject for MessageOriginUser {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageOrigin for MessageOriginUser {}

impl MessageOriginUser {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageOriginUserBuilder {
        let mut inner = MessageOriginUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageOriginUserBuilder { inner }
    }

    pub fn sender_user_id(&self) -> i64 {
        self.sender_user_id
    }
}

#[doc(hidden)]
pub struct MessageOriginUserBuilder {
    inner: MessageOriginUser,
}

#[deprecated]
pub type RTDMessageOriginUserBuilder = MessageOriginUserBuilder;

impl MessageOriginUserBuilder {
    pub fn build(&self) -> MessageOriginUser {
        self.inner.clone()
    }

    pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
        self.inner.sender_user_id = sender_user_id;
        self
    }
}

impl AsRef<MessageOriginUser> for MessageOriginUser {
    fn as_ref(&self) -> &MessageOriginUser {
        self
    }
}

impl AsRef<MessageOriginUser> for MessageOriginUserBuilder {
    fn as_ref(&self) -> &MessageOriginUser {
        &self.inner
    }
}
