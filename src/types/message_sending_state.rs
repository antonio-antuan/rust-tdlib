use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about the sending state of the message
pub trait TDMessageSendingState: Debug + RObject {}

/// Contains information about the sending state of the message
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum MessageSendingState {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The message failed to be sent
    #[serde(rename = "messageSendingStateFailed")]
    Failed(MessageSendingStateFailed),
    /// The message is being sent now, but has not yet been delivered to the server
    #[serde(rename = "messageSendingStatePending")]
    Pending(MessageSendingStatePending),
}

impl RObject for MessageSendingState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            MessageSendingState::Failed(t) => t.extra(),
            MessageSendingState::Pending(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            MessageSendingState::Failed(t) => t.client_id(),
            MessageSendingState::Pending(t) => t.client_id(),

            _ => None,
        }
    }
}

impl MessageSendingState {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, MessageSendingState::_Default)
    }
}

impl AsRef<MessageSendingState> for MessageSendingState {
    fn as_ref(&self) -> &MessageSendingState {
        self
    }
}

/// The message failed to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSendingStateFailed {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The cause of the message sending failure
    error: Error,
    /// True, if the message can be re-sent

    #[serde(default)]
    can_retry: bool,
    /// True, if the message can be re-sent only on behalf of a different sender

    #[serde(default)]
    need_another_sender: bool,
    /// True, if the message can be re-sent only if another quote is chosen in the message that is replied by the given message

    #[serde(default)]
    need_another_reply_quote: bool,
    /// True, if the message can be re-sent only if the message to be replied is removed. This will be done automatically by resendMessages

    #[serde(default)]
    need_drop_reply: bool,
    /// Time left before the message can be re-sent, in seconds. No update is sent when this field changes

    #[serde(default)]
    retry_after: f32,
}

impl RObject for MessageSendingStateFailed {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSendingState for MessageSendingStateFailed {}

impl MessageSendingStateFailed {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSendingStateFailedBuilder {
        let mut inner = MessageSendingStateFailed::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSendingStateFailedBuilder { inner }
    }

    pub fn error(&self) -> &Error {
        &self.error
    }

    pub fn can_retry(&self) -> bool {
        self.can_retry
    }

    pub fn need_another_sender(&self) -> bool {
        self.need_another_sender
    }

    pub fn need_another_reply_quote(&self) -> bool {
        self.need_another_reply_quote
    }

    pub fn need_drop_reply(&self) -> bool {
        self.need_drop_reply
    }

    pub fn retry_after(&self) -> f32 {
        self.retry_after
    }
}

#[doc(hidden)]
pub struct MessageSendingStateFailedBuilder {
    inner: MessageSendingStateFailed,
}

#[deprecated]
pub type RTDMessageSendingStateFailedBuilder = MessageSendingStateFailedBuilder;

impl MessageSendingStateFailedBuilder {
    pub fn build(&self) -> MessageSendingStateFailed {
        self.inner.clone()
    }

    pub fn error<T: AsRef<Error>>(&mut self, error: T) -> &mut Self {
        self.inner.error = error.as_ref().clone();
        self
    }

    pub fn can_retry(&mut self, can_retry: bool) -> &mut Self {
        self.inner.can_retry = can_retry;
        self
    }

    pub fn need_another_sender(&mut self, need_another_sender: bool) -> &mut Self {
        self.inner.need_another_sender = need_another_sender;
        self
    }

    pub fn need_another_reply_quote(&mut self, need_another_reply_quote: bool) -> &mut Self {
        self.inner.need_another_reply_quote = need_another_reply_quote;
        self
    }

    pub fn need_drop_reply(&mut self, need_drop_reply: bool) -> &mut Self {
        self.inner.need_drop_reply = need_drop_reply;
        self
    }

    pub fn retry_after(&mut self, retry_after: f32) -> &mut Self {
        self.inner.retry_after = retry_after;
        self
    }
}

impl AsRef<MessageSendingStateFailed> for MessageSendingStateFailed {
    fn as_ref(&self) -> &MessageSendingStateFailed {
        self
    }
}

impl AsRef<MessageSendingStateFailed> for MessageSendingStateFailedBuilder {
    fn as_ref(&self) -> &MessageSendingStateFailed {
        &self.inner
    }
}

/// The message is being sent now, but has not yet been delivered to the server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSendingStatePending {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Non-persistent message sending identifier, specified by the application

    #[serde(default)]
    sending_id: i32,
}

impl RObject for MessageSendingStatePending {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSendingState for MessageSendingStatePending {}

impl MessageSendingStatePending {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSendingStatePendingBuilder {
        let mut inner = MessageSendingStatePending::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSendingStatePendingBuilder { inner }
    }

    pub fn sending_id(&self) -> i32 {
        self.sending_id
    }
}

#[doc(hidden)]
pub struct MessageSendingStatePendingBuilder {
    inner: MessageSendingStatePending,
}

#[deprecated]
pub type RTDMessageSendingStatePendingBuilder = MessageSendingStatePendingBuilder;

impl MessageSendingStatePendingBuilder {
    pub fn build(&self) -> MessageSendingStatePending {
        self.inner.clone()
    }

    pub fn sending_id(&mut self, sending_id: i32) -> &mut Self {
        self.inner.sending_id = sending_id;
        self
    }
}

impl AsRef<MessageSendingStatePending> for MessageSendingStatePending {
    fn as_ref(&self) -> &MessageSendingStatePending {
        self
    }
}

impl AsRef<MessageSendingStatePending> for MessageSendingStatePendingBuilder {
    fn as_ref(&self) -> &MessageSendingStatePending {
        &self.inner
    }
}
