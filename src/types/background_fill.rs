use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a fill of a background
pub trait TDBackgroundFill: Debug + RObject {}

/// Describes a fill of a background
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BackgroundFill {
    #[doc(hidden)]
    _Default(()),
    /// Describes a gradient fill of a background
    Gradient(BackgroundFillGradient),
    /// Describes a solid fill of a background
    Solid(BackgroundFillSolid),
}

impl Default for BackgroundFill {
    fn default() -> Self {
        BackgroundFill::_Default(())
    }
}

impl<'de> Deserialize<'de> for BackgroundFill {
    fn deserialize<D>(deserializer: D) -> Result<BackgroundFill, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          BackgroundFill,
          (backgroundFillGradient, Gradient);
          (backgroundFillSolid, Solid);

        )(deserializer)
    }
}

impl RObject for BackgroundFill {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            BackgroundFill::Gradient(t) => t.td_name(),
            BackgroundFill::Solid(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            BackgroundFill::Gradient(t) => t.extra(),
            BackgroundFill::Solid(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl BackgroundFill {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, BackgroundFill::_Default(_))
    }
}

impl AsRef<BackgroundFill> for BackgroundFill {
    fn as_ref(&self) -> &BackgroundFill {
        self
    }
}

/// Describes a gradient fill of a background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundFillGradient {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// A top color of the background in the RGB24 format
    top_color: i32,
    /// A bottom color of the background in the RGB24 format
    bottom_color: i32,
    /// Clockwise rotation angle of the gradient, in degrees; 0-359. Should be always divisible by 45
    rotation_angle: i32,
}

impl RObject for BackgroundFillGradient {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "backgroundFillGradient"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDBackgroundFill for BackgroundFillGradient {}

impl BackgroundFillGradient {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBackgroundFillGradientBuilder {
        let mut inner = BackgroundFillGradient::default();
        inner.td_name = "backgroundFillGradient".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDBackgroundFillGradientBuilder { inner }
    }

    pub fn top_color(&self) -> i32 {
        self.top_color
    }

    pub fn bottom_color(&self) -> i32 {
        self.bottom_color
    }

    pub fn rotation_angle(&self) -> i32 {
        self.rotation_angle
    }
}

#[doc(hidden)]
pub struct RTDBackgroundFillGradientBuilder {
    inner: BackgroundFillGradient,
}

impl RTDBackgroundFillGradientBuilder {
    pub fn build(&self) -> BackgroundFillGradient {
        self.inner.clone()
    }

    pub fn top_color(&mut self, top_color: i32) -> &mut Self {
        self.inner.top_color = top_color;
        self
    }

    pub fn bottom_color(&mut self, bottom_color: i32) -> &mut Self {
        self.inner.bottom_color = bottom_color;
        self
    }

    pub fn rotation_angle(&mut self, rotation_angle: i32) -> &mut Self {
        self.inner.rotation_angle = rotation_angle;
        self
    }
}

impl AsRef<BackgroundFillGradient> for BackgroundFillGradient {
    fn as_ref(&self) -> &BackgroundFillGradient {
        self
    }
}

impl AsRef<BackgroundFillGradient> for RTDBackgroundFillGradientBuilder {
    fn as_ref(&self) -> &BackgroundFillGradient {
        &self.inner
    }
}

/// Describes a solid fill of a background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundFillSolid {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// A color of the background in the RGB24 format
    color: i32,
}

impl RObject for BackgroundFillSolid {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "backgroundFillSolid"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDBackgroundFill for BackgroundFillSolid {}

impl BackgroundFillSolid {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBackgroundFillSolidBuilder {
        let mut inner = BackgroundFillSolid::default();
        inner.td_name = "backgroundFillSolid".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDBackgroundFillSolidBuilder { inner }
    }

    pub fn color(&self) -> i32 {
        self.color
    }
}

#[doc(hidden)]
pub struct RTDBackgroundFillSolidBuilder {
    inner: BackgroundFillSolid,
}

impl RTDBackgroundFillSolidBuilder {
    pub fn build(&self) -> BackgroundFillSolid {
        self.inner.clone()
    }

    pub fn color(&mut self, color: i32) -> &mut Self {
        self.inner.color = color;
        self
    }
}

impl AsRef<BackgroundFillSolid> for BackgroundFillSolid {
    fn as_ref(&self) -> &BackgroundFillSolid {
        self
    }
}

impl AsRef<BackgroundFillSolid> for RTDBackgroundFillSolidBuilder {
    fn as_ref(&self) -> &BackgroundFillSolid {
        &self.inner
    }
}
