use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Searches for call messages. Returns the results in reverse chronological order (i. e., in order of decreasing message_id). For optimal performance, the number of returned messages is chosen by TDLib
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchCallMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the message from which to search; use 0 to get results from the last message

    #[serde(default)]
    from_message_id: i64,
    /// The maximum number of messages to be returned; up to 100. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit

    #[serde(default)]
    limit: i32,
    /// If true, returns only messages with missed/declined calls

    #[serde(default)]
    only_missed: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchCallMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchCallMessages {}

impl SearchCallMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchCallMessagesBuilder {
        let mut inner = SearchCallMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchCallMessages".to_string();

        SearchCallMessagesBuilder { inner }
    }

    pub fn from_message_id(&self) -> i64 {
        self.from_message_id
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }

    pub fn only_missed(&self) -> bool {
        self.only_missed
    }
}

#[doc(hidden)]
pub struct SearchCallMessagesBuilder {
    inner: SearchCallMessages,
}

#[deprecated]
pub type RTDSearchCallMessagesBuilder = SearchCallMessagesBuilder;

impl SearchCallMessagesBuilder {
    pub fn build(&self) -> SearchCallMessages {
        self.inner.clone()
    }

    pub fn from_message_id(&mut self, from_message_id: i64) -> &mut Self {
        self.inner.from_message_id = from_message_id;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }

    pub fn only_missed(&mut self, only_missed: bool) -> &mut Self {
        self.inner.only_missed = only_missed;
        self
    }
}

impl AsRef<SearchCallMessages> for SearchCallMessages {
    fn as_ref(&self) -> &SearchCallMessages {
        self
    }
}

impl AsRef<SearchCallMessages> for SearchCallMessagesBuilder {
    fn as_ref(&self) -> &SearchCallMessages {
        &self.inner
    }
}
