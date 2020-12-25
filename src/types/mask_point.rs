use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Part of the face, relative to which a mask should be placed
pub trait TDMaskPoint: Debug + RObject {}

/// Part of the face, relative to which a mask should be placed
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MaskPoint {
    #[doc(hidden)]
    _Default(()),
    /// A mask should be placed relatively to the chin
    Chin(MaskPointChin),
    /// A mask should be placed relatively to the eyes
    Eyes(MaskPointEyes),
    /// A mask should be placed relatively to the forehead
    Forehead(MaskPointForehead),
    /// A mask should be placed relatively to the mouth
    Mouth(MaskPointMouth),
}

impl Default for MaskPoint {
    fn default() -> Self {
        MaskPoint::_Default(())
    }
}

impl<'de> Deserialize<'de> for MaskPoint {
    fn deserialize<D>(deserializer: D) -> Result<MaskPoint, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          MaskPoint,
          (maskPointChin, Chin);
          (maskPointEyes, Eyes);
          (maskPointForehead, Forehead);
          (maskPointMouth, Mouth);

        )(deserializer)
    }
}

impl RObject for MaskPoint {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            MaskPoint::Chin(t) => t.td_name(),
            MaskPoint::Eyes(t) => t.td_name(),
            MaskPoint::Forehead(t) => t.td_name(),
            MaskPoint::Mouth(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            MaskPoint::Chin(t) => t.extra(),
            MaskPoint::Eyes(t) => t.extra(),
            MaskPoint::Forehead(t) => t.extra(),
            MaskPoint::Mouth(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl MaskPoint {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, MaskPoint::_Default(_))
    }
}

impl AsRef<MaskPoint> for MaskPoint {
    fn as_ref(&self) -> &MaskPoint {
        self
    }
}

/// A mask should be placed relatively to the chin
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MaskPointChin {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for MaskPointChin {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "maskPointChin"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDMaskPoint for MaskPointChin {}

impl MaskPointChin {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMaskPointChinBuilder {
        let mut inner = MaskPointChin::default();
        inner.td_name = "maskPointChin".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDMaskPointChinBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDMaskPointChinBuilder {
    inner: MaskPointChin,
}

impl RTDMaskPointChinBuilder {
    pub fn build(&self) -> MaskPointChin {
        self.inner.clone()
    }
}

impl AsRef<MaskPointChin> for MaskPointChin {
    fn as_ref(&self) -> &MaskPointChin {
        self
    }
}

impl AsRef<MaskPointChin> for RTDMaskPointChinBuilder {
    fn as_ref(&self) -> &MaskPointChin {
        &self.inner
    }
}

/// A mask should be placed relatively to the eyes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MaskPointEyes {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for MaskPointEyes {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "maskPointEyes"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDMaskPoint for MaskPointEyes {}

impl MaskPointEyes {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMaskPointEyesBuilder {
        let mut inner = MaskPointEyes::default();
        inner.td_name = "maskPointEyes".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDMaskPointEyesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDMaskPointEyesBuilder {
    inner: MaskPointEyes,
}

impl RTDMaskPointEyesBuilder {
    pub fn build(&self) -> MaskPointEyes {
        self.inner.clone()
    }
}

impl AsRef<MaskPointEyes> for MaskPointEyes {
    fn as_ref(&self) -> &MaskPointEyes {
        self
    }
}

impl AsRef<MaskPointEyes> for RTDMaskPointEyesBuilder {
    fn as_ref(&self) -> &MaskPointEyes {
        &self.inner
    }
}

/// A mask should be placed relatively to the forehead
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MaskPointForehead {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for MaskPointForehead {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "maskPointForehead"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDMaskPoint for MaskPointForehead {}

impl MaskPointForehead {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMaskPointForeheadBuilder {
        let mut inner = MaskPointForehead::default();
        inner.td_name = "maskPointForehead".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDMaskPointForeheadBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDMaskPointForeheadBuilder {
    inner: MaskPointForehead,
}

impl RTDMaskPointForeheadBuilder {
    pub fn build(&self) -> MaskPointForehead {
        self.inner.clone()
    }
}

impl AsRef<MaskPointForehead> for MaskPointForehead {
    fn as_ref(&self) -> &MaskPointForehead {
        self
    }
}

impl AsRef<MaskPointForehead> for RTDMaskPointForeheadBuilder {
    fn as_ref(&self) -> &MaskPointForehead {
        &self.inner
    }
}

/// A mask should be placed relatively to the mouth
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MaskPointMouth {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for MaskPointMouth {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "maskPointMouth"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDMaskPoint for MaskPointMouth {}

impl MaskPointMouth {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMaskPointMouthBuilder {
        let mut inner = MaskPointMouth::default();
        inner.td_name = "maskPointMouth".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDMaskPointMouthBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDMaskPointMouthBuilder {
    inner: MaskPointMouth,
}

impl RTDMaskPointMouthBuilder {
    pub fn build(&self) -> MaskPointMouth {
        self.inner.clone()
    }
}

impl AsRef<MaskPointMouth> for MaskPointMouth {
    fn as_ref(&self) -> &MaskPointMouth {
        self
    }
}

impl AsRef<MaskPointMouth> for RTDMaskPointMouthBuilder {
    fn as_ref(&self) -> &MaskPointMouth {
        &self.inner
    }
}
