use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Searches for outgoing messages with content of the type messageDocument in all chats except secret chats. Returns the results in reverse chronological order
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchOutgoingDocumentMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Query to search for in document file name and message caption
    query: String,
    /// The maximum number of messages to be returned; up to 100
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchOutgoingDocumentMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchOutgoingDocumentMessages {}

impl SearchOutgoingDocumentMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchOutgoingDocumentMessagesBuilder {
        let mut inner = SearchOutgoingDocumentMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchOutgoingDocumentMessages".to_string();

        RTDSearchOutgoingDocumentMessagesBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDSearchOutgoingDocumentMessagesBuilder {
    inner: SearchOutgoingDocumentMessages,
}

impl RTDSearchOutgoingDocumentMessagesBuilder {
    pub fn build(&self) -> SearchOutgoingDocumentMessages {
        self.inner.clone()
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<SearchOutgoingDocumentMessages> for SearchOutgoingDocumentMessages {
    fn as_ref(&self) -> &SearchOutgoingDocumentMessages {
        self
    }
}

impl AsRef<SearchOutgoingDocumentMessages> for RTDSearchOutgoingDocumentMessagesBuilder {
    fn as_ref(&self) -> &SearchOutgoingDocumentMessages {
        &self.inner
    }
}
