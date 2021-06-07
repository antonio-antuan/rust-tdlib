use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about the origin of a forwarded message
pub trait TDMessageForwardOrigin: Debug + RObject {}

/// Contains information about the origin of a forwarded message
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageForwardOrigin {
    #[doc(hidden)]
    _Default,
    /// The message was originally a post in a channel
    #[serde(rename(
        serialize = "messageForwardOriginChannel",
        deserialize = "messageForwardOriginChannel"
    ))]
    Channel(MessageForwardOriginChannel),
    /// The message was originally sent by an anonymous chat administrator on behalf of the chat
    #[serde(rename(
        serialize = "messageForwardOriginChat",
        deserialize = "messageForwardOriginChat"
    ))]
    Chat(MessageForwardOriginChat),
    /// The message was originally sent by a user, which is hidden by their privacy settings
    #[serde(rename(
        serialize = "messageForwardOriginHiddenUser",
        deserialize = "messageForwardOriginHiddenUser"
    ))]
    HiddenUser(MessageForwardOriginHiddenUser),
    /// The message was originally sent by a known user
    #[serde(rename(
        serialize = "messageForwardOriginUser",
        deserialize = "messageForwardOriginUser"
    ))]
    User(MessageForwardOriginUser),
}

impl Default for MessageForwardOrigin {
    fn default() -> Self {
        MessageForwardOrigin::_Default
    }
}

impl RObject for MessageForwardOrigin {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            MessageForwardOrigin::Channel(t) => t.extra(),
            MessageForwardOrigin::Chat(t) => t.extra(),
            MessageForwardOrigin::HiddenUser(t) => t.extra(),
            MessageForwardOrigin::User(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            MessageForwardOrigin::Channel(t) => t.client_id(),
            MessageForwardOrigin::Chat(t) => t.client_id(),
            MessageForwardOrigin::HiddenUser(t) => t.client_id(),
            MessageForwardOrigin::User(t) => t.client_id(),

            _ => None,
        }
    }
}

impl MessageForwardOrigin {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, MessageForwardOrigin::_Default)
    }
}

impl AsRef<MessageForwardOrigin> for MessageForwardOrigin {
    fn as_ref(&self) -> &MessageForwardOrigin {
        self
    }
}

/// The message was originally a post in a channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageForwardOriginChannel {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat from which the message was originally forwarded
    chat_id: i64,
    /// Message identifier of the original message
    message_id: i64,
    /// Original post author signature
    author_signature: String,
}

impl RObject for MessageForwardOriginChannel {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageForwardOrigin for MessageForwardOriginChannel {}

impl MessageForwardOriginChannel {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageForwardOriginChannelBuilder {
        let mut inner = MessageForwardOriginChannel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageForwardOriginChannelBuilder { inner }
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
pub struct RTDMessageForwardOriginChannelBuilder {
    inner: MessageForwardOriginChannel,
}

impl RTDMessageForwardOriginChannelBuilder {
    pub fn build(&self) -> MessageForwardOriginChannel {
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

impl AsRef<MessageForwardOriginChannel> for MessageForwardOriginChannel {
    fn as_ref(&self) -> &MessageForwardOriginChannel {
        self
    }
}

impl AsRef<MessageForwardOriginChannel> for RTDMessageForwardOriginChannelBuilder {
    fn as_ref(&self) -> &MessageForwardOriginChannel {
        &self.inner
    }
}

/// The message was originally sent by an anonymous chat administrator on behalf of the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageForwardOriginChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat that originally sent the message
    sender_chat_id: i64,
    /// Original message author signature
    author_signature: String,
}

impl RObject for MessageForwardOriginChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageForwardOrigin for MessageForwardOriginChat {}

impl MessageForwardOriginChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageForwardOriginChatBuilder {
        let mut inner = MessageForwardOriginChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageForwardOriginChatBuilder { inner }
    }

    pub fn sender_chat_id(&self) -> i64 {
        self.sender_chat_id
    }

    pub fn author_signature(&self) -> &String {
        &self.author_signature
    }
}

#[doc(hidden)]
pub struct RTDMessageForwardOriginChatBuilder {
    inner: MessageForwardOriginChat,
}

impl RTDMessageForwardOriginChatBuilder {
    pub fn build(&self) -> MessageForwardOriginChat {
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

impl AsRef<MessageForwardOriginChat> for MessageForwardOriginChat {
    fn as_ref(&self) -> &MessageForwardOriginChat {
        self
    }
}

impl AsRef<MessageForwardOriginChat> for RTDMessageForwardOriginChatBuilder {
    fn as_ref(&self) -> &MessageForwardOriginChat {
        &self.inner
    }
}

/// The message was originally sent by a user, which is hidden by their privacy settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageForwardOriginHiddenUser {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the sender
    sender_name: String,
}

impl RObject for MessageForwardOriginHiddenUser {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageForwardOrigin for MessageForwardOriginHiddenUser {}

impl MessageForwardOriginHiddenUser {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageForwardOriginHiddenUserBuilder {
        let mut inner = MessageForwardOriginHiddenUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageForwardOriginHiddenUserBuilder { inner }
    }

    pub fn sender_name(&self) -> &String {
        &self.sender_name
    }
}

#[doc(hidden)]
pub struct RTDMessageForwardOriginHiddenUserBuilder {
    inner: MessageForwardOriginHiddenUser,
}

impl RTDMessageForwardOriginHiddenUserBuilder {
    pub fn build(&self) -> MessageForwardOriginHiddenUser {
        self.inner.clone()
    }

    pub fn sender_name<T: AsRef<str>>(&mut self, sender_name: T) -> &mut Self {
        self.inner.sender_name = sender_name.as_ref().to_string();
        self
    }
}

impl AsRef<MessageForwardOriginHiddenUser> for MessageForwardOriginHiddenUser {
    fn as_ref(&self) -> &MessageForwardOriginHiddenUser {
        self
    }
}

impl AsRef<MessageForwardOriginHiddenUser> for RTDMessageForwardOriginHiddenUserBuilder {
    fn as_ref(&self) -> &MessageForwardOriginHiddenUser {
        &self.inner
    }
}

/// The message was originally sent by a known user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageForwardOriginUser {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the user that originally sent the message
    sender_user_id: i32,
}

impl RObject for MessageForwardOriginUser {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageForwardOrigin for MessageForwardOriginUser {}

impl MessageForwardOriginUser {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageForwardOriginUserBuilder {
        let mut inner = MessageForwardOriginUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageForwardOriginUserBuilder { inner }
    }

    pub fn sender_user_id(&self) -> i32 {
        self.sender_user_id
    }
}

#[doc(hidden)]
pub struct RTDMessageForwardOriginUserBuilder {
    inner: MessageForwardOriginUser,
}

impl RTDMessageForwardOriginUserBuilder {
    pub fn build(&self) -> MessageForwardOriginUser {
        self.inner.clone()
    }

    pub fn sender_user_id(&mut self, sender_user_id: i32) -> &mut Self {
        self.inner.sender_user_id = sender_user_id;
        self
    }
}

impl AsRef<MessageForwardOriginUser> for MessageForwardOriginUser {
    fn as_ref(&self) -> &MessageForwardOriginUser {
        self
    }
}

impl AsRef<MessageForwardOriginUser> for RTDMessageForwardOriginUserBuilder {
    fn as_ref(&self) -> &MessageForwardOriginUser {
        &self.inner
    }
}
