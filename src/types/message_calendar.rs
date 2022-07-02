use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about found messages, split by days according to the option "utc_time_offset"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCalendar {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Total number of found messages

    #[serde(default)]
    total_count: i32,
    /// Information about messages sent

    #[serde(default)]
    days: Vec<MessageCalendarDay>,
}

impl RObject for MessageCalendar {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageCalendar {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageCalendarBuilder {
        let mut inner = MessageCalendar::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageCalendarBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn days(&self) -> &Vec<MessageCalendarDay> {
        &self.days
    }
}

#[doc(hidden)]
pub struct MessageCalendarBuilder {
    inner: MessageCalendar,
}

#[deprecated]
pub type RTDMessageCalendarBuilder = MessageCalendarBuilder;

impl MessageCalendarBuilder {
    pub fn build(&self) -> MessageCalendar {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn days(&mut self, days: Vec<MessageCalendarDay>) -> &mut Self {
        self.inner.days = days;
        self
    }
}

impl AsRef<MessageCalendar> for MessageCalendar {
    fn as_ref(&self) -> &MessageCalendar {
        self
    }
}

impl AsRef<MessageCalendar> for MessageCalendarBuilder {
    fn as_ref(&self) -> &MessageCalendar {
        &self.inner
    }
}
