use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Removes a sticker from the list of recently used stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveRecentSticker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pass true to remove the sticker from the list of stickers recently attached to photo or video files; pass false to remove the sticker from the list of recently sent stickers

    #[serde(default)]
    is_attached: bool,
    /// Sticker file to delete

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveRecentSticker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveRecentSticker {}

impl RemoveRecentSticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RemoveRecentStickerBuilder {
        let mut inner = RemoveRecentSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeRecentSticker".to_string();

        RemoveRecentStickerBuilder { inner }
    }

    pub fn is_attached(&self) -> bool {
        self.is_attached
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }
}

#[doc(hidden)]
pub struct RemoveRecentStickerBuilder {
    inner: RemoveRecentSticker,
}

#[deprecated]
pub type RTDRemoveRecentStickerBuilder = RemoveRecentStickerBuilder;

impl RemoveRecentStickerBuilder {
    pub fn build(&self) -> RemoveRecentSticker {
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

impl AsRef<RemoveRecentSticker> for RemoveRecentSticker {
    fn as_ref(&self) -> &RemoveRecentSticker {
        self
    }
}

impl AsRef<RemoveRecentSticker> for RemoveRecentStickerBuilder {
    fn as_ref(&self) -> &RemoveRecentSticker {
        &self.inner
    }
}
