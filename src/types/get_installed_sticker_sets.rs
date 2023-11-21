use crate::errors::Result;
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
    /// Type of the sticker sets to return

    #[serde(skip_serializing_if = "StickerType::_is_default")]
    sticker_type: StickerType,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetInstalledStickerSetsBuilder {
        let mut inner = GetInstalledStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getInstalledStickerSets".to_string();

        GetInstalledStickerSetsBuilder { inner }
    }

    pub fn sticker_type(&self) -> &StickerType {
        &self.sticker_type
    }
}

#[doc(hidden)]
pub struct GetInstalledStickerSetsBuilder {
    inner: GetInstalledStickerSets,
}

#[deprecated]
pub type RTDGetInstalledStickerSetsBuilder = GetInstalledStickerSetsBuilder;

impl GetInstalledStickerSetsBuilder {
    pub fn build(&self) -> GetInstalledStickerSets {
        self.inner.clone()
    }

    pub fn sticker_type<T: AsRef<StickerType>>(&mut self, sticker_type: T) -> &mut Self {
        self.inner.sticker_type = sticker_type.as_ref().clone();
        self
    }
}

impl AsRef<GetInstalledStickerSets> for GetInstalledStickerSets {
    fn as_ref(&self) -> &GetInstalledStickerSets {
        self
    }
}

impl AsRef<GetInstalledStickerSets> for GetInstalledStickerSetsBuilder {
    fn as_ref(&self) -> &GetInstalledStickerSets {
        &self.inner
    }
}
