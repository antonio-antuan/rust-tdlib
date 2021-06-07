use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a sticker that needs to be added to a sticker set
pub trait TDInputSticker: Debug + RObject {}

/// Describes a sticker that needs to be added to a sticker set
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputSticker {
    #[doc(hidden)]
    _Default,
    /// An animated sticker in TGS format
    #[serde(rename(
        serialize = "inputStickerAnimated",
        deserialize = "inputStickerAnimated"
    ))]
    Animated(InputStickerAnimated),
    /// A static sticker in PNG format, which will be converted to WEBP server-side
    #[serde(rename(serialize = "inputStickerStatic", deserialize = "inputStickerStatic"))]
    Static(InputStickerStatic),
}

impl Default for InputSticker {
    fn default() -> Self {
        InputSticker::_Default
    }
}

impl RObject for InputSticker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InputSticker::Animated(t) => t.extra(),
            InputSticker::Static(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InputSticker::Animated(t) => t.client_id(),
            InputSticker::Static(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InputSticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputSticker::_Default)
    }
}

impl AsRef<InputSticker> for InputSticker {
    fn as_ref(&self) -> &InputSticker {
        self
    }
}

/// An animated sticker in TGS format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStickerAnimated {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// File with the animated sticker. Only local or uploaded within a week files are supported. See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,
    /// Emojis corresponding to the sticker
    emojis: String,
}

impl RObject for InputStickerAnimated {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputSticker for InputStickerAnimated {}

impl InputStickerAnimated {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputStickerAnimatedBuilder {
        let mut inner = InputStickerAnimated::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputStickerAnimatedBuilder { inner }
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }

    pub fn emojis(&self) -> &String {
        &self.emojis
    }
}

#[doc(hidden)]
pub struct RTDInputStickerAnimatedBuilder {
    inner: InputStickerAnimated,
}

impl RTDInputStickerAnimatedBuilder {
    pub fn build(&self) -> InputStickerAnimated {
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
}

impl AsRef<InputStickerAnimated> for InputStickerAnimated {
    fn as_ref(&self) -> &InputStickerAnimated {
        self
    }
}

impl AsRef<InputStickerAnimated> for RTDInputStickerAnimatedBuilder {
    fn as_ref(&self) -> &InputStickerAnimated {
        &self.inner
    }
}

/// A static sticker in PNG format, which will be converted to WEBP server-side
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStickerStatic {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// PNG image with the sticker; must be up to 512 KB in size and fit in a 512x512 square

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,
    /// Emojis corresponding to the sticker
    emojis: String,
    /// For masks, position where the mask should be placed; may be null
    mask_position: Option<MaskPosition>,
}

impl RObject for InputStickerStatic {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputSticker for InputStickerStatic {}

impl InputStickerStatic {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputStickerStaticBuilder {
        let mut inner = InputStickerStatic::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputStickerStaticBuilder { inner }
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }

    pub fn emojis(&self) -> &String {
        &self.emojis
    }

    pub fn mask_position(&self) -> &Option<MaskPosition> {
        &self.mask_position
    }
}

#[doc(hidden)]
pub struct RTDInputStickerStaticBuilder {
    inner: InputStickerStatic,
}

impl RTDInputStickerStaticBuilder {
    pub fn build(&self) -> InputStickerStatic {
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
        self.inner.mask_position = Some(mask_position.as_ref().clone());
        self
    }
}

impl AsRef<InputStickerStatic> for InputStickerStatic {
    fn as_ref(&self) -> &InputStickerStatic {
        self
    }
}

impl AsRef<InputStickerStatic> for RTDInputStickerStaticBuilder {
    fn as_ref(&self) -> &InputStickerStatic {
        &self.inner
    }
}
