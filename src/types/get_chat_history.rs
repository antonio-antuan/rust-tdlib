use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id). For optimal performance, the number of returned messages is chosen by TDLib. This is an offline request if only_local is true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatHistory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message starting from which history must be fetched; use 0 to get results from the last message

    #[serde(default)]
    from_message_id: i64,
    /// Specify 0 to get results from exactly the from_message_id or a negative offset up to 99 to get additionally some newer messages

    #[serde(default)]
    offset: i32,
    /// The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater than or equal to offset. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit

    #[serde(default)]
    limit: i32,
    /// If true, returns only messages that are available locally without sending network requests

    #[serde(default)]
    only_local: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatHistory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatHistory {}

impl GetChatHistory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatHistoryBuilder {
        let mut inner = GetChatHistory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatHistory".to_string();

        GetChatHistoryBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
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

    pub fn only_local(&self) -> bool {
        self.only_local
    }
}

#[doc(hidden)]
pub struct GetChatHistoryBuilder {
    inner: GetChatHistory,
}

#[deprecated]
pub type RTDGetChatHistoryBuilder = GetChatHistoryBuilder;

impl GetChatHistoryBuilder {
    pub fn build(&self) -> GetChatHistory {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
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

    pub fn only_local(&mut self, only_local: bool) -> &mut Self {
        self.inner.only_local = only_local;
        self
    }
}

impl AsRef<GetChatHistory> for GetChatHistory {
    fn as_ref(&self) -> &GetChatHistory {
        self
    }
}

impl AsRef<GetChatHistory> for GetChatHistoryBuilder {
    fn as_ref(&self) -> &GetChatHistory {
        &self.inner
    }
}
