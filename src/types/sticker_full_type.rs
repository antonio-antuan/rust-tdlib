use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains full information about sticker type
pub trait TDStickerFullType: Debug + RObject {}

/// Contains full information about sticker type
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum StickerFullType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The sticker is a custom emoji to be used inside message text and caption. Currently, only Telegram Premium users can use custom emoji
    #[serde(rename = "stickerFullTypeCustomEmoji")]
    CustomEmoji(StickerFullTypeCustomEmoji),
    /// The sticker is a mask in WEBP format to be placed on photos or videos
    #[serde(rename = "stickerFullTypeMask")]
    Mask(StickerFullTypeMask),
    /// The sticker is a regular sticker
    #[serde(rename = "stickerFullTypeRegular")]
    Regular(StickerFullTypeRegular),
}

impl RObject for StickerFullType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            StickerFullType::CustomEmoji(t) => t.extra(),
            StickerFullType::Mask(t) => t.extra(),
            StickerFullType::Regular(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            StickerFullType::CustomEmoji(t) => t.client_id(),
            StickerFullType::Mask(t) => t.client_id(),
            StickerFullType::Regular(t) => t.client_id(),

            _ => None,
        }
    }
}

impl StickerFullType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, StickerFullType::_Default)
    }
}

impl AsRef<StickerFullType> for StickerFullType {
    fn as_ref(&self) -> &StickerFullType {
        self
    }
}

/// The sticker is a custom emoji to be used inside message text and caption. Currently, only Telegram Premium users can use custom emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerFullTypeCustomEmoji {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the custom emoji

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    custom_emoji_id: i64,
    /// True, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places

    #[serde(default)]
    needs_repainting: bool,
}

impl RObject for StickerFullTypeCustomEmoji {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStickerFullType for StickerFullTypeCustomEmoji {}

impl StickerFullTypeCustomEmoji {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StickerFullTypeCustomEmojiBuilder {
        let mut inner = StickerFullTypeCustomEmoji::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StickerFullTypeCustomEmojiBuilder { inner }
    }

    pub fn custom_emoji_id(&self) -> i64 {
        self.custom_emoji_id
    }

    pub fn needs_repainting(&self) -> bool {
        self.needs_repainting
    }
}

#[doc(hidden)]
pub struct StickerFullTypeCustomEmojiBuilder {
    inner: StickerFullTypeCustomEmoji,
}

#[deprecated]
pub type RTDStickerFullTypeCustomEmojiBuilder = StickerFullTypeCustomEmojiBuilder;

impl StickerFullTypeCustomEmojiBuilder {
    pub fn build(&self) -> StickerFullTypeCustomEmoji {
        self.inner.clone()
    }

    pub fn custom_emoji_id(&mut self, custom_emoji_id: i64) -> &mut Self {
        self.inner.custom_emoji_id = custom_emoji_id;
        self
    }

    pub fn needs_repainting(&mut self, needs_repainting: bool) -> &mut Self {
        self.inner.needs_repainting = needs_repainting;
        self
    }
}

impl AsRef<StickerFullTypeCustomEmoji> for StickerFullTypeCustomEmoji {
    fn as_ref(&self) -> &StickerFullTypeCustomEmoji {
        self
    }
}

impl AsRef<StickerFullTypeCustomEmoji> for StickerFullTypeCustomEmojiBuilder {
    fn as_ref(&self) -> &StickerFullTypeCustomEmoji {
        &self.inner
    }
}

/// The sticker is a mask in WEBP format to be placed on photos or videos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerFullTypeMask {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Position where the mask is placed; may be null
    mask_position: Option<MaskPosition>,
}

impl RObject for StickerFullTypeMask {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStickerFullType for StickerFullTypeMask {}

impl StickerFullTypeMask {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StickerFullTypeMaskBuilder {
        let mut inner = StickerFullTypeMask::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StickerFullTypeMaskBuilder { inner }
    }

    pub fn mask_position(&self) -> &Option<MaskPosition> {
        &self.mask_position
    }
}

#[doc(hidden)]
pub struct StickerFullTypeMaskBuilder {
    inner: StickerFullTypeMask,
}

#[deprecated]
pub type RTDStickerFullTypeMaskBuilder = StickerFullTypeMaskBuilder;

impl StickerFullTypeMaskBuilder {
    pub fn build(&self) -> StickerFullTypeMask {
        self.inner.clone()
    }

    pub fn mask_position<T: AsRef<MaskPosition>>(&mut self, mask_position: T) -> &mut Self {
        self.inner.mask_position = Some(mask_position.as_ref().clone());
        self
    }
}

impl AsRef<StickerFullTypeMask> for StickerFullTypeMask {
    fn as_ref(&self) -> &StickerFullTypeMask {
        self
    }
}

impl AsRef<StickerFullTypeMask> for StickerFullTypeMaskBuilder {
    fn as_ref(&self) -> &StickerFullTypeMask {
        &self.inner
    }
}

/// The sticker is a regular sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerFullTypeRegular {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Premium animation of the sticker; may be null. If present, only Telegram Premium users can use the sticker
    premium_animation: Option<File>,
}

impl RObject for StickerFullTypeRegular {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStickerFullType for StickerFullTypeRegular {}

impl StickerFullTypeRegular {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StickerFullTypeRegularBuilder {
        let mut inner = StickerFullTypeRegular::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StickerFullTypeRegularBuilder { inner }
    }

    pub fn premium_animation(&self) -> &Option<File> {
        &self.premium_animation
    }
}

#[doc(hidden)]
pub struct StickerFullTypeRegularBuilder {
    inner: StickerFullTypeRegular,
}

#[deprecated]
pub type RTDStickerFullTypeRegularBuilder = StickerFullTypeRegularBuilder;

impl StickerFullTypeRegularBuilder {
    pub fn build(&self) -> StickerFullTypeRegular {
        self.inner.clone()
    }

    pub fn premium_animation<T: AsRef<File>>(&mut self, premium_animation: T) -> &mut Self {
        self.inner.premium_animation = Some(premium_animation.as_ref().clone());
        self
    }
}

impl AsRef<StickerFullTypeRegular> for StickerFullTypeRegular {
    fn as_ref(&self) -> &StickerFullTypeRegular {
        self
    }
}

impl AsRef<StickerFullTypeRegular> for StickerFullTypeRegularBuilder {
    fn as_ref(&self) -> &StickerFullTypeRegular {
        &self.inner
    }
}
