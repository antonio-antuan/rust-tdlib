use crate::errors::*;
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
    #[serde(rename(serialize = "backgroundTypeFill", deserialize = "backgroundTypeFill"))]
    Fill(BackgroundTypeFill),
    /// A PNG or TGV (gzipped subset of SVG with MIME type "application/x-tgwallpattern") pattern to be combined with the background fill chosen by the user
    #[serde(rename(
        serialize = "backgroundTypePattern",
        deserialize = "backgroundTypePattern"
    ))]
    Pattern(BackgroundTypePattern),
    /// A wallpaper in JPEG format
    #[serde(rename(
        serialize = "backgroundTypeWallpaper",
        deserialize = "backgroundTypeWallpaper"
    ))]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    /// Description of the background fill

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBackgroundTypeFillBuilder {
        let mut inner = BackgroundTypeFill::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBackgroundTypeFillBuilder { inner }
    }

    pub fn fill(&self) -> &BackgroundFill {
        &self.fill
    }
}

#[doc(hidden)]
pub struct RTDBackgroundTypeFillBuilder {
    inner: BackgroundTypeFill,
}

impl RTDBackgroundTypeFillBuilder {
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

impl AsRef<BackgroundTypeFill> for RTDBackgroundTypeFillBuilder {
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
    /// Description of the background fill

    #[serde(skip_serializing_if = "BackgroundFill::_is_default")]
    fill: BackgroundFill,
    /// Intensity of the pattern when it is shown above the filled background, 0-100
    intensity: i32,
    /// True, if the background needs to be slightly moved when device is tilted
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBackgroundTypePatternBuilder {
        let mut inner = BackgroundTypePattern::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBackgroundTypePatternBuilder { inner }
    }

    pub fn fill(&self) -> &BackgroundFill {
        &self.fill
    }

    pub fn intensity(&self) -> i32 {
        self.intensity
    }

    pub fn is_moving(&self) -> bool {
        self.is_moving
    }
}

#[doc(hidden)]
pub struct RTDBackgroundTypePatternBuilder {
    inner: BackgroundTypePattern,
}

impl RTDBackgroundTypePatternBuilder {
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

impl AsRef<BackgroundTypePattern> for RTDBackgroundTypePatternBuilder {
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
    is_blurred: bool,
    /// True, if the background needs to be slightly moved when device is tilted
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBackgroundTypeWallpaperBuilder {
        let mut inner = BackgroundTypeWallpaper::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBackgroundTypeWallpaperBuilder { inner }
    }

    pub fn is_blurred(&self) -> bool {
        self.is_blurred
    }

    pub fn is_moving(&self) -> bool {
        self.is_moving
    }
}

#[doc(hidden)]
pub struct RTDBackgroundTypeWallpaperBuilder {
    inner: BackgroundTypeWallpaper,
}

impl RTDBackgroundTypeWallpaperBuilder {
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

impl AsRef<BackgroundTypeWallpaper> for RTDBackgroundTypeWallpaperBuilder {
    fn as_ref(&self) -> &BackgroundTypeWallpaper {
        &self.inner
    }
}
