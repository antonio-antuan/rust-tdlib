use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes type of a sticker
pub trait TDStickerType: Debug + RObject {}

/// Describes type of a sticker
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StickerType {
    #[doc(hidden)]
    _Default,
    /// The sticker is an animation in TGS format
    #[serde(rename(deserialize = "stickerTypeAnimated"))]
    Animated(StickerTypeAnimated),
    /// The sticker is a mask in WEBP format to be placed on photos or videos
    #[serde(rename(deserialize = "stickerTypeMask"))]
    Mask(StickerTypeMask),
    /// The sticker is an image in WEBP format
    #[serde(rename(deserialize = "stickerTypeStatic"))]
    Static(StickerTypeStatic),
    /// The sticker is a video in WEBM format
    #[serde(rename(deserialize = "stickerTypeVideo"))]
    Video(StickerTypeVideo),
}

impl Default for StickerType {
    fn default() -> Self {
        StickerType::_Default
    }
}

impl RObject for StickerType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            StickerType::Animated(t) => t.extra(),
            StickerType::Mask(t) => t.extra(),
            StickerType::Static(t) => t.extra(),
            StickerType::Video(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            StickerType::Animated(t) => t.client_id(),
            StickerType::Mask(t) => t.client_id(),
            StickerType::Static(t) => t.client_id(),
            StickerType::Video(t) => t.client_id(),

            _ => None,
        }
    }
}

impl StickerType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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

/// The sticker is an animation in TGS format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerTypeAnimated {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for StickerTypeAnimated {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStickerType for StickerTypeAnimated {}

impl StickerTypeAnimated {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStickerTypeAnimatedBuilder {
        let mut inner = StickerTypeAnimated::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDStickerTypeAnimatedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDStickerTypeAnimatedBuilder {
    inner: StickerTypeAnimated,
}

impl RTDStickerTypeAnimatedBuilder {
    pub fn build(&self) -> StickerTypeAnimated {
        self.inner.clone()
    }
}

impl AsRef<StickerTypeAnimated> for StickerTypeAnimated {
    fn as_ref(&self) -> &StickerTypeAnimated {
        self
    }
}

impl AsRef<StickerTypeAnimated> for RTDStickerTypeAnimatedBuilder {
    fn as_ref(&self) -> &StickerTypeAnimated {
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
    /// Position where the mask is placed; may be null
    mask_position: Option<MaskPosition>,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStickerTypeMaskBuilder {
        let mut inner = StickerTypeMask::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDStickerTypeMaskBuilder { inner }
    }

    pub fn mask_position(&self) -> &Option<MaskPosition> {
        &self.mask_position
    }
}

#[doc(hidden)]
pub struct RTDStickerTypeMaskBuilder {
    inner: StickerTypeMask,
}

impl RTDStickerTypeMaskBuilder {
    pub fn build(&self) -> StickerTypeMask {
        self.inner.clone()
    }

    pub fn mask_position<T: AsRef<MaskPosition>>(&mut self, mask_position: T) -> &mut Self {
        self.inner.mask_position = Some(mask_position.as_ref().clone());
        self
    }
}

impl AsRef<StickerTypeMask> for StickerTypeMask {
    fn as_ref(&self) -> &StickerTypeMask {
        self
    }
}

impl AsRef<StickerTypeMask> for RTDStickerTypeMaskBuilder {
    fn as_ref(&self) -> &StickerTypeMask {
        &self.inner
    }
}

/// The sticker is an image in WEBP format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerTypeStatic {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for StickerTypeStatic {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStickerType for StickerTypeStatic {}

impl StickerTypeStatic {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStickerTypeStaticBuilder {
        let mut inner = StickerTypeStatic::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDStickerTypeStaticBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDStickerTypeStaticBuilder {
    inner: StickerTypeStatic,
}

impl RTDStickerTypeStaticBuilder {
    pub fn build(&self) -> StickerTypeStatic {
        self.inner.clone()
    }
}

impl AsRef<StickerTypeStatic> for StickerTypeStatic {
    fn as_ref(&self) -> &StickerTypeStatic {
        self
    }
}

impl AsRef<StickerTypeStatic> for RTDStickerTypeStaticBuilder {
    fn as_ref(&self) -> &StickerTypeStatic {
        &self.inner
    }
}

/// The sticker is a video in WEBM format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerTypeVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for StickerTypeVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStickerType for StickerTypeVideo {}

impl StickerTypeVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStickerTypeVideoBuilder {
        let mut inner = StickerTypeVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDStickerTypeVideoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDStickerTypeVideoBuilder {
    inner: StickerTypeVideo,
}

impl RTDStickerTypeVideoBuilder {
    pub fn build(&self) -> StickerTypeVideo {
        self.inner.clone()
    }
}

impl AsRef<StickerTypeVideo> for StickerTypeVideo {
    fn as_ref(&self) -> &StickerTypeVideo {
        self
    }
}

impl AsRef<StickerTypeVideo> for RTDStickerTypeVideoBuilder {
    fn as_ref(&self) -> &StickerTypeVideo {
        &self.inner
    }
}
