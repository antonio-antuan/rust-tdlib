use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Searches for the specified query in the first names, last names and usernames of the known user contacts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchContacts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Query to search for; may be empty to return all contacts

    #[serde(default)]
    query: String,
    /// The maximum number of users to be returned

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchContacts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchContacts {}

impl SearchContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchContactsBuilder {
        let mut inner = SearchContacts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchContacts".to_string();

        SearchContactsBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct SearchContactsBuilder {
    inner: SearchContacts,
}

#[deprecated]
pub type RTDSearchContactsBuilder = SearchContactsBuilder;

impl SearchContactsBuilder {
    pub fn build(&self) -> SearchContacts {
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

impl AsRef<SearchContacts> for SearchContacts {
    fn as_ref(&self) -> &SearchContacts {
        self
    }
}

impl AsRef<SearchContacts> for SearchContactsBuilder {
    fn as_ref(&self) -> &SearchContacts {
        &self.inner
    }
}
