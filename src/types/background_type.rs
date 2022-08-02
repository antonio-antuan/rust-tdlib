use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the type of a background
pub trait TDBackgroundType: Debug + RObject {}

/// Describes the type of a background
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BackgroundType {
    #[doc(hidden)]
    _Default,
    /// A filled background
    #[serde(rename = "backgroundTypeFill")]
    Fill(BackgroundTypeFill),
    /// A PNG or TGV (gzipped subset of SVG with MIME type "application/x-tgwallpattern") pattern to be combined with the background fill chosen by the user
    #[serde(rename = "backgroundTypePattern")]
    Pattern(BackgroundTypePattern),
    /// A wallpaper in JPEG format
    #[serde(rename = "backgroundTypeWallpaper")]
    Wallpaper(BackgroundTypeWallpaper),
}

impl Default for BackgroundType {
    fn default() -> Self {
        BackgroundType::_Default
    }
}

impl RObject for BackgroundType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            BackgroundType::Fill(t) => t.extra(),
            BackgroundType::Pattern(t) => t.extra(),
            BackgroundType::Wallpaper(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            BackgroundType::Fill(t) => t.client_id(),
            BackgroundType::Pattern(t) => t.client_id(),
            BackgroundType::Wallpaper(t) => t.client_id(),

            _ => None,
        }
    }
}

impl BackgroundType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, BackgroundType::_Default)
    }
}

impl AsRef<BackgroundType> for BackgroundType {
    fn as_ref(&self) -> &BackgroundType {
        self
    }
}

/// A filled background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundTypeFill {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The background fill

    #[serde(skip_serializing_if = "BackgroundFill::_is_default")]
    fill: BackgroundFill,
}

impl RObject for BackgroundTypeFill {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBackgroundType for BackgroundTypeFill {}

impl BackgroundTypeFill {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BackgroundTypeFillBuilder {
        let mut inner = BackgroundTypeFill::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BackgroundTypeFillBuilder { inner }
    }

    pub fn fill(&self) -> &BackgroundFill {
        &self.fill
    }
}

#[doc(hidden)]
pub struct BackgroundTypeFillBuilder {
    inner: BackgroundTypeFill,
}

#[deprecated]
pub type RTDBackgroundTypeFillBuilder = BackgroundTypeFillBuilder;

impl BackgroundTypeFillBuilder {
    pub fn build(&self) -> BackgroundTypeFill {
        self.inner.clone()
    }

    pub fn fill<T: AsRef<BackgroundFill>>(&mut self, fill: T) -> &mut Self {
        self.inner.fill = fill.as_ref().clone();
        self
    }
}

impl AsRef<BackgroundTypeFill> for BackgroundTypeFill {
    fn as_ref(&self) -> &BackgroundTypeFill {
        self
    }
}

impl AsRef<BackgroundTypeFill> for BackgroundTypeFillBuilder {
    fn as_ref(&self) -> &BackgroundTypeFill {
        &self.inner
    }
}

/// A PNG or TGV (gzipped subset of SVG with MIME type "application/x-tgwallpattern") pattern to be combined with the background fill chosen by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundTypePattern {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Fill of the background

    #[serde(skip_serializing_if = "BackgroundFill::_is_default")]
    fill: BackgroundFill,
    /// Intensity of the pattern when it is shown above the filled background; 0-100.

    #[serde(default)]
    intensity: i32,
    /// True, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only

    #[serde(default)]
    is_inverted: bool,
    /// True, if the background needs to be slightly moved when device is tilted

    #[serde(default)]
    is_moving: bool,
}

impl RObject for BackgroundTypePattern {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBackgroundType for BackgroundTypePattern {}

impl BackgroundTypePattern {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BackgroundTypePatternBuilder {
        let mut inner = BackgroundTypePattern::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BackgroundTypePatternBuilder { inner }
    }

    pub fn fill(&self) -> &BackgroundFill {
        &self.fill
    }

    pub fn intensity(&self) -> i32 {
        self.intensity
    }

    pub fn is_inverted(&self) -> bool {
        self.is_inverted
    }

    pub fn is_moving(&self) -> bool {
        self.is_moving
    }
}

#[doc(hidden)]
pub struct BackgroundTypePatternBuilder {
    inner: BackgroundTypePattern,
}

#[deprecated]
pub type RTDBackgroundTypePatternBuilder = BackgroundTypePatternBuilder;

impl BackgroundTypePatternBuilder {
    pub fn build(&self) -> BackgroundTypePattern {
        self.inner.clone()
    }

    pub fn fill<T: AsRef<BackgroundFill>>(&mut self, fill: T) -> &mut Self {
        self.inner.fill = fill.as_ref().clone();
        self
    }

    pub fn intensity(&mut self, intensity: i32) -> &mut Self {
        self.inner.intensity = intensity;
        self
    }

    pub fn is_inverted(&mut self, is_inverted: bool) -> &mut Self {
        self.inner.is_inverted = is_inverted;
        self
    }

    pub fn is_moving(&mut self, is_moving: bool) -> &mut Self {
        self.inner.is_moving = is_moving;
        self
    }
}

impl AsRef<BackgroundTypePattern> for BackgroundTypePattern {
    fn as_ref(&self) -> &BackgroundTypePattern {
        self
    }
}

impl AsRef<BackgroundTypePattern> for BackgroundTypePatternBuilder {
    fn as_ref(&self) -> &BackgroundTypePattern {
        &self.inner
    }
}

/// A wallpaper in JPEG format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundTypeWallpaper {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the wallpaper must be downscaled to fit in 450x450 square and then box-blurred with radius 12

    #[serde(default)]
    is_blurred: bool,
    /// True, if the background needs to be slightly moved when device is tilted

    #[serde(default)]
    is_moving: bool,
}

impl RObject for BackgroundTypeWallpaper {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBackgroundType for BackgroundTypeWallpaper {}

impl BackgroundTypeWallpaper {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BackgroundTypeWallpaperBuilder {
        let mut inner = BackgroundTypeWallpaper::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BackgroundTypeWallpaperBuilder { inner }
    }

    pub fn is_blurred(&self) -> bool {
        self.is_blurred
    }

    pub fn is_moving(&self) -> bool {
        self.is_moving
    }
}

#[doc(hidden)]
pub struct BackgroundTypeWallpaperBuilder {
    inner: BackgroundTypeWallpaper,
}

#[deprecated]
pub type RTDBackgroundTypeWallpaperBuilder = BackgroundTypeWallpaperBuilder;

impl BackgroundTypeWallpaperBuilder {
    pub fn build(&self) -> BackgroundTypeWallpaper {
        self.inner.clone()
    }

    pub fn is_blurred(&mut self, is_blurred: bool) -> &mut Self {
        self.inner.is_blurred = is_blurred;
        self
    }

    pub fn is_moving(&mut self, is_moving: bool) -> &mut Self {
        self.inner.is_moving = is_moving;
        self
    }
}

impl AsRef<BackgroundTypeWallpaper> for BackgroundTypeWallpaper {
    fn as_ref(&self) -> &BackgroundTypeWallpaper {
        self
    }
}

impl AsRef<BackgroundTypeWallpaper> for BackgroundTypeWallpaperBuilder {
    fn as_ref(&self) -> &BackgroundTypeWallpaper {
        &self.inner
    }
}
