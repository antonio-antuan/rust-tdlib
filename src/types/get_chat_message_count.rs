use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns approximate number of messages of the specified type in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatMessageCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat in which to count messages
    chat_id: i64,
    /// Filter for message content; searchMessagesFilterEmpty is unsupported in this function

    #[serde(skip_serializing_if = "SearchMessagesFilter::_is_default")]
    filter: SearchMessagesFilter,
    /// If true, returns count that is available locally without sending network requests, returning 1 if the number of messages is unknown
    return_local: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatMessageCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatMessageCount {}

impl GetChatMessageCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetChatMessageCountBuilder {
        let mut inner = GetChatMessageCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatMessageCount".to_string();

        RTDGetChatMessageCountBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn filter(&self) -> &SearchMessagesFilter {
        &self.filter
    }

    pub fn return_local(&self) -> bool {
        self.return_local
    }
}

#[doc(hidden)]
pub struct RTDGetChatMessageCountBuilder {
    inner: GetChatMessageCount,
}

impl RTDGetChatMessageCountBuilder {
    pub fn build(&self) -> GetChatMessageCount {
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

    pub fn return_local(&mut self, return_local: bool) -> &mut Self {
        self.inner.return_local = return_local;
        self
    }
}

impl AsRef<GetChatMessageCount> for GetChatMessageCount {
    fn as_ref(&self) -> &GetChatMessageCount {
        self
    }
}

impl AsRef<GetChatMessageCount> for RTDGetChatMessageCountBuilder {
    fn as_ref(&self) -> &GetChatMessageCount {
        &self.inner
    }
}
