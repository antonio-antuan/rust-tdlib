use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Searches for ordinary sticker sets by looking for specified query in their title and name. Excludes installed sticker sets from the results
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchStickerSets {
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

impl RObject for SearchStickerSets {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchStickerSets {}

impl SearchStickerSets {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchStickerSetsBuilder {
        let mut inner = SearchStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchStickerSets".to_string();

        SearchStickerSetsBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

#[doc(hidden)]
pub struct SearchStickerSetsBuilder {
    inner: SearchStickerSets,
}

#[deprecated]
pub type RTDSearchStickerSetsBuilder = SearchStickerSetsBuilder;

impl SearchStickerSetsBuilder {
    pub fn build(&self) -> SearchStickerSets {
        self.inner.clone()
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }
}

impl AsRef<SearchStickerSets> for SearchStickerSets {
    fn as_ref(&self) -> &SearchStickerSets {
        self
    }
}

impl AsRef<SearchStickerSets> for SearchStickerSetsBuilder {
    fn as_ref(&self) -> &SearchStickerSets {
        &self.inner
    }
}
