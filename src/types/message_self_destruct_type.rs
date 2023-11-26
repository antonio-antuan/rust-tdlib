use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes when a message will be self-destructed
pub trait TDMessageSelfDestructType: Debug + RObject {}

/// Describes when a message will be self-destructed
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum MessageSelfDestructType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The message can be opened only once and will be self-destructed once closed
    #[serde(rename = "messageSelfDestructTypeImmediately")]
    Immediately(MessageSelfDestructTypeImmediately),
    /// The message will be self-destructed in the specified time after its content was opened
    #[serde(rename = "messageSelfDestructTypeTimer")]
    Timer(MessageSelfDestructTypeTimer),
}

impl RObject for MessageSelfDestructType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            MessageSelfDestructType::Immediately(t) => t.extra(),
            MessageSelfDestructType::Timer(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            MessageSelfDestructType::Immediately(t) => t.client_id(),
            MessageSelfDestructType::Timer(t) => t.client_id(),

            _ => None,
        }
    }
}

impl MessageSelfDestructType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, MessageSelfDestructType::_Default)
    }
}

impl AsRef<MessageSelfDestructType> for MessageSelfDestructType {
    fn as_ref(&self) -> &MessageSelfDestructType {
        self
    }
}

/// The message can be opened only once and will be self-destructed once closed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSelfDestructTypeImmediately {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MessageSelfDestructTypeImmediately {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSelfDestructType for MessageSelfDestructTypeImmediately {}

impl MessageSelfDestructTypeImmediately {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSelfDestructTypeImmediatelyBuilder {
        let mut inner = MessageSelfDestructTypeImmediately::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSelfDestructTypeImmediatelyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MessageSelfDestructTypeImmediatelyBuilder {
    inner: MessageSelfDestructTypeImmediately,
}

#[deprecated]
pub type RTDMessageSelfDestructTypeImmediatelyBuilder = MessageSelfDestructTypeImmediatelyBuilder;

impl MessageSelfDestructTypeImmediatelyBuilder {
    pub fn build(&self) -> MessageSelfDestructTypeImmediately {
        self.inner.clone()
    }
}

impl AsRef<MessageSelfDestructTypeImmediately> for MessageSelfDestructTypeImmediately {
    fn as_ref(&self) -> &MessageSelfDestructTypeImmediately {
        self
    }
}

impl AsRef<MessageSelfDestructTypeImmediately> for MessageSelfDestructTypeImmediatelyBuilder {
    fn as_ref(&self) -> &MessageSelfDestructTypeImmediately {
        &self.inner
    }
}

/// The message will be self-destructed in the specified time after its content was opened
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSelfDestructTypeTimer {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The message's self-destruct time, in seconds; must be between 0 and 60 in private chats

    #[serde(default)]
    self_destruct_time: i32,
}

impl RObject for MessageSelfDestructTypeTimer {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSelfDestructType for MessageSelfDestructTypeTimer {}

impl MessageSelfDestructTypeTimer {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSelfDestructTypeTimerBuilder {
        let mut inner = MessageSelfDestructTypeTimer::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSelfDestructTypeTimerBuilder { inner }
    }

    pub fn self_destruct_time(&self) -> i32 {
        self.self_destruct_time
    }
}

#[doc(hidden)]
pub struct MessageSelfDestructTypeTimerBuilder {
    inner: MessageSelfDestructTypeTimer,
}

#[deprecated]
pub type RTDMessageSelfDestructTypeTimerBuilder = MessageSelfDestructTypeTimerBuilder;

impl MessageSelfDestructTypeTimerBuilder {
    pub fn build(&self) -> MessageSelfDestructTypeTimer {
        self.inner.clone()
    }

    pub fn self_destruct_time(&mut self, self_destruct_time: i32) -> &mut Self {
        self.inner.self_destruct_time = self_destruct_time;
        self
    }
}

impl AsRef<MessageSelfDestructTypeTimer> for MessageSelfDestructTypeTimer {
    fn as_ref(&self) -> &MessageSelfDestructTypeTimer {
        self
    }
}

impl AsRef<MessageSelfDestructTypeTimer> for MessageSelfDestructTypeTimerBuilder {
    fn as_ref(&self) -> &MessageSelfDestructTypeTimer {
        &self.inner
    }
}
