use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains detailed information about a notification
pub trait TDNotificationType: Debug + RObject {}

/// Contains detailed information about a notification
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NotificationType {
    #[doc(hidden)]
    _Default,
    /// New call was received
    #[serde(rename = "notificationTypeNewCall")]
    NewCall(NotificationTypeNewCall),
    /// New message was received
    #[serde(rename = "notificationTypeNewMessage")]
    NewMessage(Box<NotificationTypeNewMessage>),
    /// New message was received through a push notification
    #[serde(rename = "notificationTypeNewPushMessage")]
    NewPushMessage(Box<NotificationTypeNewPushMessage>),
    /// New secret chat was created
    #[serde(rename = "notificationTypeNewSecretChat")]
    NewSecretChat(NotificationTypeNewSecretChat),
}

impl Default for NotificationType {
    fn default() -> Self {
        NotificationType::_Default
    }
}

impl RObject for NotificationType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            NotificationType::NewCall(t) => t.extra(),
            NotificationType::NewMessage(t) => t.extra(),
            NotificationType::NewPushMessage(t) => t.extra(),
            NotificationType::NewSecretChat(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            NotificationType::NewCall(t) => t.client_id(),
            NotificationType::NewMessage(t) => t.client_id(),
            NotificationType::NewPushMessage(t) => t.client_id(),
            NotificationType::NewSecretChat(t) => t.client_id(),

            _ => None,
        }
    }
}

impl NotificationType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, NotificationType::_Default)
    }
}

impl AsRef<NotificationType> for NotificationType {
    fn as_ref(&self) -> &NotificationType {
        self
    }
}

/// New call was received
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationTypeNewCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Call identifier

    #[serde(default)]
    call_id: i32,
}

impl RObject for NotificationTypeNewCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNotificationType for NotificationTypeNewCall {}

impl NotificationTypeNewCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NotificationTypeNewCallBuilder {
        let mut inner = NotificationTypeNewCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NotificationTypeNewCallBuilder { inner }
    }

    pub fn call_id(&self) -> i32 {
        self.call_id
    }
}

#[doc(hidden)]
pub struct NotificationTypeNewCallBuilder {
    inner: NotificationTypeNewCall,
}

#[deprecated]
pub type RTDNotificationTypeNewCallBuilder = NotificationTypeNewCallBuilder;

impl NotificationTypeNewCallBuilder {
    pub fn build(&self) -> NotificationTypeNewCall {
        self.inner.clone()
    }

    pub fn call_id(&mut self, call_id: i32) -> &mut Self {
        self.inner.call_id = call_id;
        self
    }
}

impl AsRef<NotificationTypeNewCall> for NotificationTypeNewCall {
    fn as_ref(&self) -> &NotificationTypeNewCall {
        self
    }
}

impl AsRef<NotificationTypeNewCall> for NotificationTypeNewCallBuilder {
    fn as_ref(&self) -> &NotificationTypeNewCall {
        &self.inner
    }
}

/// New message was received
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationTypeNewMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The message
    message: Message,
}

impl RObject for NotificationTypeNewMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNotificationType for NotificationTypeNewMessage {}

impl NotificationTypeNewMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NotificationTypeNewMessageBuilder {
        let mut inner = NotificationTypeNewMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NotificationTypeNewMessageBuilder { inner }
    }

    pub fn message(&self) -> &Message {
        &self.message
    }
}

#[doc(hidden)]
pub struct NotificationTypeNewMessageBuilder {
    inner: NotificationTypeNewMessage,
}

#[deprecated]
pub type RTDNotificationTypeNewMessageBuilder = NotificationTypeNewMessageBuilder;

impl NotificationTypeNewMessageBuilder {
    pub fn build(&self) -> NotificationTypeNewMessage {
        self.inner.clone()
    }

    pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
        self.inner.message = message.as_ref().clone();
        self
    }
}

impl AsRef<NotificationTypeNewMessage> for NotificationTypeNewMessage {
    fn as_ref(&self) -> &NotificationTypeNewMessage {
        self
    }
}

