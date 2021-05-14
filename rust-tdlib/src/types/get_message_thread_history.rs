use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns messages in a message thread of a message. Can be used only if message.can_get_message_thread == true. Message thread of a channel message is in the channel's linked supergroup. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id). For optimal performance the number of returned messages is chosen by the library
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageThreadHistory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// Message identifier, which thread history needs to be returned
    message_id: i64,
    /// Identifier of the message starting from which history must be fetched; use 0 to get results from the last message
    from_message_id: i64,
    /// Specify 0 to get results from exactly the from_message_id or a negative offset up to 99 to get additionally some newer messages
    offset: i32,
    /// The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater than or equal to offset. Fewer messages may be returned than specified by the limit, even if the end of the message thread history has not been reached
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessageThreadHistory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMessageThreadHistory {}

impl GetMessageThreadHistory {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetMessageThreadHistoryBuilder {
        let mut inner = GetMessageThreadHistory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessageThreadHistory".to_string();

        RTDGetMessageThreadHistoryBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
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
}

#[doc(hidden)]
pub struct RTDGetMessageThreadHistoryBuilder {
    inner: GetMessageThreadHistory,
}

impl RTDGetMessageThreadHistoryBuilder {
    pub fn build(&self) -> GetMessageThreadHistory {
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
}

impl AsRef<GetMessageThreadHistory> for GetMessageThreadHistory {
    fn as_ref(&self) -> &GetMessageThreadHistory {
        self
    }
}

impl AsRef<GetMessageThreadHistory> for RTDGetMessageThreadHistoryBuilder {
    fn as_ref(&self) -> &GetMessageThreadHistory {
        &self.inner
    }
}
