use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes format of a sticker
pub trait TDStickerFormat: Debug + RObject {}

/// Describes format of a sticker
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum StickerFormat {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The sticker is an animation in TGS format
    #[serde(rename = "stickerFormatTgs")]
    Tgs(StickerFormatTgs),
    /// The sticker is a video in WEBM format
    #[serde(rename = "stickerFormatWebm")]
    Webm(StickerFormatWebm),
    /// The sticker is an image in WEBP format
    #[serde(rename = "stickerFormatWebp")]
    Webp(StickerFormatWebp),
}

impl RObject for StickerFormat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            StickerFormat::Tgs(t) => t.extra(),
            StickerFormat::Webm(t) => t.extra(),
            StickerFormat::Webp(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            StickerFormat::Tgs(t) => t.client_id(),
            StickerFormat::Webm(t) => t.client_id(),
            StickerFormat::Webp(t) => t.client_id(),

            _ => None,
        }
    }
}

impl StickerFormat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, StickerFormat::_Default)
    }
}

impl AsRef<StickerFormat> for StickerFormat {
    fn as_ref(&self) -> &StickerFormat {
        self
    }
}

/// The sticker is an animation in TGS format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerFormatTgs {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for StickerFormatTgs {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStickerFormat for StickerFormatTgs {}

impl StickerFormatTgs {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StickerFormatTgsBuilder {
        let mut inner = StickerFormatTgs::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StickerFormatTgsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct StickerFormatTgsBuilder {
    inner: StickerFormatTgs,
}

#[deprecated]
pub type RTDStickerFormatTgsBuilder = StickerFormatTgsBuilder;

impl StickerFormatTgsBuilder {
    pub fn build(&self) -> StickerFormatTgs {
        self.inner.clone()
    }
}

impl AsRef<StickerFormatTgs> for StickerFormatTgs {
    fn as_ref(&self) -> &StickerFormatTgs {
        self
    }
}

impl AsRef<StickerFormatTgs> for StickerFormatTgsBuilder {
    fn as_ref(&self) -> &StickerFormatTgs {
        &self.inner
    }
}

/// The sticker is a video in WEBM format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerFormatWebm {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for StickerFormatWebm {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStickerFormat for StickerFormatWebm {}

impl StickerFormatWebm {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StickerFormatWebmBuilder {
        let mut inner = StickerFormatWebm::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StickerFormatWebmBuilder { inner }
    }
}

#[doc(hidden)]
pub struct StickerFormatWebmBuilder {
    inner: StickerFormatWebm,
}

#[deprecated]
pub type RTDStickerFormatWebmBuilder = StickerFormatWebmBuilder;

impl StickerFormatWebmBuilder {
    pub fn build(&self) -> StickerFormatWebm {
        self.inner.clone()
    }
}

impl AsRef<StickerFormatWebm> for StickerFormatWebm {
    fn as_ref(&self) -> &StickerFormatWebm {
        self
    }
}

impl AsRef<StickerFormatWebm> for StickerFormatWebmBuilder {
    fn as_ref(&self) -> &StickerFormatWebm {
        &self.inner
    }
}

/// The sticker is an image in WEBP format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerFormatWebp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for StickerFormatWebp {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStickerFormat for StickerFormatWebp {}

impl StickerFormatWebp {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StickerFormatWebpBuilder {
        let mut inner = StickerFormatWebp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StickerFormatWebpBuilder { inner }
    }
}

#[doc(hidden)]
pub struct StickerFormatWebpBuilder {
    inner: StickerFormatWebp,
}

#[deprecated]
pub type RTDStickerFormatWebpBuilder = StickerFormatWebpBuilder;

impl StickerFormatWebpBuilder {
    pub fn build(&self) -> StickerFormatWebp {
        self.inner.clone()
    }
}

impl AsRef<StickerFormatWebp> for StickerFormatWebp {
    fn as_ref(&self) -> &StickerFormatWebp {
        self
    }
}

impl AsRef<StickerFormatWebp> for StickerFormatWebpBuilder {
    fn as_ref(&self) -> &StickerFormatWebp {
        &self.inner
    }
}
