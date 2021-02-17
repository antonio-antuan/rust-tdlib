use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Searches for messages in secret chats. Returns the results in reverse chronological order. For optimal performance the number of returned messages is chosen by the library
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchSecretMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat in which to search. Specify 0 to search in all secret chats
    chat_id: i64,
    /// Query to search for. If empty, searchChatMessages should be used instead
    query: String,
    /// Offset of the first entry to return as received from the previous request; use empty string to get first chunk of results
    offset: String,
    /// The maximum number of messages to be returned; up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached
    limit: i32,
    /// A filter for message content in the search results

    #[serde(skip_serializing_if = "SearchMessagesFilter::_is_default")]
    filter: SearchMessagesFilter,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchSecretMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchSecretMessages {}

impl SearchSecretMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchSecretMessagesBuilder {
        let mut inner = SearchSecretMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchSecretMessages".to_string();

        RTDSearchSecretMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn offset(&self) -> &String {
        &self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }

    pub fn filter(&self) -> &SearchMessagesFilter {
        &self.filter
    }
}

#[doc(hidden)]
pub struct RTDSearchSecretMessagesBuilder {
    inner: SearchSecretMessages,
}

impl RTDSearchSecretMessagesBuilder {
    pub fn build(&self) -> SearchSecretMessages {
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

    pub fn offset<T: AsRef<str>>(&mut self, offset: T) -> &mut Self {
        self.inner.offset = offset.as_ref().to_string();
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
}

impl AsRef<SearchSecretMessages> for SearchSecretMessages {
    fn as_ref(&self) -> &SearchSecretMessages {
        self
    }
}

impl AsRef<SearchSecretMessages> for RTDSearchSecretMessagesBuilder {
    fn as_ref(&self) -> &SearchSecretMessages {
        &self.inner
    }
}
