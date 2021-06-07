use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about the time when a scheduled message will be sent
pub trait TDMessageSchedulingState: Debug + RObject {}

/// Contains information about the time when a scheduled message will be sent
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSchedulingState {
    #[doc(hidden)]
    _Default,
    /// The message will be sent at the specified date
    #[serde(rename(
        serialize = "messageSchedulingStateSendAtDate",
        deserialize = "messageSchedulingStateSendAtDate"
    ))]
    SendAtDate(MessageSchedulingStateSendAtDate),
    /// The message will be sent when the peer will be online. Applicable to private chats only and when the exact online status of the peer is known
    #[serde(rename(
        serialize = "messageSchedulingStateSendWhenOnline",
        deserialize = "messageSchedulingStateSendWhenOnline"
    ))]
    SendWhenOnline(MessageSchedulingStateSendWhenOnline),
}

impl Default for MessageSchedulingState {
    fn default() -> Self {
        MessageSchedulingState::_Default
    }
}

impl RObject for MessageSchedulingState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            MessageSchedulingState::SendAtDate(t) => t.extra(),
            MessageSchedulingState::SendWhenOnline(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            MessageSchedulingState::SendAtDate(t) => t.client_id(),
            MessageSchedulingState::SendWhenOnline(t) => t.client_id(),

            _ => None,
        }
    }
}

impl MessageSchedulingState {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, MessageSchedulingState::_Default)
    }
}

impl AsRef<MessageSchedulingState> for MessageSchedulingState {
    fn as_ref(&self) -> &MessageSchedulingState {
        self
    }
}

/// The message will be sent at the specified date
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSchedulingStateSendAtDate {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Date the message will be sent. The date must be within 367 days in the future
    send_date: i32,
}

impl RObject for MessageSchedulingStateSendAtDate {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSchedulingState for MessageSchedulingStateSendAtDate {}

impl MessageSchedulingStateSendAtDate {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageSchedulingStateSendAtDateBuilder {
        let mut inner = MessageSchedulingStateSendAtDate::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageSchedulingStateSendAtDateBuilder { inner }
    }

    pub fn send_date(&self) -> i32 {
        self.send_date
    }
}

#[doc(hidden)]
pub struct RTDMessageSchedulingStateSendAtDateBuilder {
    inner: MessageSchedulingStateSendAtDate,
}

impl RTDMessageSchedulingStateSendAtDateBuilder {
    pub fn build(&self) -> MessageSchedulingStateSendAtDate {
        self.inner.clone()
    }

    pub fn send_date(&mut self, send_date: i32) -> &mut Self {
        self.inner.send_date = send_date;
        self
    }
}

impl AsRef<MessageSchedulingStateSendAtDate> for MessageSchedulingStateSendAtDate {
    fn as_ref(&self) -> &MessageSchedulingStateSendAtDate {
        self
    }
}

impl AsRef<MessageSchedulingStateSendAtDate> for RTDMessageSchedulingStateSendAtDateBuilder {
    fn as_ref(&self) -> &MessageSchedulingStateSendAtDate {
        &self.inner
    }
}

/// The message will be sent when the peer will be online. Applicable to private chats only and when the exact online status of the peer is known
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSchedulingStateSendWhenOnline {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MessageSchedulingStateSendWhenOnline {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSchedulingState for MessageSchedulingStateSendWhenOnline {}

impl MessageSchedulingStateSendWhenOnline {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageSchedulingStateSendWhenOnlineBuilder {
        let mut inner = MessageSchedulingStateSendWhenOnline::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageSchedulingStateSendWhenOnlineBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDMessageSchedulingStateSendWhenOnlineBuilder {
    inner: MessageSchedulingStateSendWhenOnline,
}

impl RTDMessageSchedulingStateSendWhenOnlineBuilder {
    pub fn build(&self) -> MessageSchedulingStateSendWhenOnline {
        self.inner.clone()
    }
}

impl AsRef<MessageSchedulingStateSendWhenOnline> for MessageSchedulingStateSendWhenOnline {
    fn as_ref(&self) -> &MessageSchedulingStateSendWhenOnline {
        self
    }
}

impl AsRef<MessageSchedulingStateSendWhenOnline>
    for RTDMessageSchedulingStateSendWhenOnlineBuilder
{
    fn as_ref(&self) -> &MessageSchedulingStateSendWhenOnline {
        &self.inner
    }
}
