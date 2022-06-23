use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A sticker to be added to a sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputSticker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// File with the sticker; must fit in a 512x512 square. For WEBP stickers and masks the file must be in PNG format, which will be converted to WEBP server-side. Otherwise, the file must be local or uploaded within a week. See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,
    /// Emojis corresponding to the sticker
    emojis: String,
    /// Sticker type

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "StickerType::_is_default")]
    type_: StickerType,
}

impl RObject for InputSticker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl InputSticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputStickerBuilder {
        let mut inner = InputSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputStickerBuilder { inner }
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }

    pub fn emojis(&self) -> &String {
        &self.emojis
    }

    pub fn type_(&self) -> &StickerType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct RTDInputStickerBuilder {
    inner: InputSticker,
}

impl RTDInputStickerBuilder {
    pub fn build(&self) -> InputSticker {
        self.inner.clone()
    }

    pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }

    pub fn emojis<T: AsRef<str>>(&mut self, emojis: T) -> &mut Self {
        self.inner.emojis = emojis.as_ref().to_string();
        self
    }

    pub fn type_<T: AsRef<StickerType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<InputSticker> for InputSticker {
    fn as_ref(&self) -> &InputSticker {
        self
    }
}

impl AsRef<InputSticker> for RTDInputStickerBuilder {
    fn as_ref(&self) -> &InputSticker {
        &self.inner
    }
}
