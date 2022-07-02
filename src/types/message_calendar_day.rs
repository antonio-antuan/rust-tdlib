use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about found messages sent on a specific day
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCalendarDay {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Total number of found messages sent on the day

    #[serde(default)]
    total_count: i32,
    /// First message sent on the day
    message: Message,
}

impl RObject for MessageCalendarDay {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageCalendarDay {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageCalendarDayBuilder {
        let mut inner = MessageCalendarDay::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageCalendarDayBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn message(&self) -> &Message {
        &self.message
    }
}

#[doc(hidden)]
pub struct MessageCalendarDayBuilder {
    inner: MessageCalendarDay,
}

#[deprecated]
pub type RTDMessageCalendarDayBuilder = MessageCalendarDayBuilder;

impl MessageCalendarDayBuilder {
    pub fn build(&self) -> MessageCalendarDay {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
        self.inner.message = message.as_ref().clone();
        self
    }
}

impl AsRef<MessageCalendarDay> for MessageCalendarDay {
    fn as_ref(&self) -> &MessageCalendarDay {
        self
    }
}

impl AsRef<MessageCalendarDay> for MessageCalendarDayBuilder {
    fn as_ref(&self) -> &MessageCalendarDay {
        &self.inner
    }
}
