use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Searches for recently used hashtags by their prefix
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchHashtags {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Hashtag prefix to search for
    prefix: String,
    /// The maximum number of hashtags to be returned
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchHashtags {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchHashtags {}

impl SearchHashtags {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchHashtagsBuilder {
        let mut inner = SearchHashtags::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchHashtags".to_string();

        RTDSearchHashtagsBuilder { inner }
    }

    pub fn prefix(&self) -> &String {
        &self.prefix
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDSearchHashtagsBuilder {
    inner: SearchHashtags,
}

impl RTDSearchHashtagsBuilder {
    pub fn build(&self) -> SearchHashtags {
        self.inner.clone()
    }

    pub fn prefix<T: AsRef<str>>(&mut self, prefix: T) -> &mut Self {
        self.inner.prefix = prefix.as_ref().to_string();
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<SearchHashtags> for SearchHashtags {
    fn as_ref(&self) -> &SearchHashtags {
        self
    }
}

impl AsRef<SearchHashtags> for RTDSearchHashtagsBuilder {
    fn as_ref(&self) -> &SearchHashtags {
        &self.inner
    }
}
