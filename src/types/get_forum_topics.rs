use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns found forum topics in a forum chat. This is a temporary method for getting information about topic list from the server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetForumTopics {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the forum chat

    #[serde(default)]
    chat_id: i64,
    /// Query to search for in the forum topic's name

    #[serde(default)]
    query: String,
    /// The date starting from which the results need to be fetched. Use 0 or any date in the future to get results from the last topic

    #[serde(default)]
    offset_date: i32,
    /// The message identifier of the last message in the last found topic, or 0 for the first request

    #[serde(default)]
    offset_message_id: i64,
    /// The message thread identifier of the last found topic, or 0 for the first request

    #[serde(default)]
    offset_message_thread_id: i64,
    /// The maximum number of forum topics to be returned; up to 100. For optimal performance, the number of returned forum topics is chosen by TDLib and can be smaller than the specified limit

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetForumTopics {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetForumTopics {}

impl GetForumTopics {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetForumTopicsBuilder {
        let mut inner = GetForumTopics::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getForumTopics".to_string();

        GetForumTopicsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn offset_date(&self) -> i32 {
        self.offset_date
    }

    pub fn offset_message_id(&self) -> i64 {
        self.offset_message_id
    }

    pub fn offset_message_thread_id(&self) -> i64 {
        self.offset_message_thread_id
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct GetForumTopicsBuilder {
    inner: GetForumTopics,
}

#[deprecated]
pub type RTDGetForumTopicsBuilder = GetForumTopicsBuilder;

impl GetForumTopicsBuilder {
    pub fn build(&self) -> GetForumTopics {
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

    pub fn offset_date(&mut self, offset_date: i32) -> &mut Self {
        self.inner.offset_date = offset_date;
        self
    }

    pub fn offset_message_id(&mut self, offset_message_id: i64) -> &mut Self {
        self.inner.offset_message_id = offset_message_id;
        self
    }

    pub fn offset_message_thread_id(&mut self, offset_message_thread_id: i64) -> &mut Self {
        self.inner.offset_message_thread_id = offset_message_thread_id;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetForumTopics> for GetForumTopics {
    fn as_ref(&self) -> &GetForumTopics {
        self
    }
}

impl AsRef<GetForumTopics> for GetForumTopicsBuilder {
    fn as_ref(&self) -> &GetForumTopics {
        &self.inner
    }
}
