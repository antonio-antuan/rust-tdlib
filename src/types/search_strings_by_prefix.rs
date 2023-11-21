use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Searches specified query by word prefixes in the provided strings. Returns 0-based positions of strings that matched. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchStringsByPrefix {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The strings to search in for the query

    #[serde(default)]
    strings: Vec<String>,
    /// Query to search for

    #[serde(default)]
    query: String,
    /// The maximum number of objects to return

    #[serde(default)]
    limit: i32,
    /// Pass true to receive no results for an empty query

    #[serde(default)]
    return_none_for_empty_query: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchStringsByPrefix {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchStringsByPrefix {}

impl SearchStringsByPrefix {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchStringsByPrefixBuilder {
        let mut inner = SearchStringsByPrefix::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchStringsByPrefix".to_string();

        SearchStringsByPrefixBuilder { inner }
    }

    pub fn strings(&self) -> &Vec<String> {
        &self.strings
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }

    pub fn return_none_for_empty_query(&self) -> bool {
        self.return_none_for_empty_query
    }
}

#[doc(hidden)]
pub struct SearchStringsByPrefixBuilder {
    inner: SearchStringsByPrefix,
}

#[deprecated]
pub type RTDSearchStringsByPrefixBuilder = SearchStringsByPrefixBuilder;

impl SearchStringsByPrefixBuilder {
    pub fn build(&self) -> SearchStringsByPrefix {
        self.inner.clone()
    }

    pub fn strings(&mut self, strings: Vec<String>) -> &mut Self {
        self.inner.strings = strings;
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

    pub fn return_none_for_empty_query(&mut self, return_none_for_empty_query: bool) -> &mut Self {
        self.inner.return_none_for_empty_query = return_none_for_empty_query;
        self
    }
}

impl AsRef<SearchStringsByPrefix> for SearchStringsByPrefix {
    fn as_ref(&self) -> &SearchStringsByPrefix {
        self
    }
}

impl AsRef<SearchStringsByPrefix> for SearchStringsByPrefixBuilder {
    fn as_ref(&self) -> &SearchStringsByPrefix {
        &self.inner
    }
}
