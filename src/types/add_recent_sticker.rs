use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Manually adds a new sticker to the list of recently used stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddRecentSticker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pass true to add the sticker to the list of stickers recently attached to photo or video files; pass false to add the sticker to the list of recently sent stickers

    #[serde(default)]
    is_attached: bool,
    /// Sticker file to add

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddRecentSticker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddRecentSticker {}

impl AddRecentSticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddRecentStickerBuilder {
        let mut inner = AddRecentSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addRecentSticker".to_string();

        AddRecentStickerBuilder { inner }
    }

    pub fn is_attached(&self) -> bool {
        self.is_attached
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }
}

#[doc(hidden)]
pub struct AddRecentStickerBuilder {
    inner: AddRecentSticker,
}

#[deprecated]
pub type RTDAddRecentStickerBuilder = AddRecentStickerBuilder;

impl AddRecentStickerBuilder {
    pub fn build(&self) -> AddRecentSticker {
        self.inner.clone()
    }

    pub fn is_attached(&mut self, is_attached: bool) -> &mut Self {
        self.inner.is_attached = is_attached;
        self
    }

    pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }
}

impl AsRef<AddRecentSticker> for AddRecentSticker {
    fn as_ref(&self) -> &AddRecentSticker {
        self
    }
}

impl AsRef<AddRecentSticker> for AddRecentStickerBuilder {
    fn as_ref(&self) -> &AddRecentSticker {
        &self.inner
    }
}
