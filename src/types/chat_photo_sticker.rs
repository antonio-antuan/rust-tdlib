use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Information about the sticker, which was used to create the chat photo. The sticker is shown at the center of the photo and occupies at most 67% of it
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPhotoSticker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the sticker

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "ChatPhotoStickerType::_is_default")]
    type_: ChatPhotoStickerType,
    /// The fill to be used as background for the sticker; rotation angle in backgroundFillGradient isn't supported

    #[serde(skip_serializing_if = "BackgroundFill::_is_default")]
    background_fill: BackgroundFill,
}

impl RObject for ChatPhotoSticker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatPhotoSticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatPhotoStickerBuilder {
        let mut inner = ChatPhotoSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatPhotoStickerBuilder { inner }
    }

    pub fn type_(&self) -> &ChatPhotoStickerType {
        &self.type_
    }

    pub fn background_fill(&self) -> &BackgroundFill {
        &self.background_fill
    }
}

#[doc(hidden)]
pub struct ChatPhotoStickerBuilder {
    inner: ChatPhotoSticker,
}

#[deprecated]
pub type RTDChatPhotoStickerBuilder = ChatPhotoStickerBuilder;

impl ChatPhotoStickerBuilder {
    pub fn build(&self) -> ChatPhotoSticker {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<ChatPhotoStickerType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn background_fill<T: AsRef<BackgroundFill>>(&mut self, background_fill: T) -> &mut Self {
        self.inner.background_fill = background_fill.as_ref().clone();
        self
    }
}

impl AsRef<ChatPhotoSticker> for ChatPhotoSticker {
    fn as_ref(&self) -> &ChatPhotoSticker {
        self
    }
}

impl AsRef<ChatPhotoSticker> for ChatPhotoStickerBuilder {
    fn as_ref(&self) -> &ChatPhotoSticker {
        &self.inner
    }
}
