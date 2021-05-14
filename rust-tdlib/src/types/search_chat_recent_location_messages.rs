use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about the recent locations of chat members that were sent to the chat. Returns up to 1 location message per user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchChatRecentLocationMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// The maximum number of messages to be returned
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchChatRecentLocationMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchChatRecentLocationMessages {}

impl SearchChatRecentLocationMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchChatRecentLocationMessagesBuilder {
        let mut inner = SearchChatRecentLocationMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchChatRecentLocationMessages".to_string();

        RTDSearchChatRecentLocationMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDSearchChatRecentLocationMessagesBuilder {
    inner: SearchChatRecentLocationMessages,
}

impl RTDSearchChatRecentLocationMessagesBuilder {
    pub fn build(&self) -> SearchChatRecentLocationMessages {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<SearchChatRecentLocationMessages> for SearchChatRecentLocationMessages {
    fn as_ref(&self) -> &SearchChatRecentLocationMessages {
        self
    }
}

impl AsRef<SearchChatRecentLocationMessages> for RTDSearchChatRecentLocationMessagesBuilder {
    fn as_ref(&self) -> &SearchChatRecentLocationMessages {
        &self.inner
    }
}
