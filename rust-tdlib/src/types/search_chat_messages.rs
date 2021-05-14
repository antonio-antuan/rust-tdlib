use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Searches for messages with given words in the chat. Returns the results in reverse chronological order, i.e. in order of decreasing message_id. Cannot be used in secret chats with a non-empty query (searchSecretMessages should be used instead), or without an enabled message database. For optimal performance the number of returned messages is chosen by the library
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchChatMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat in which to search messages
    chat_id: i64,
    /// Query to search for
    query: String,
    /// If not null, only messages sent by the specified sender will be returned. Not supported in secret chats

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender: MessageSender,
    /// Identifier of the message starting from which history must be fetched; use 0 to get results from the last message
    from_message_id: i64,
    /// Specify 0 to get results from exactly the from_message_id or a negative offset to get the specified message and some newer messages
    offset: i32,
    /// The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater than offset. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached
    limit: i32,
    /// Filter for message content in the search results

    #[serde(skip_serializing_if = "SearchMessagesFilter::_is_default")]
    filter: SearchMessagesFilter,
    /// If not 0, only messages in the specified thread will be returned; supergroups only
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchChatMessagesBuilder {
        let mut inner = SearchChatMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchChatMessages".to_string();

        RTDSearchChatMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn sender(&self) -> &MessageSender {
        &self.sender
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
pub struct RTDSearchChatMessagesBuilder {
    inner: SearchChatMessages,
}

impl RTDSearchChatMessagesBuilder {
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

    pub fn sender<T: AsRef<MessageSender>>(&mut self, sender: T) -> &mut Self {
        self.inner.sender = sender.as_ref().clone();
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

impl AsRef<SearchChatMessages> for RTDSearchChatMessagesBuilder {
    fn as_ref(&self) -> &SearchChatMessages {
        &self.inner
    }
}
