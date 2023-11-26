use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes type of a sticker
pub trait TDStickerType: Debug + RObject {}

/// Describes type of a sticker
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum StickerType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The sticker is a custom emoji to be used inside message text and caption
    #[serde(rename = "stickerTypeCustomEmoji")]
    CustomEmoji(StickerTypeCustomEmoji),
    /// The sticker is a mask in WEBP format to be placed on photos or videos
    #[serde(rename = "stickerTypeMask")]
    Mask(StickerTypeMask),
    /// The sticker is a regular sticker
    #[serde(rename = "stickerTypeRegular")]
    Regular(StickerTypeRegular),
}

impl RObject for StickerType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            StickerType::CustomEmoji(t) => t.extra(),
            StickerType::Mask(t) => t.extra(),
            StickerType::Regular(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            StickerType::CustomEmoji(t) => t.client_id(),
            StickerType::Mask(t) => t.client_id(),
            StickerType::Regular(t) => t.client_id(),

            _ => None,
        }
    }
}

impl StickerType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, StickerType::_Default)
    }
}

impl AsRef<StickerType> for StickerType {
    fn as_ref(&self) -> &StickerType {
        self
    }
}

/// The sticker is a custom emoji to be used inside message text and caption
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerTypeCustomEmoji {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for StickerTypeCustomEmoji {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStickerType for StickerTypeCustomEmoji {}

impl StickerTypeCustomEmoji {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StickerTypeCustomEmojiBuilder {
        let mut inner = StickerTypeCustomEmoji::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StickerTypeCustomEmojiBuilder { inner }
    }
}

#[doc(hidden)]
pub struct StickerTypeCustomEmojiBuilder {
    inner: StickerTypeCustomEmoji,
}

#[deprecated]
pub type RTDStickerTypeCustomEmojiBuilder = StickerTypeCustomEmojiBuilder;

impl StickerTypeCustomEmojiBuilder {
    pub fn build(&self) -> StickerTypeCustomEmoji {
        self.inner.clone()
    }
}

impl AsRef<StickerTypeCustomEmoji> for StickerTypeCustomEmoji {
    fn as_ref(&self) -> &StickerTypeCustomEmoji {
        self
    }
}

impl AsRef<StickerTypeCustomEmoji> for StickerTypeCustomEmojiBuilder {
    fn as_ref(&self) -> &StickerTypeCustomEmoji {
        &self.inner
    }
}

/// The sticker is a mask in WEBP format to be placed on photos or videos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerTypeMask {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for StickerTypeMask {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStickerType for StickerTypeMask {}

impl StickerTypeMask {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StickerTypeMaskBuilder {
        let mut inner = StickerTypeMask::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StickerTypeMaskBuilder { inner }
    }
}

#[doc(hidden)]
pub struct StickerTypeMaskBuilder {
    inner: StickerTypeMask,
}

#[deprecated]
pub type RTDStickerTypeMaskBuilder = StickerTypeMaskBuilder;

impl StickerTypeMaskBuilder {
    pub fn build(&self) -> StickerTypeMask {
        self.inner.clone()
    }
}

impl AsRef<StickerTypeMask> for StickerTypeMask {
    fn as_ref(&self) -> &StickerTypeMask {
        self
    }
}

impl AsRef<StickerTypeMask> for StickerTypeMaskBuilder {
    fn as_ref(&self) -> &StickerTypeMask {
        &self.inner
    }
}

/// The sticker is a regular sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerTypeRegular {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for StickerTypeRegular {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStickerType for StickerTypeRegular {}

impl StickerTypeRegular {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StickerTypeRegularBuilder {
        let mut inner = StickerTypeRegular::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StickerTypeRegularBuilder { inner }
    }
}

#[doc(hidden)]
pub struct StickerTypeRegularBuilder {
    inner: StickerTypeRegular,
}

#[deprecated]
pub type RTDStickerTypeRegularBuilder = StickerTypeRegularBuilder;

impl StickerTypeRegularBuilder {
    pub fn build(&self) -> StickerTypeRegular {
        self.inner.clone()
    }
}

impl AsRef<StickerTypeRegular> for StickerTypeRegular {
    fn as_ref(&self) -> &StickerTypeRegular {
        self
    }
}

impl AsRef<StickerTypeRegular> for StickerTypeRegularBuilder {
    fn as_ref(&self) -> &StickerTypeRegular {
        &self.inner
    }
}
