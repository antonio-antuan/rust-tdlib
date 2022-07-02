use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns sparse positions of messages of the specified type in the chat to be used for shared media scroll implementation. Returns the results in reverse chronological order (i.e., in order of decreasing message_id). Cannot be used in secret chats or with searchMessagesFilterFailedToSend filter without an enabled message database
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatSparseMessagePositions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat in which to return information about message positions

    #[serde(default)]
    chat_id: i64,
    /// Filter for message content. Filters searchMessagesFilterEmpty, searchMessagesFilterMention and searchMessagesFilterUnreadMention are unsupported in this function

    #[serde(skip_serializing_if = "SearchMessagesFilter::_is_default")]
    filter: SearchMessagesFilter,
    /// The message identifier from which to return information about message positions

    #[serde(default)]
    from_message_id: i64,
    /// The expected number of message positions to be returned; 50-2000. A smaller number of positions can be returned, if there are not enough appropriate messages

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatSparseMessagePositions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatSparseMessagePositions {}

impl GetChatSparseMessagePositions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatSparseMessagePositionsBuilder {
        let mut inner = GetChatSparseMessagePositions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatSparseMessagePositions".to_string();

        GetChatSparseMessagePositionsBuilder { inner }
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

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct GetChatSparseMessagePositionsBuilder {
    inner: GetChatSparseMessagePositions,
}

#[deprecated]
pub type RTDGetChatSparseMessagePositionsBuilder = GetChatSparseMessagePositionsBuilder;

impl GetChatSparseMessagePositionsBuilder {
    pub fn build(&self) -> GetChatSparseMessagePositions {
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

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetChatSparseMessagePositions> for GetChatSparseMessagePositions {
    fn as_ref(&self) -> &GetChatSparseMessagePositions {
        self
    }
}

impl AsRef<GetChatSparseMessagePositions> for GetChatSparseMessagePositionsBuilder {
    fn as_ref(&self) -> &GetChatSparseMessagePositions {
        &self.inner
    }
}
