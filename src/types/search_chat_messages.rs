use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Searches for messages with given words in the chat. Returns the results in reverse chronological order, i.e. in order of decreasing message_id. Cannot be used in secret chats with a non-empty query (searchSecretMessages must be used instead), or without an enabled message database. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchChatMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat in which to search messages

    #[serde(default)]
    chat_id: i64,
    /// Query to search for

    #[serde(default)]
    query: String,
    /// Identifier of the sender of messages to search for; pass null to search for messages from any sender. Not supported in secret chats

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender_id: MessageSender,
    /// Identifier of the message starting from which history must be fetched; use 0 to get results from the last message

    #[serde(default)]
    from_message_id: i64,
    /// Specify 0 to get results from exactly the from_message_id or a negative offset to get the specified message and some newer messages

    #[serde(default)]
    offset: i32,
    /// The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater than offset. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit

    #[serde(default)]
    limit: i32,
    /// Additional filter for messages to search; pass null to search for all messages

    #[serde(skip_serializing_if = "SearchMessagesFilter::_is_default")]
    filter: SearchMessagesFilter,
    /// If not 0, only messages in the specified thread will be returned; supergroups only

    #[serde(default)]
    message_thread_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchChatMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchChatMessages {}

impl SearchChatMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchChatMessagesBuilder {
        let mut inner = SearchChatMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchChatMessages".to_string();

        SearchChatMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn sender_id(&self) -> &MessageSender {
        &self.sender_id
    }

    pub fn from_message_id(&self) -> i64 {
        self.from_message_id
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }

    pub fn filter(&self) -> &SearchMessagesFilter {
        &self.filter
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }
}

#[doc(hidden)]
pub struct SearchChatMessagesBuilder {
    inner: SearchChatMessages,
}

#[deprecated]
pub type RTDSearchChatMessagesBuilder = SearchChatMessagesBuilder;

impl SearchChatMessagesBuilder {
    pub fn build(&self) -> SearchChatMessages {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }

    pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
        self.inner.sender_id = sender_id.as_ref().clone();
        self
    }

    pub fn from_message_id(&mut self, from_message_id: i64) -> &mut Self {
        self.inner.from_message_id = from_message_id;
        self
    }

    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.inner.offset = offset;
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

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }
}

impl AsRef<SearchChatMessages> for SearchChatMessages {
    fn as_ref(&self) -> &SearchChatMessages {
        self
    }
}

impl AsRef<SearchChatMessages> for SearchChatMessagesBuilder {
    fn as_ref(&self) -> &SearchChatMessages {
        &self.inner
    }
}
