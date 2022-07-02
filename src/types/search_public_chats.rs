use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Searches public chats by looking for specified query in their username and title. Currently, only private chats, supergroups and channels can be public. Returns a meaningful number of results. Excludes private chats with contacts and chats from the chat list from the results
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchPublicChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Query to search for

    #[serde(default)]
    query: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchPublicChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchPublicChats {}

impl SearchPublicChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchPublicChatsBuilder {
        let mut inner = SearchPublicChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchPublicChats".to_string();

        SearchPublicChatsBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

#[doc(hidden)]
pub struct SearchPublicChatsBuilder {
    inner: SearchPublicChats,
}

#[deprecated]
pub type RTDSearchPublicChatsBuilder = SearchPublicChatsBuilder;

impl SearchPublicChatsBuilder {
    pub fn build(&self) -> SearchPublicChats {
        self.inner.clone()
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }
}

impl AsRef<SearchPublicChats> for SearchPublicChats {
    fn as_ref(&self) -> &SearchPublicChats {
        self
    }
}

impl AsRef<SearchPublicChats> for SearchPublicChatsBuilder {
    fn as_ref(&self) -> &SearchPublicChats {
        &self.inner
    }
}
