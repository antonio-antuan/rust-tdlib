use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns a list of installed sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstalledStickerSets {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pass true to return mask sticker sets; pass false to return ordinary sticker sets
    is_masks: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetInstalledStickerSets {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetInstalledStickerSets {}

impl GetInstalledStickerSets {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetInstalledStickerSetsBuilder {
        let mut inner = GetInstalledStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getInstalledStickerSets".to_string();

        RTDGetInstalledStickerSetsBuilder { inner }
    }

    pub fn is_masks(&self) -> bool {
        self.is_masks
    }
}

#[doc(hidden)]
pub struct RTDGetInstalledStickerSetsBuilder {
    inner: GetInstalledStickerSets,
}

impl RTDGetInstalledStickerSetsBuilder {
    pub fn build(&self) -> GetInstalledStickerSets {
        self.inner.clone()
    }

    pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
        self.inner.is_masks = is_masks;
        self
    }
}

impl AsRef<GetInstalledStickerSets> for GetInstalledStickerSets {
    fn as_ref(&self) -> &GetInstalledStickerSets {
        self
    }
}

impl AsRef<GetInstalledStickerSets> for RTDGetInstalledStickerSetsBuilder {
    fn as_ref(&self) -> &GetInstalledStickerSets {
        &self.inner
    }
}
