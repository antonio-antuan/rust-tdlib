use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Searches for a specified query in the first name, last name and username of the members of a specified chat. Requires administrator rights in channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchChatMembers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// Query to search for
    query: String,
    /// The maximum number of users to be returned
    limit: i32,
    /// The type of users to return. By default, chatMembersFilterMembers

    #[serde(skip_serializing_if = "ChatMembersFilter::_is_default")]
    filter: ChatMembersFilter,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchChatMembers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchChatMembers {}

impl SearchChatMembers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchChatMembersBuilder {
        let mut inner = SearchChatMembers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchChatMembers".to_string();

        RTDSearchChatMembersBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }

    pub fn filter(&self) -> &ChatMembersFilter {
        &self.filter
    }
}

#[doc(hidden)]
pub struct RTDSearchChatMembersBuilder {
    inner: SearchChatMembers,
}

impl RTDSearchChatMembersBuilder {
    pub fn build(&self) -> SearchChatMembers {
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

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }

    pub fn filter<T: AsRef<ChatMembersFilter>>(&mut self, filter: T) -> &mut Self {
        self.inner.filter = filter.as_ref().clone();
        self
    }
}

impl AsRef<SearchChatMembers> for SearchChatMembers {
    fn as_ref(&self) -> &SearchChatMembers {
        self
    }
}

impl AsRef<SearchChatMembers> for RTDSearchChatMembersBuilder {
    fn as_ref(&self) -> &SearchChatMembers {
        &self.inner
    }
}
