use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Searches for stickers from public sticker sets that correspond to a given emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// String representation of emoji; must be non-empty
    emoji: String,
    /// The maximum number of stickers to be returned
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchStickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchStickers {}

impl SearchStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchStickersBuilder {
        let mut inner = SearchStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchStickers".to_string();

        RTDSearchStickersBuilder { inner }
    }

    pub fn emoji(&self) -> &String {
        &self.emoji
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDSearchStickersBuilder {
    inner: SearchStickers,
}

impl RTDSearchStickersBuilder {
    pub fn build(&self) -> SearchStickers {
        self.inner.clone()
    }

    pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
        self.inner.emoji = emoji.as_ref().to_string();
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<SearchStickers> for SearchStickers {
    fn as_ref(&self) -> &SearchStickers {
        self
    }
}

impl AsRef<SearchStickers> for RTDSearchStickersBuilder {
    fn as_ref(&self) -> &SearchStickers {
        &self.inner
    }
}
