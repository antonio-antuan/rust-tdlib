use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns a list of service actions taken by chat members and administrators in the last 48 hours. Available only for supergroups and channels. Requires administrator rights. Returns results in reverse chronological order (i. e., in order of decreasing event_id)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatEventLog {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Search query by which to filter events

    #[serde(default)]
    query: String,
    /// Identifier of an event from which to return results. Use 0 to get results from the latest events

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    from_event_id: i64,
    /// The maximum number of events to return; up to 100

    #[serde(default)]
    limit: i32,
    /// The types of events to return; pass null to get chat events of all types
    filters: ChatEventLogFilters,
    /// User identifiers by which to filter events. By default, events relating to all users will be returned

    #[serde(default)]
    user_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatEventLog {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatEventLog {}

impl GetChatEventLog {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatEventLogBuilder {
        let mut inner = GetChatEventLog::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatEventLog".to_string();

        GetChatEventLogBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn from_event_id(&self) -> i64 {
        self.from_event_id
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }

    pub fn filters(&self) -> &ChatEventLogFilters {
        &self.filters
    }

    pub fn user_ids(&self) -> &Vec<i64> {
        &self.user_ids
    }
}

#[doc(hidden)]
pub struct GetChatEventLogBuilder {
    inner: GetChatEventLog,
}

#[deprecated]
pub type RTDGetChatEventLogBuilder = GetChatEventLogBuilder;

impl GetChatEventLogBuilder {
    pub fn build(&self) -> GetChatEventLog {
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

    pub fn from_event_id(&mut self, from_event_id: i64) -> &mut Self {
        self.inner.from_event_id = from_event_id;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }

    pub fn filters<T: AsRef<ChatEventLogFilters>>(&mut self, filters: T) -> &mut Self {
        self.inner.filters = filters.as_ref().clone();
        self
    }

    pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
        self.inner.user_ids = user_ids;
        self
    }
}

impl AsRef<GetChatEventLog> for GetChatEventLog {
    fn as_ref(&self) -> &GetChatEventLog {
        self
    }
}

impl AsRef<GetChatEventLog> for GetChatEventLogBuilder {
    fn as_ref(&self) -> &GetChatEventLog {
        &self.inner
    }
}
