use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the quality of a group call video
pub trait TDGroupCallVideoQuality: Debug + RObject {}

/// Describes the quality of a group call video
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallVideoQuality {
    #[doc(hidden)]
    _Default,
    /// The best available video quality
    #[serde(rename = "groupCallVideoQualityFull")]
    Full(GroupCallVideoQualityFull),
    /// The medium video quality
    #[serde(rename = "groupCallVideoQualityMedium")]
    Medium(GroupCallVideoQualityMedium),
    /// The worst available video quality
    #[serde(rename = "groupCallVideoQualityThumbnail")]
    Thumbnail(GroupCallVideoQualityThumbnail),
}

impl Default for GroupCallVideoQuality {
    fn default() -> Self {
        GroupCallVideoQuality::_Default
    }
}

impl RObject for GroupCallVideoQuality {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            GroupCallVideoQuality::Full(t) => t.extra(),
            GroupCallVideoQuality::Medium(t) => t.extra(),
            GroupCallVideoQuality::Thumbnail(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            GroupCallVideoQuality::Full(t) => t.client_id(),
            GroupCallVideoQuality::Medium(t) => t.client_id(),
            GroupCallVideoQuality::Thumbnail(t) => t.client_id(),

            _ => None,
        }
    }
}

impl GroupCallVideoQuality {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, GroupCallVideoQuality::_Default)
    }
}

impl AsRef<GroupCallVideoQuality> for GroupCallVideoQuality {
    fn as_ref(&self) -> &GroupCallVideoQuality {
        self
    }
}

/// The best available video quality
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallVideoQualityFull {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for GroupCallVideoQualityFull {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDGroupCallVideoQuality for GroupCallVideoQualityFull {}

impl GroupCallVideoQualityFull {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GroupCallVideoQualityFullBuilder {
        let mut inner = GroupCallVideoQualityFull::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        GroupCallVideoQualityFullBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GroupCallVideoQualityFullBuilder {
    inner: GroupCallVideoQualityFull,
}

#[deprecated]
pub type RTDGroupCallVideoQualityFullBuilder = GroupCallVideoQualityFullBuilder;

impl GroupCallVideoQualityFullBuilder {
    pub fn build(&self) -> GroupCallVideoQualityFull {
        self.inner.clone()
    }
}

impl AsRef<GroupCallVideoQualityFull> for GroupCallVideoQualityFull {
    fn as_ref(&self) -> &GroupCallVideoQualityFull {
        self
    }
}

impl AsRef<GroupCallVideoQualityFull> for GroupCallVideoQualityFullBuilder {
    fn as_ref(&self) -> &GroupCallVideoQualityFull {
        &self.inner
    }
}

/// The medium video quality
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallVideoQualityMedium {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for GroupCallVideoQualityMedium {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDGroupCallVideoQuality for GroupCallVideoQualityMedium {}

impl GroupCallVideoQualityMedium {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GroupCallVideoQualityMediumBuilder {
        let mut inner = GroupCallVideoQualityMedium::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        GroupCallVideoQualityMediumBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GroupCallVideoQualityMediumBuilder {
    inner: GroupCallVideoQualityMedium,
}

#[deprecated]
pub type RTDGroupCallVideoQualityMediumBuilder = GroupCallVideoQualityMediumBuilder;

impl GroupCallVideoQualityMediumBuilder {
    pub fn build(&self) -> GroupCallVideoQualityMedium {
        self.inner.clone()
    }
}

impl AsRef<GroupCallVideoQualityMedium> for GroupCallVideoQualityMedium {
    fn as_ref(&self) -> &GroupCallVideoQualityMedium {
        self
    }
}

impl AsRef<GroupCallVideoQualityMedium> for GroupCallVideoQualityMediumBuilder {
    fn as_ref(&self) -> &GroupCallVideoQualityMedium {
        &self.inner
    }
}

/// The worst available video quality
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallVideoQualityThumbnail {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for GroupCallVideoQualityThumbnail {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDGroupCallVideoQuality for GroupCallVideoQualityThumbnail {}

impl GroupCallVideoQualityThumbnail {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GroupCallVideoQualityThumbnailBuilder {
        let mut inner = GroupCallVideoQualityThumbnail::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        GroupCallVideoQualityThumbnailBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GroupCallVideoQualityThumbnailBuilder {
    inner: GroupCallVideoQualityThumbnail,
}

#[deprecated]
pub type RTDGroupCallVideoQualityThumbnailBuilder = GroupCallVideoQualityThumbnailBuilder;

impl GroupCallVideoQualityThumbnailBuilder {
    pub fn build(&self) -> GroupCallVideoQualityThumbnail {
        self.inner.clone()
    }
}

impl AsRef<GroupCallVideoQualityThumbnail> for GroupCallVideoQualityThumbnail {
    fn as_ref(&self) -> &GroupCallVideoQualityThumbnail {
        self
    }
}

impl AsRef<GroupCallVideoQualityThumbnail> for GroupCallVideoQualityThumbnailBuilder {
    fn as_ref(&self) -> &GroupCallVideoQualityThumbnail {
        &self.inner
    }
}
