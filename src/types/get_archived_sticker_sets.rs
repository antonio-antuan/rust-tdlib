use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns a list of archived sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetArchivedStickerSets {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pass true to return mask stickers sets; pass false to return ordinary sticker sets
    is_masks: bool,
    /// Identifier of the sticker set from which to return the result

    #[serde(deserialize_with = "super::_common::number_from_string")]
    offset_sticker_set_id: i64,
    /// The maximum number of sticker sets to return
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetArchivedStickerSets {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetArchivedStickerSets {}

impl GetArchivedStickerSets {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetArchivedStickerSetsBuilder {
        let mut inner = GetArchivedStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getArchivedStickerSets".to_string();

        RTDGetArchivedStickerSetsBuilder { inner }
    }

    pub fn is_masks(&self) -> bool {
        self.is_masks
    }

    pub fn offset_sticker_set_id(&self) -> i64 {
        self.offset_sticker_set_id
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDGetArchivedStickerSetsBuilder {
    inner: GetArchivedStickerSets,
}

impl RTDGetArchivedStickerSetsBuilder {
    pub fn build(&self) -> GetArchivedStickerSets {
        self.inner.clone()
    }

    pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
        self.inner.is_masks = is_masks;
        self
    }

    pub fn offset_sticker_set_id(&mut self, offset_sticker_set_id: i64) -> &mut Self {
        self.inner.offset_sticker_set_id = offset_sticker_set_id;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetArchivedStickerSets> for GetArchivedStickerSets {
    fn as_ref(&self) -> &GetArchivedStickerSets {
        self
    }
}

impl AsRef<GetArchivedStickerSets> for RTDGetArchivedStickerSetsBuilder {
    fn as_ref(&self) -> &GetArchivedStickerSets {
        &self.inner
    }
}
