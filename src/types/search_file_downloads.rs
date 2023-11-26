use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Searches for files in the file download list or recently downloaded files from the list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchFileDownloads {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Query to search for; may be empty to return all downloaded files

    #[serde(default)]
    query: String,
    /// Pass true to search only for active downloads, including paused

    #[serde(default)]
    only_active: bool,
    /// Pass true to search only for completed downloads

    #[serde(default)]
    only_completed: bool,
    /// Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results

    #[serde(default)]
    offset: String,
    /// The maximum number of files to be returned

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchFileDownloads {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchFileDownloads {}

impl SearchFileDownloads {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchFileDownloadsBuilder {
        let mut inner = SearchFileDownloads::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchFileDownloads".to_string();

        SearchFileDownloadsBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn only_active(&self) -> bool {
        self.only_active
    }

    pub fn only_completed(&self) -> bool {
        self.only_completed
    }

    pub fn offset(&self) -> &String {
        &self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct SearchFileDownloadsBuilder {
    inner: SearchFileDownloads,
}

#[deprecated]
pub type RTDSearchFileDownloadsBuilder = SearchFileDownloadsBuilder;

impl SearchFileDownloadsBuilder {
    pub fn build(&self) -> SearchFileDownloads {
        self.inner.clone()
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }

    pub fn only_active(&mut self, only_active: bool) -> &mut Self {
        self.inner.only_active = only_active;
        self
    }

    pub fn only_completed(&mut self, only_completed: bool) -> &mut Self {
        self.inner.only_completed = only_completed;
        self
    }

    pub fn offset<T: AsRef<str>>(&mut self, offset: T) -> &mut Self {
        self.inner.offset = offset.as_ref().to_string();
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<SearchFileDownloads> for SearchFileDownloads {
    fn as_ref(&self) -> &SearchFileDownloads {
        self
    }
}

impl AsRef<SearchFileDownloads> for SearchFileDownloadsBuilder {
    fn as_ref(&self) -> &SearchFileDownloads {
        &self.inner
    }
}
