use crate::errors::Result;
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
    /// File with the sticker; must fit in a 512x512 square. For WEBP stickers the file must be in WEBP or PNG format, which will be converted to WEBP server-side. See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,
    /// String with 1-20 emoji corresponding to the sticker

    #[serde(default)]
    emojis: String,
    /// Position where the mask is placed; pass null if not specified
    mask_position: MaskPosition,
    /// List of up to 20 keywords with total length up to 64 characters, which can be used to find the sticker

    #[serde(default)]
    keywords: Vec<String>,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputStickerBuilder {
        let mut inner = InputSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputStickerBuilder { inner }
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }

    pub fn emojis(&self) -> &String {
        &self.emojis
    }

    pub fn mask_position(&self) -> &MaskPosition {
        &self.mask_position
    }

    pub fn keywords(&self) -> &Vec<String> {
        &self.keywords
    }
}

#[doc(hidden)]
pub struct InputStickerBuilder {
    inner: InputSticker,
}

#[deprecated]
pub type RTDInputStickerBuilder = InputStickerBuilder;

impl InputStickerBuilder {
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

    pub fn mask_position<T: AsRef<MaskPosition>>(&mut self, mask_position: T) -> &mut Self {
        self.inner.mask_position = mask_position.as_ref().clone();
        self
    }

    pub fn keywords(&mut self, keywords: Vec<String>) -> &mut Self {
        self.inner.keywords = keywords;
        self
    }
}

impl AsRef<InputSticker> for InputSticker {
    fn as_ref(&self) -> &InputSticker {
        self
    }
}

impl AsRef<InputSticker> for InputStickerBuilder {
    fn as_ref(&self) -> &InputSticker {
        &self.inner
    }
}
