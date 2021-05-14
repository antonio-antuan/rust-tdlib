use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Searches for a sticker set by its name
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchStickerSet {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the sticker set
    name: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchStickerSet {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchStickerSet {}

impl SearchStickerSet {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchStickerSetBuilder {
        let mut inner = SearchStickerSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchStickerSet".to_string();

        RTDSearchStickerSetBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct RTDSearchStickerSetBuilder {
    inner: SearchStickerSet,
}

impl RTDSearchStickerSetBuilder {
    pub fn build(&self) -> SearchStickerSet {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }
}

impl AsRef<SearchStickerSet> for SearchStickerSet {
    fn as_ref(&self) -> &SearchStickerSet {
        self
    }
}

impl AsRef<SearchStickerSet> for RTDSearchStickerSetBuilder {
    fn as_ref(&self) -> &SearchStickerSet {
        &self.inner
    }
}
