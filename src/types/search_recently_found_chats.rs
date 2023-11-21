use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Searches for the specified query in the title and username of up to 50 recently found chats; this is an offline request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchRecentlyFoundChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Query to search for

    #[serde(default)]
    query: String,
    /// The maximum number of chats to be returned

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchRecentlyFoundChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchRecentlyFoundChats {}

impl SearchRecentlyFoundChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchRecentlyFoundChatsBuilder {
        let mut inner = SearchRecentlyFoundChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchRecentlyFoundChats".to_string();

        SearchRecentlyFoundChatsBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct SearchRecentlyFoundChatsBuilder {
    inner: SearchRecentlyFoundChats,
}

#[deprecated]
pub type RTDSearchRecentlyFoundChatsBuilder = SearchRecentlyFoundChatsBuilder;

impl SearchRecentlyFoundChatsBuilder {
    pub fn build(&self) -> SearchRecentlyFoundChats {
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

impl AsRef<SearchRecentlyFoundChats> for SearchRecentlyFoundChats {
    fn as_ref(&self) -> &SearchRecentlyFoundChats {
        self
    }
}

impl AsRef<SearchRecentlyFoundChats> for SearchRecentlyFoundChatsBuilder {
    fn as_ref(&self) -> &SearchRecentlyFoundChats {
        &self.inner
    }
}
