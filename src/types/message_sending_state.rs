use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about the sending state of the message
pub trait TDMessageSendingState: Debug + RObject {}

/// Contains information about the sending state of the message
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSendingState {
    #[doc(hidden)]
    _Default,
    /// The message failed to be sent
    #[serde(rename = "messageSendingStateFailed")]
    Failed(MessageSendingStateFailed),
    /// The message is being sent now, but has not yet been delivered to the server
    #[serde(rename = "messageSendingStatePending")]
    Pending(MessageSendingStatePending),
}

impl Default for MessageSendingState {
    fn default() -> Self {
        MessageSendingState::_Default
    }
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
    /// An error code; 0 if unknown

    #[serde(default)]
    error_code: i32,
    /// Error message

    #[serde(default)]
    error_message: String,
    /// True, if the message can be re-sent

    #[serde(default)]
    can_retry: bool,
    /// True, if the message can be re-sent only on behalf of a different sender

    #[serde(default)]
    need_another_sender: bool,
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

    pub fn error_code(&self) -> i32 {
        self.error_code
    }

    pub fn error_message(&self) -> &String {
        &self.error_message
    }

    pub fn can_retry(&self) -> bool {
        self.can_retry
    }

    pub fn need_another_sender(&self) -> bool {
        self.need_another_sender
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

    pub fn error_code(&mut self, error_code: i32) -> &mut Self {
        self.inner.error_code = error_code;
        self
    }

    pub fn error_message<T: AsRef<str>>(&mut self, error_message: T) -> &mut Self {
        self.inner.error_message = error_message.as_ref().to_string();
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
