use crate::errors::*;
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
    /// Pass true to change the order of mask sticker sets; pass false to change the order of ordinary sticker sets
    is_masks: bool,
    /// Identifiers of installed sticker sets in the new correct order

    #[serde(deserialize_with = "super::_common::vec_of_i64_from_str")]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDReorderInstalledStickerSetsBuilder {
        let mut inner = ReorderInstalledStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "reorderInstalledStickerSets".to_string();

        RTDReorderInstalledStickerSetsBuilder { inner }
    }

    pub fn is_masks(&self) -> bool {
        self.is_masks
    }

    pub fn sticker_set_ids(&self) -> &Vec<i64> {
        &self.sticker_set_ids
    }
}

#[doc(hidden)]
pub struct RTDReorderInstalledStickerSetsBuilder {
    inner: ReorderInstalledStickerSets,
}

impl RTDReorderInstalledStickerSetsBuilder {
    pub fn build(&self) -> ReorderInstalledStickerSets {
        self.inner.clone()
    }

    pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
        self.inner.is_masks = is_masks;
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

impl AsRef<ReorderInstalledStickerSets> for RTDReorderInstalledStickerSetsBuilder {
    fn as_ref(&self) -> &ReorderInstalledStickerSets {
        &self.inner
    }
}
