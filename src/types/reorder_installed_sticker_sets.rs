use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the order of installed sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReorderInstalledStickerSets {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the sticker sets to reorder

    #[serde(skip_serializing_if = "StickerType::_is_default")]
    sticker_type: StickerType,
    /// Identifiers of installed sticker sets in the new correct order

    #[serde(deserialize_with = "super::_common::vec_of_i64_from_str")]
    #[serde(default)]
    sticker_set_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReorderInstalledStickerSets {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReorderInstalledStickerSets {}

impl ReorderInstalledStickerSets {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReorderInstalledStickerSetsBuilder {
        let mut inner = ReorderInstalledStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "reorderInstalledStickerSets".to_string();

        ReorderInstalledStickerSetsBuilder { inner }
    }

    pub fn sticker_type(&self) -> &StickerType {
        &self.sticker_type
    }

    pub fn sticker_set_ids(&self) -> &Vec<i64> {
        &self.sticker_set_ids
    }
}

#[doc(hidden)]
pub struct ReorderInstalledStickerSetsBuilder {
    inner: ReorderInstalledStickerSets,
}

#[deprecated]
pub type RTDReorderInstalledStickerSetsBuilder = ReorderInstalledStickerSetsBuilder;

impl ReorderInstalledStickerSetsBuilder {
    pub fn build(&self) -> ReorderInstalledStickerSets {
        self.inner.clone()
    }

    pub fn sticker_type<T: AsRef<StickerType>>(&mut self, sticker_type: T) -> &mut Self {
        self.inner.sticker_type = sticker_type.as_ref().clone();
        self
    }

    pub fn sticker_set_ids(&mut self, sticker_set_ids: Vec<i64>) -> &mut Self {
        self.inner.sticker_set_ids = sticker_set_ids;
        self
    }
}

impl AsRef<ReorderInstalledStickerSets> for ReorderInstalledStickerSets {
    fn as_ref(&self) -> &ReorderInstalledStickerSets {
        self
    }
}

impl AsRef<ReorderInstalledStickerSets> for ReorderInstalledStickerSetsBuilder {
    fn as_ref(&self) -> &ReorderInstalledStickerSets {
        &self.inner
    }
}
