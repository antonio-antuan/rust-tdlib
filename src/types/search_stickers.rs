use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Searches for stickers from public sticker sets that correspond to any of the given emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the stickers to return

    #[serde(skip_serializing_if = "StickerType::_is_default")]
    sticker_type: StickerType,
    /// Space-separated list of emoji to search for; must be non-empty

    #[serde(default)]
    emojis: String,
    /// The maximum number of stickers to be returned; 0-100

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchStickersBuilder {
        let mut inner = SearchStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchStickers".to_string();

        SearchStickersBuilder { inner }
    }

    pub fn sticker_type(&self) -> &StickerType {
        &self.sticker_type
    }

    pub fn emojis(&self) -> &String {
        &self.emojis
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct SearchStickersBuilder {
    inner: SearchStickers,
}

#[deprecated]
pub type RTDSearchStickersBuilder = SearchStickersBuilder;

impl SearchStickersBuilder {
    pub fn build(&self) -> SearchStickers {
        self.inner.clone()
    }

    pub fn sticker_type<T: AsRef<StickerType>>(&mut self, sticker_type: T) -> &mut Self {
        self.inner.sticker_type = sticker_type.as_ref().clone();
        self
    }

    pub fn emojis<T: AsRef<str>>(&mut self, emojis: T) -> &mut Self {
        self.inner.emojis = emojis.as_ref().to_string();
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

impl AsRef<SearchStickers> for SearchStickersBuilder {
    fn as_ref(&self) -> &SearchStickers {
        &self.inner
    }
}
