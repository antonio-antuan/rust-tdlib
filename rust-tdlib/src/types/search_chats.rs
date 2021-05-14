use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Searches for the specified query in the title and username of already known chats, this is an offline request. Returns chats in the order seen in the main chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Query to search for. If the query is empty, returns up to 20 recently found chats
    query: String,
    /// The maximum number of chats to be returned
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchChats {}

impl SearchChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchChatsBuilder {
        let mut inner = SearchChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchChats".to_string();

        RTDSearchChatsBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDSearchChatsBuilder {
    inner: SearchChats,
}

impl RTDSearchChatsBuilder {
    pub fn build(&self) -> SearchChats {
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

impl AsRef<SearchChats> for SearchChats {
    fn as_ref(&self) -> &SearchChats {
        self
    }
}

impl AsRef<SearchChats> for RTDSearchChatsBuilder {
    fn as_ref(&self) -> &SearchChats {
        &self.inner
    }
}
