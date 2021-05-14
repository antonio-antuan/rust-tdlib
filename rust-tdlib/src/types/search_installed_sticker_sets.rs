use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Searches for installed sticker sets by looking for specified query in their title and name
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchInstalledStickerSets {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pass true to return mask sticker sets; pass false to return ordinary sticker sets
    is_masks: bool,
    /// Query to search for
    query: String,
    /// The maximum number of sticker sets to return
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchInstalledStickerSets {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchInstalledStickerSets {}

impl SearchInstalledStickerSets {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchInstalledStickerSetsBuilder {
        let mut inner = SearchInstalledStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchInstalledStickerSets".to_string();

        RTDSearchInstalledStickerSetsBuilder { inner }
    }

    pub fn is_masks(&self) -> bool {
        self.is_masks
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDSearchInstalledStickerSetsBuilder {
    inner: SearchInstalledStickerSets,
}

impl RTDSearchInstalledStickerSetsBuilder {
    pub fn build(&self) -> SearchInstalledStickerSets {
        self.inner.clone()
    }

    pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
        self.inner.is_masks = is_masks;
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
}

impl AsRef<SearchInstalledStickerSets> for SearchInstalledStickerSets {
    fn as_ref(&self) -> &SearchInstalledStickerSets {
        self
    }
}

impl AsRef<SearchInstalledStickerSets> for RTDSearchInstalledStickerSetsBuilder {
    fn as_ref(&self) -> &SearchInstalledStickerSets {
        &self.inner
    }
}
