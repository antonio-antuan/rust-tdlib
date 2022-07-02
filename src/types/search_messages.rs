use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Searches for messages in all chats except secret chats. Returns the results in reverse chronological order (i.e., in order of decreasing (date, chat_id, message_id)). For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat list in which to search messages; pass null to search in all chats regardless of their chat list. Only Main and Archive chat lists are supported

    #[serde(skip_serializing_if = "ChatList::_is_default")]
    chat_list: ChatList,
    /// Query to search for

    #[serde(default)]
    query: String,
    /// The date of the message starting from which the results need to be fetched. Use 0 or any date in the future to get results from the last message

    #[serde(default)]
    offset_date: i32,
    /// The chat identifier of the last found message, or 0 for the first request

    #[serde(default)]
    offset_chat_id: i64,
    /// The message identifier of the last found message, or 0 for the first request

    #[serde(default)]
    offset_message_id: i64,
    /// The maximum number of messages to be returned; up to 100. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit

    #[serde(default)]
    limit: i32,
    /// Additional filter for messages to search; pass null to search for all messages. Filters searchMessagesFilterMention, searchMessagesFilterUnreadMention, searchMessagesFilterFailedToSend and searchMessagesFilterPinned are unsupported in this function

    #[serde(skip_serializing_if = "SearchMessagesFilter::_is_default")]
    filter: SearchMessagesFilter,
    /// If not 0, the minimum date of the messages to return

    #[serde(default)]
    min_date: i32,
    /// If not 0, the maximum date of the messages to return

    #[serde(default)]
    max_date: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchMessages {}

impl SearchMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesBuilder {
        let mut inner = SearchMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchMessages".to_string();

        SearchMessagesBuilder { inner }
    }

    pub fn chat_list(&self) -> &ChatList {
        &self.chat_list
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn offset_date(&self) -> i32 {
        self.offset_date
    }

    pub fn offset_chat_id(&self) -> i64 {
        self.offset_chat_id
    }

    pub fn offset_message_id(&self) -> i64 {
        self.offset_message_id
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }

    pub fn filter(&self) -> &SearchMessagesFilter {
        &self.filter
    }

    pub fn min_date(&self) -> i32 {
        self.min_date
    }

    pub fn max_date(&self) -> i32 {
        self.max_date
    }
}

#[doc(hidden)]
pub struct SearchMessagesBuilder {
    inner: SearchMessages,
}

#[deprecated]
pub type RTDSearchMessagesBuilder = SearchMessagesBuilder;

impl SearchMessagesBuilder {
    pub fn build(&self) -> SearchMessages {
        self.inner.clone()
    }

    pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
        self.inner.chat_list = chat_list.as_ref().clone();
        self
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }

    pub fn offset_date(&mut self, offset_date: i32) -> &mut Self {
        self.inner.offset_date = offset_date;
        self
    }

    pub fn offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self {
        self.inner.offset_chat_id = offset_chat_id;
        self
    }

    pub fn offset_message_id(&mut self, offset_message_id: i64) -> &mut Self {
        self.inner.offset_message_id = offset_message_id;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }

    pub fn filter<T: AsRef<SearchMessagesFilter>>(&mut self, filter: T) -> &mut Self {
        self.inner.filter = filter.as_ref().clone();
        self
    }

    pub fn min_date(&mut self, min_date: i32) -> &mut Self {
        self.inner.min_date = min_date;
        self
    }

    pub fn max_date(&mut self, max_date: i32) -> &mut Self {
        self.inner.max_date = max_date;
        self
    }
}

impl AsRef<SearchMessages> for SearchMessages {
    fn as_ref(&self) -> &SearchMessages {
        self
    }
}

impl AsRef<SearchMessages> for SearchMessagesBuilder {
    fn as_ref(&self) -> &SearchMessages {
        &self.inner
    }
}