impl AsRef<NotificationTypeNewMessage> for NotificationTypeNewMessageBuilder {
    fn as_ref(&self) -> &NotificationTypeNewMessage {
        &self.inner
    }
}

/// New message was received through a push notification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationTypeNewPushMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The message identifier. The message will not be available in the chat history, but the ID can be used in viewMessages, or as reply_to_message_id

    #[serde(default)]
    message_id: i64,
    /// Identifier of the sender of the message. Corresponding user or chat may be inaccessible

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender_id: MessageSender,
    /// Name of the sender

    #[serde(default)]
    sender_name: String,
    /// True, if the message is outgoing

    #[serde(default)]
    is_outgoing: bool,
    /// Push message content

    #[serde(skip_serializing_if = "PushMessageContent::_is_default")]
    content: PushMessageContent,
}

impl RObject for NotificationTypeNewPushMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNotificationType for NotificationTypeNewPushMessage {}

impl NotificationTypeNewPushMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NotificationTypeNewPushMessageBuilder {
        let mut inner = NotificationTypeNewPushMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NotificationTypeNewPushMessageBuilder { inner }
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn sender_id(&self) -> &MessageSender {
        &self.sender_id
    }

    pub fn sender_name(&self) -> &String {
        &self.sender_name
    }

    pub fn is_outgoing(&self) -> bool {
        self.is_outgoing
    }

    pub fn content(&self) -> &PushMessageContent {
        &self.content
    }
}

#[doc(hidden)]
pub struct NotificationTypeNewPushMessageBuilder {
    inner: NotificationTypeNewPushMessage,
}

#[deprecated]
pub type RTDNotificationTypeNewPushMessageBuilder = NotificationTypeNewPushMessageBuilder;

impl NotificationTypeNewPushMessageBuilder {
    pub fn build(&self) -> NotificationTypeNewPushMessage {
        self.inner.clone()
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
        self.inner.sender_id = sender_id.as_ref().clone();
        self
    }

    pub fn sender_name<T: AsRef<str>>(&mut self, sender_name: T) -> &mut Self {
        self.inner.sender_name = sender_name.as_ref().to_string();
        self
    }

    pub fn is_outgoing(&mut self, is_outgoing: bool) -> &mut Self {
        self.inner.is_outgoing = is_outgoing;
        self
    }

    pub fn content<T: AsRef<PushMessageContent>>(&mut self, content: T) -> &mut Self {
        self.inner.content = content.as_ref().clone();
        self
    }
}

impl AsRef<NotificationTypeNewPushMessage> for NotificationTypeNewPushMessage {
    fn as_ref(&self) -> &NotificationTypeNewPushMessage {
        self
    }
}

impl AsRef<NotificationTypeNewPushMessage> for NotificationTypeNewPushMessageBuilder {
    fn as_ref(&self) -> &NotificationTypeNewPushMessage {
        &self.inner
    }
}

/// New secret chat was created
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationTypeNewSecretChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for NotificationTypeNewSecretChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNotificationType for NotificationTypeNewSecretChat {}

impl NotificationTypeNewSecretChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NotificationTypeNewSecretChatBuilder {
        let mut inner = NotificationTypeNewSecretChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NotificationTypeNewSecretChatBuilder { inner }
    }
}

#[doc(hidden)]
pub struct NotificationTypeNewSecretChatBuilder {
    inner: NotificationTypeNewSecretChat,
}

#[deprecated]
pub type RTDNotificationTypeNewSecretChatBuilder = NotificationTypeNewSecretChatBuilder;

impl NotificationTypeNewSecretChatBuilder {
    pub fn build(&self) -> NotificationTypeNewSecretChat {
        self.inner.clone()
    }
}

impl AsRef<NotificationTypeNewSecretChat> for NotificationTypeNewSecretChat {
    fn as_ref(&self) -> &NotificationTypeNewSecretChat {
        self
    }
}

impl AsRef<NotificationTypeNewSecretChat> for NotificationTypeNewSecretChatBuilder {
    fn as_ref(&self) -> &NotificationTypeNewSecretChat {
        &self.inner
    }
}
