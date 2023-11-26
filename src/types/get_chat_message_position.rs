use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns approximate 1-based position of a message among messages, which can be found by the specified filter in the chat. Cannot be used in secret chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatMessagePosition {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat in which to find message position

    #[serde(default)]
    chat_id: i64,
    /// Message identifier

    #[serde(default)]
    message_id: i64,
    /// Filter for message content; searchMessagesFilterEmpty, searchMessagesFilterUnreadMention, searchMessagesFilterUnreadReaction, and searchMessagesFilterFailedToSend are unsupported in this function

    #[serde(skip_serializing_if = "SearchMessagesFilter::_is_default")]
    filter: SearchMessagesFilter,
    /// If not 0, only messages in the specified thread will be considered; supergroups only

    #[serde(default)]
    message_thread_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatMessagePosition {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatMessagePosition {}

impl GetChatMessagePosition {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatMessagePositionBuilder {
        let mut inner = GetChatMessagePosition::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatMessagePosition".to_string();

        GetChatMessagePositionBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn filter(&self) -> &SearchMessagesFilter {
        &self.filter
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }
}

#[doc(hidden)]
pub struct GetChatMessagePositionBuilder {
    inner: GetChatMessagePosition,
}

#[deprecated]
pub type RTDGetChatMessagePositionBuilder = GetChatMessagePositionBuilder;

impl GetChatMessagePositionBuilder {
    pub fn build(&self) -> GetChatMessagePosition {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
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

impl AsRef<GetChatMessagePosition> for GetChatMessagePosition {
    fn as_ref(&self) -> &GetChatMessagePosition {
        self
    }
}

impl AsRef<GetChatMessagePosition> for GetChatMessagePositionBuilder {
    fn as_ref(&self) -> &GetChatMessagePosition {
        &self.inner
    }
}
