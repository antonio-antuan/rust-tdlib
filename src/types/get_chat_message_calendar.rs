use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about the next messages of the specified type in the chat split by days. Returns the results in reverse chronological order. Can return partial result for the last returned day. Behavior of this method depends on the value of the option "utc_time_offset"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatMessageCalendar {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat in which to return information about messages

    #[serde(default)]
    chat_id: i64,
    /// Filter for message content. Filters searchMessagesFilterEmpty, searchMessagesFilterMention and searchMessagesFilterUnreadMention are unsupported in this function

    #[serde(skip_serializing_if = "SearchMessagesFilter::_is_default")]
    filter: SearchMessagesFilter,
    /// The message identifier from which to return information about messages; use 0 to get results from the last message

    #[serde(default)]
    from_message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatMessageCalendar {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatMessageCalendar {}

impl GetChatMessageCalendar {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatMessageCalendarBuilder {
        let mut inner = GetChatMessageCalendar::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatMessageCalendar".to_string();

        GetChatMessageCalendarBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn filter(&self) -> &SearchMessagesFilter {
        &self.filter
    }

    pub fn from_message_id(&self) -> i64 {
        self.from_message_id
    }
}

#[doc(hidden)]
pub struct GetChatMessageCalendarBuilder {
    inner: GetChatMessageCalendar,
}

#[deprecated]
pub type RTDGetChatMessageCalendarBuilder = GetChatMessageCalendarBuilder;

impl GetChatMessageCalendarBuilder {
    pub fn build(&self) -> GetChatMessageCalendar {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn filter<T: AsRef<SearchMessagesFilter>>(&mut self, filter: T) -> &mut Self {
        self.inner.filter = filter.as_ref().clone();
        self
    }

    pub fn from_message_id(&mut self, from_message_id: i64) -> &mut Self {
        self.inner.from_message_id = from_message_id;
        self
    }
}

impl AsRef<GetChatMessageCalendar> for GetChatMessageCalendar {
    fn as_ref(&self) -> &GetChatMessageCalendar {
        self
    }
}

impl AsRef<GetChatMessageCalendar> for GetChatMessageCalendarBuilder {
    fn as_ref(&self) -> &GetChatMessageCalendar {
        &self.inner
    }
}
